/// Unified tray interface.
/// Linux: ksni (StatusNotifierItem, works natively on KDE/GNOME/XFCE)
/// Windows: tray-icon
use std::sync::{Arc, Mutex};

const ICON_OFF: &[u8] = include_bytes!("../resources/guard-off.png");
const ICON_ON: &[u8] = include_bytes!("../resources/guard-on.png");

pub enum TrayEvent {
    ShowWindow,
    ToggleArm,
    ToggleTestMode,
    ToggleShutdownOnClose,
}

pub struct TrayState {
    pub armed: bool,
    pub has_key_device: bool,
    pub test_mode: bool,
    pub shutdown_on_close: bool,
}

pub struct AppTray {
    state: Arc<Mutex<TrayState>>,
    event_rx: std::sync::mpsc::Receiver<TrayEvent>,
    #[cfg(target_os = "linux")]
    ksni_handle: ksni::blocking::Handle<KsniTray>,
    #[cfg(target_os = "windows")]
    _inner: tray_icon::TrayIcon,
}

impl AppTray {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<TrayEvent>();
        let state = Arc::new(Mutex::new(TrayState {
            armed: false,
            has_key_device: false,
            test_mode: false,
            shutdown_on_close: false,
        }));

        #[cfg(target_os = "linux")]
        let ksni_handle = spawn_ksni(state.clone(), tx);

        #[cfg(target_os = "windows")]
        let _inner = spawn_windows_tray(state.clone(), tx);

        Self {
            state,
            event_rx: rx,
            #[cfg(target_os = "linux")]
            ksni_handle,
            #[cfg(target_os = "windows")]
            _inner,
        }
    }

    pub fn set_state(
        &self,
        armed: bool,
        has_key_device: bool,
        test_mode: bool,
        shutdown_on_close: bool,
    ) {
        let mut s = self.state.lock().unwrap();
        s.armed = armed;
        s.has_key_device = has_key_device;
        s.test_mode = test_mode;
        s.shutdown_on_close = shutdown_on_close;
        drop(s);
        #[cfg(target_os = "linux")]
        {
            let _ = self.ksni_handle.update(|_| {});
        }
    }

    pub fn poll(&self) -> Option<TrayEvent> {
        self.event_rx.try_recv().ok()
    }
}

// ── Linux (ksni + async-io) ───────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn spawn_ksni(
    state: Arc<Mutex<TrayState>>,
    tx: std::sync::mpsc::Sender<TrayEvent>,
) -> ksni::blocking::Handle<KsniTray> {
    use ksni::blocking::TrayMethods;
    let tray = KsniTray {
        state,
        tx,
        icon_off: decode_ksni_icon(ICON_OFF),
        icon_on: decode_ksni_icon(ICON_ON),
    };
    // Blocking spawn: sets up D-Bus session on this thread (via async-io block_on),
    // then moves the service event loop to its own background std::thread.
    tray.spawn().expect("ksni spawn")
}

#[cfg(target_os = "linux")]
struct KsniTray {
    state: Arc<Mutex<TrayState>>,
    tx: std::sync::mpsc::Sender<TrayEvent>,
    icon_off: ksni::Icon,
    icon_on: ksni::Icon,
}

#[cfg(target_os = "linux")]
impl ksni::Tray for KsniTray {
    fn id(&self) -> String {
        "xxusbsentinel".into()
    }
    fn title(&self) -> String {
        "xxUSBSentinel".into()
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        let armed = self.state.lock().unwrap().armed;
        vec![if armed {
            self.icon_on.clone()
        } else {
            self.icon_off.clone()
        }]
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        let _ = self.tx.send(TrayEvent::ShowWindow);
    }

    fn menu(&self) -> Vec<ksni::menu::MenuItem<Self>> {
        use ksni::menu::*;
        let (armed, has_key, test_mode, shutdown_on_close) = {
            let s = self.state.lock().unwrap();
            (s.armed, s.has_key_device, s.test_mode, s.shutdown_on_close)
        };
        let arm_label = if armed { "Disarm" } else { "Arm Sentinel" };
        let test_label = if test_mode {
            "✓  Test mode"
        } else {
            "    Test mode"
        };
        let soc_label = if shutdown_on_close {
            "✓  Shutdown on close"
        } else {
            "    Shutdown on close"
        };
        vec![
            StandardItem {
                label: "Show xxUSBSentinel".into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.tx.send(TrayEvent::ShowWindow);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: arm_label.into(),
                enabled: has_key,
                activate: Box::new(|t: &mut Self| {
                    let _ = t.tx.send(TrayEvent::ToggleArm);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: test_label.into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.tx.send(TrayEvent::ToggleTestMode);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: soc_label.into(),
                activate: Box::new(|t: &mut Self| {
                    let _ = t.tx.send(TrayEvent::ToggleShutdownOnClose);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(|_| std::process::exit(0)),
                ..Default::default()
            }
            .into(),
        ]
    }
}

#[cfg(target_os = "linux")]
fn decode_ksni_icon(bytes: &[u8]) -> ksni::Icon {
    let img = image::load_from_memory(bytes).expect("icon").into_rgba8();
    let (width, height) = img.dimensions();
    // SNI protocol wants ARGB32 in network byte order
    let data: Vec<u8> = img
        .pixels()
        .flat_map(|p| [p[3], p[0], p[1], p[2]])
        .collect();
    ksni::Icon {
        width: width as i32,
        height: height as i32,
        data,
    }
}

// ── Windows (tray-icon) ───────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn spawn_windows_tray(
    _state: Arc<Mutex<TrayState>>,
    _tx: std::sync::mpsc::Sender<TrayEvent>,
) -> tray_icon::TrayIcon {
    let icon = decode_tray_icon(ICON_OFF);
    tray_icon::TrayIconBuilder::new()
        .with_tooltip("xxUSBSentinel — disarmed")
        .with_icon(icon)
        .build()
        .expect("tray icon")
}

#[cfg(target_os = "windows")]
fn decode_tray_icon(bytes: &[u8]) -> tray_icon::Icon {
    let img = image::load_from_memory(bytes).expect("icon").into_rgba8();
    let (w, h) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), w, h).expect("icon")
}
