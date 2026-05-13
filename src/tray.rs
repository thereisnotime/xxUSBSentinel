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
    /// The live tray icon handle.  On Windows we must call set_icon() /
    /// set_tooltip() explicitly when the armed state changes.
    #[cfg(target_os = "windows")]
    inner: tray_icon::TrayIcon,
    /// Pre-decoded icons kept to avoid re-decoding PNG on every arm/disarm.
    #[cfg(target_os = "windows")]
    icon_off: tray_icon::Icon,
    #[cfg(target_os = "windows")]
    icon_on: tray_icon::Icon,
    /// Menu item handles held so we can update their label text in set_state().
    /// MenuItem uses Rc internally and is therefore !Send — it must only be
    /// touched from the GUI (main) thread, which is where set_state() is called.
    #[cfg(target_os = "windows")]
    menu_item_arm: tray_icon::menu::MenuItem,
    #[cfg(target_os = "windows")]
    menu_item_test: tray_icon::menu::MenuItem,
    #[cfg(target_os = "windows")]
    menu_item_soc: tray_icon::menu::MenuItem,
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
        let (inner, icon_off, icon_on, menu_item_arm, menu_item_test, menu_item_soc) =
            spawn_windows_tray(state.clone(), tx);

        Self {
            state,
            event_rx: rx,
            #[cfg(target_os = "linux")]
            ksni_handle,
            #[cfg(target_os = "windows")]
            inner,
            #[cfg(target_os = "windows")]
            icon_off,
            #[cfg(target_os = "windows")]
            icon_on,
            #[cfg(target_os = "windows")]
            menu_item_arm,
            #[cfg(target_os = "windows")]
            menu_item_test,
            #[cfg(target_os = "windows")]
            menu_item_soc,
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

        // On Windows the tray-icon crate does not re-render automatically.
        // Push the new icon and tooltip, and update the menu label text.
        #[cfg(target_os = "windows")]
        {
            let icon = if armed {
                self.icon_on.clone()
            } else {
                self.icon_off.clone()
            };
            let tooltip = if armed {
                "xxUSBSentinel — ARMED"
            } else {
                "xxUSBSentinel — disarmed"
            };
            let _ = self.inner.set_icon(Some(icon));
            let _ = self.inner.set_tooltip(Some(tooltip));

            self.menu_item_arm
                .set_text(if armed { "Disarm" } else { "Arm Sentinel" });
            self.menu_item_test.set_text(if test_mode {
                "✓  Test mode"
            } else {
                "    Test mode"
            });
            self.menu_item_soc.set_text(if shutdown_on_close {
                "✓  Shutdown on close"
            } else {
                "    Shutdown on close"
            });
        }
    }

    pub fn poll(&self) -> Option<TrayEvent> {
        // On Windows we must also drain the tray-icon event channels so that
        // menu clicks and double-click events are delivered to the caller.
        #[cfg(target_os = "windows")]
        {
            use tray_icon::{menu::MenuEvent, TrayIconEvent};

            // Check for tray icon events (left-click, double-click, …).
            if let Ok(ev) = TrayIconEvent::receiver().try_recv() {
                if let TrayIconEvent::DoubleClick { .. } = ev {
                    return Some(TrayEvent::ShowWindow);
                }
            }

            // Check for menu item clicks.
            if let Ok(menu_ev) = MenuEvent::receiver().try_recv() {
                match menu_ev.id().0.as_str() {
                    "show" => return Some(TrayEvent::ShowWindow),
                    "arm" => return Some(TrayEvent::ToggleArm),
                    "test_mode" => return Some(TrayEvent::ToggleTestMode),
                    "soc" => return Some(TrayEvent::ToggleShutdownOnClose),
                    "exit" => std::process::exit(0),
                    _ => {}
                }
            }
        }

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

/// Returns `(TrayIcon, icon_off, icon_on, item_arm, item_test, item_soc)`.
///
/// The `TrayIcon` must be kept alive for the duration of the process (dropping
/// it removes the tray icon).  The three `MenuItem` handles are kept so
/// `set_state()` can update their labels on the GUI thread.  MenuItem uses
/// `Rc` internally and is therefore `!Send`; all access must stay on the main
/// (GUI) thread.
#[cfg(target_os = "windows")]
#[allow(clippy::type_complexity)]
fn spawn_windows_tray(
    _state: Arc<Mutex<TrayState>>,
    _tx: std::sync::mpsc::Sender<TrayEvent>,
) -> (
    tray_icon::TrayIcon,
    tray_icon::Icon,
    tray_icon::Icon,
    tray_icon::menu::MenuItem,
    tray_icon::menu::MenuItem,
    tray_icon::menu::MenuItem,
) {
    use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};

    let icon_off = decode_tray_icon(ICON_OFF);
    let icon_on = decode_tray_icon(ICON_ON);

    // Build the context menu.  Each item carries a stable string ID that
    // poll() matches against MenuEvent::id().
    let menu = Menu::new();
    let item_show = MenuItem::with_id("show", "Show xxUSBSentinel", true, None);
    let sep1 = PredefinedMenuItem::separator();
    // Initial labels reflect the disarmed / unchecked state.
    let item_arm = MenuItem::with_id("arm", "Arm Sentinel", true, None);
    let item_test = MenuItem::with_id("test_mode", "    Test mode", true, None);
    let item_soc = MenuItem::with_id("soc", "    Shutdown on close", true, None);
    let sep2 = PredefinedMenuItem::separator();
    let item_exit = MenuItem::with_id("exit", "Exit", true, None);
    let _ = menu.append_items(&[
        &item_show, &sep1, &item_arm, &item_test, &item_soc, &sep2, &item_exit,
    ]);

    let tray = tray_icon::TrayIconBuilder::new()
        .with_tooltip("xxUSBSentinel — disarmed")
        .with_icon(icon_off.clone())
        .with_menu(Box::new(menu))
        .build()
        .expect("tray icon");

    (tray, icon_off, icon_on, item_arm, item_test, item_soc)
}

#[cfg(target_os = "windows")]
fn decode_tray_icon(bytes: &[u8]) -> tray_icon::Icon {
    let img = image::load_from_memory(bytes).expect("icon").into_rgba8();
    let (w, h) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), w, h).expect("icon")
}
