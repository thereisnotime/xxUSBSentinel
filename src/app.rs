use std::sync::{mpsc, Arc, Mutex};

use eframe::egui::{self, Color32, RichText, Stroke};

use crate::config::{autostart_enabled, set_autostart, Config, Hook};
use crate::sentinel::{GuiEvent, LogEntry, SharedState, UsbDevice};
use crate::tray::{AppTray, TrayEvent};

const GREEN: Color32 = Color32::from_rgb(80, 200, 100);
const RED: Color32 = Color32::from_rgb(220, 60, 60);
const YELLOW: Color32 = Color32::from_rgb(230, 190, 50);
const DIM: Color32 = Color32::from_rgb(130, 130, 130);

pub struct SentinelApp {
    state: Arc<Mutex<SharedState>>,
    rx: mpsc::Receiver<GuiEvent>,
    log: Vec<LogEntry>,
    devices: Vec<UsbDevice>,
    tray: AppTray,
    cfg: Config,
    autostart: bool,
    shutdown_ok: bool,
    prev_armed: bool,
    prev_has_key: bool,
    prev_test_mode: bool,
    prev_soc: bool,
    show_advanced: bool,
    show_help: bool,
    show_about: bool,
    hooks_buf: Vec<Hook>,
    // Fake BSOD overlay state
    bsod_active: bool,
    bsod_style: String,
    bsod_frames: u8, // counts rendered frames before firing shutdown
    bsod_wipe_swap: bool,
    bsod_wipe_hiberfil: bool,
    bsod_preview_until: Option<std::time::Instant>,
}

impl SentinelApp {
    pub fn new(
        cc: &eframe::CreationContext,
        state: Arc<Mutex<SharedState>>,
        rx: mpsc::Receiver<GuiEvent>,
        cfg: Config,
    ) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(dark_visuals());

        let autostart = autostart_enabled();
        let shutdown_ok = crate::shutdown::can_shutdown();
        Self {
            state,
            rx,
            log: Vec::new(),
            devices: Vec::new(),
            tray: AppTray::new(),
            hooks_buf: cfg.hooks.clone(),
            cfg,
            autostart,
            shutdown_ok,
            prev_armed: false,
            prev_has_key: false,
            prev_test_mode: false,
            prev_soc: false,
            show_advanced: false,
            show_help: false,
            show_about: false,
            bsod_active: false,
            bsod_style: String::new(),
            bsod_frames: 0,
            bsod_wipe_swap: false,
            bsod_wipe_hiberfil: false,
            bsod_preview_until: None,
        }
    }
}

impl eframe::App for SentinelApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain USB events
        while let Ok(event) = self.rx.try_recv() {
            match event {
                GuiEvent::InitialDevices(list) => self.devices = list,
                GuiEvent::DeviceConnected(d) => {
                    if !self.devices.iter().any(|x| x.vid_pid == d.vid_pid) {
                        let comment = self
                            .cfg
                            .device_comments
                            .get(&d.vid_pid)
                            .cloned()
                            .unwrap_or_default();
                        let label = display_label(&d.vid_pid, &d.name, &comment);
                        push_log(&mut self.log, &format!("Connected: {}", label));
                        run_hooks(&self.cfg.hooks, &d.vid_pid, &d.name, "connected");
                        self.devices.push(d);
                    }
                }
                GuiEvent::DeviceDisconnected(vp) => {
                    let name = self
                        .devices
                        .iter()
                        .find(|d| d.vid_pid == vp)
                        .map(|d| d.name.clone())
                        .unwrap_or_default();
                    let comment = self
                        .cfg
                        .device_comments
                        .get(&vp)
                        .cloned()
                        .unwrap_or_default();
                    let label = display_label(&vp, &name, &comment);
                    push_log(&mut self.log, &format!("Disconnected: {}", label));
                    run_hooks(&self.cfg.hooks, &vp, &name, "disconnected");
                    self.devices.retain(|d| d.vid_pid != vp);
                }
                GuiEvent::DeviceMapped(vp) => {
                    self.cfg.key_device = vp.clone();
                    self.cfg.save();
                    let name = self
                        .devices
                        .iter()
                        .find(|d| d.vid_pid == vp)
                        .map(|d| d.name.clone())
                        .unwrap_or_default();
                    let comment = self
                        .cfg
                        .device_comments
                        .get(&vp)
                        .cloned()
                        .unwrap_or_default();
                    let label = display_label(&vp, &name, &comment);
                    push_log(&mut self.log, &format!("Key device mapped: {}", label));
                }
                GuiEvent::TestTriggered => {
                    push_log(
                        &mut self.log,
                        "Test triggered — key device removed (shutdown suppressed).",
                    );
                    let name = self
                        .devices
                        .iter()
                        .find(|d| d.vid_pid == self.cfg.key_device)
                        .map(|d| d.name.clone())
                        .unwrap_or_default();
                    run_hooks(&self.cfg.hooks, &self.cfg.key_device, &name, "triggered");
                }
                GuiEvent::ShutdownTriggered {
                    wipe_swap,
                    wipe_hiberfil,
                    fake_bsod,
                    bsod_style,
                } => {
                    let name = self
                        .devices
                        .iter()
                        .find(|d| d.vid_pid == self.cfg.key_device)
                        .map(|d| d.name.clone())
                        .unwrap_or_default();
                    run_hooks(&self.cfg.hooks, &self.cfg.key_device, &name, "triggered");
                    if fake_bsod {
                        self.bsod_active = true;
                        self.bsod_style = bsod_style;
                        self.bsod_wipe_swap = wipe_swap;
                        self.bsod_wipe_hiberfil = wipe_hiberfil;
                        self.bsod_frames = 0;
                    } else {
                        if wipe_swap {
                            crate::wipe::wipe_swap();
                        }
                        if wipe_hiberfil {
                            crate::wipe::wipe_hiberfil();
                        }
                        crate::shutdown::execute();
                    }
                }
            }
        }

        // Sync tray state whenever any relevant field changes
        let (armed, has_key, test_mode_s, soc_s) = {
            let s = self.state.lock().unwrap();
            (
                s.armed,
                !s.key_device.is_empty(),
                s.test_mode,
                s.shutdown_on_close,
            )
        };
        if armed != self.prev_armed
            || has_key != self.prev_has_key
            || test_mode_s != self.prev_test_mode
            || soc_s != self.prev_soc
        {
            self.prev_armed = armed;
            self.prev_has_key = has_key;
            self.prev_test_mode = test_mode_s;
            self.prev_soc = soc_s;
            self.tray.set_state(armed, has_key, test_mode_s, soc_s);
        }

        // Handle tray events
        while let Some(ev) = self.tray.poll() {
            match ev {
                TrayEvent::ShowWindow => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                        egui::UserAttentionType::Informational,
                    ));
                }
                TrayEvent::ToggleArm => {
                    let mut s = self.state.lock().unwrap();
                    if !s.key_device.is_empty() {
                        s.armed = !s.armed;
                        let msg = if s.armed {
                            "Sentinel ARMED (via tray)."
                        } else {
                            "Sentinel DISARMED (via tray)."
                        };
                        drop(s);
                        push_log(&mut self.log, msg);
                    }
                }
                TrayEvent::ToggleTestMode => {
                    let new_val = {
                        let mut s = self.state.lock().unwrap();
                        s.test_mode = !s.test_mode;
                        s.test_mode
                    };
                    self.cfg.test_mode = new_val;
                    self.cfg.save();
                    push_log(
                        &mut self.log,
                        if new_val {
                            "Test mode ON (via tray)."
                        } else {
                            "Test mode OFF (via tray)."
                        },
                    );
                }
                TrayEvent::ToggleShutdownOnClose => {
                    let new_val = {
                        let mut s = self.state.lock().unwrap();
                        s.shutdown_on_close = !s.shutdown_on_close;
                        s.shutdown_on_close
                    };
                    self.cfg.shutdown_on_close = new_val;
                    self.cfg.save();
                    push_log(
                        &mut self.log,
                        if new_val {
                            "Shutdown on close ON (via tray)."
                        } else {
                            "Shutdown on close OFF (via tray)."
                        },
                    );
                }
            }
        }

        // Expire preview
        if let Some(until) = self.bsod_preview_until {
            if std::time::Instant::now() >= until {
                self.bsod_preview_until = None;
            }
        }

        // Fire shutdown after BSOD has rendered a couple of frames
        if self.bsod_active && self.bsod_preview_until.is_none() {
            self.bsod_frames = self.bsod_frames.saturating_add(1);
            if self.bsod_frames >= 3 {
                if self.bsod_wipe_swap {
                    crate::wipe::wipe_swap();
                }
                if self.bsod_wipe_hiberfil {
                    crate::wipe::wipe_hiberfil();
                }
                crate::shutdown::execute();
            }
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // ── Fake BSOD overlay (real trigger or preview) ───────────────────
        if self.bsod_active || self.bsod_preview_until.is_some() {
            let style = if self.bsod_active {
                self.bsod_style.as_str()
            } else {
                self.cfg.bsod_style.as_str()
            };
            let img_bytes: Option<(&[u8], &str)> = match style {
                "win10" => Some((
                    include_bytes!("../resources/bsod-win10.png"),
                    "bytes://bsod-win10",
                )),
                "win11" => Some((
                    include_bytes!("../resources/bsod-win11.png"),
                    "bytes://bsod-win11",
                )),
                "linux" => Some((
                    include_bytes!("../resources/bsod-linux.png"),
                    "bytes://bsod-linux",
                )),
                _ => None, // "blank" — solid black
            };
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("bsod_overlay"),
                egui::ViewportBuilder::default()
                    .with_fullscreen(true)
                    .with_decorations(false)
                    .with_always_on_top()
                    .with_title(""),
                |ctx, _| {
                    bsod_panel(ctx, img_bytes);
                },
            );
            return;
        }

        // Close button: either minimise to tray, or shut down if armed and shutdown_on_close
        if ctx.input(|i| i.viewport().close_requested()) {
            let s = self.state.lock().unwrap();
            let should_shutdown = s.armed && s.shutdown_on_close;
            let is_test = s.test_mode;
            drop(s);
            if should_shutdown {
                if is_test {
                    push_log(
                        &mut self.log,
                        "Test triggered — window closed while armed (shutdown suppressed).",
                    );
                } else {
                    crate::shutdown::execute();
                }
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        let (armed, test_mode, waiting, key_device, shutdown_on_close) = {
            let s = self.state.lock().unwrap();
            (
                s.armed,
                s.test_mode,
                s.waiting,
                s.key_device.clone(),
                s.shutdown_on_close,
            )
        };

        // ── Popups ────────────────────────────────────────────────────────

        if self.show_help {
            egui::Window::new("Help")
                .collapsible(false).resizable(true)
                .min_width(320.0).max_width(520.0).min_height(200.0)
                .show(&ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.strong("Quick start");
                    ui.add_space(4.0);
                    ui.label("1. Plug in the USB device you want to use as your kill-switch key.");
                    ui.label("2. Click Set Key next to it in the device list.");
                    ui.label("3. Click Arm Sentinel. The status turns green.");
                    ui.label("4. If the key device is removed while armed, the PC shuts down immediately.");
                    ui.add_space(10.0);

                    ui.strong("Map Device");
                    ui.add_space(4.0);
                    ui.label("An alternative way to pick a key: click Map Device, then physically unplug the USB device you want to map. The app detects the removal and sets it as the key automatically.");
                    ui.add_space(10.0);

                    ui.strong("Test mode");
                    ui.add_space(4.0);
                    ui.label("Enable the Test mode checkbox before arming. When the key device is removed, a popup appears instead of triggering a real shutdown. Use this to verify your setup without risk.");
                    ui.add_space(10.0);

                    ui.strong("Shutdown on close");
                    ui.add_space(4.0);
                    ui.label("Enable this checkbox to treat closing the window as a trigger while armed. Normally, closing minimises to tray. With this on, closing the window while armed shuts down the PC (or shows the test popup in Test mode).");
                    ui.add_space(10.0);

                    ui.strong("Tray icon");
                    ui.add_space(4.0);
                    ui.label("The app always stays in the system tray. Left-click or use the tray menu to show the window. The tray menu also lets you Arm/Disarm without opening the window.");
                    ui.add_space(10.0);

                    ui.colored_label(YELLOW, "This app does not encrypt your data.");
                    ui.colored_label(YELLOW, "Use VeraCrypt and/or LUKS for full disk encryption.");
                    ui.add_space(10.0);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Close  ").clicked() { self.show_help = false; }
                    });
                }); // ScrollArea
                });
        }

        if self.show_about {
            egui::Window::new("About")
                .collapsible(false).resizable(false)
                .show(&ctx, |ui| {
                    ui.heading(format!("xxUSBSentinel  v{}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(4.0);
                    ui.label("USB kill-switch — shuts down the PC when the mapped key device is removed.");
                    ui.add_space(4.0);
                    ui.label("Rust rewrite, cross-platform (Linux & Windows).");
                    ui.label("https://github.com/thereisnotime/xxUSBSentinel");
                    ui.add_space(10.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Close  ").clicked() { self.show_about = false; }
                    });
                });
        }

        if self.show_advanced {
            egui::Window::new("Advanced Settings")
                .collapsible(false).resizable(true)
                .min_width(400.0).min_height(200.0)
                .show(&ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {

                    // ── General ───────────────────────────────────────────
                    ui.strong("General");
                    ui.add_space(4.0);
                    let mut soc = shutdown_on_close;
                    if ui.checkbox(&mut soc, "Shutdown on close")
                        .on_hover_text("When armed, closing the window triggers shutdown instead of minimising to tray. Respects Test mode.")
                        .changed()
                    {
                        self.state.lock().unwrap().shutdown_on_close = soc;
                        self.cfg.shutdown_on_close = soc;
                        self.cfg.save();
                    }
                    let mut ast = self.autostart;
                    if ui.checkbox(&mut ast, "Autostart on login")
                        .on_hover_text("Launch xxUSBSentinel automatically when you log in")
                        .changed()
                    {
                        set_autostart(ast);
                        self.autostart = ast;
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // ── On-trigger actions ────────────────────────────────
                    ui.strong("Actions on trigger (before shutdown)");
                    ui.add_space(4.0);
                    ui.label(RichText::new("These run when the key device is removed while armed. Skipped in test mode.").color(DIM).small());
                    ui.add_space(6.0);

                    let mut ws = self.cfg.wipe_swap;
                    if ui.checkbox(&mut ws, "Disable swap / pagefile")
                        .on_hover_text(
                            "Linux: runs swapoff -a (flushes all swap back to RAM instantly).\n\
                             Windows: sets ClearPageFileAtShutdown so the pagefile is zeroed during shutdown."
                        ).changed()
                    {
                        self.cfg.wipe_swap = ws;
                        self.state.lock().unwrap().wipe_swap = ws;
                        self.cfg.save();
                    }

                    let mut wh = self.cfg.wipe_hiberfil;
                    if ui.checkbox(&mut wh, "Remove hibernation file")
                        .on_hover_text(
                            "Linux: sets /sys/power/image_size to 0 and masks hibernate.target.\n\
                             Windows: runs powercfg /h off which deletes hiberfil.sys immediately."
                        ).changed()
                    {
                        self.cfg.wipe_hiberfil = wh;
                        self.state.lock().unwrap().wipe_hiberfil = wh;
                        self.cfg.save();
                    }

                    ui.horizontal(|ui| {
                        let mut fb = self.cfg.fake_bsod;
                        if ui.checkbox(&mut fb, "Show fake crash screen")
                            .on_hover_text("Displays a fullscreen BSOD / kernel panic screenshot for a few seconds before shutting down.")
                            .changed()
                        {
                            self.cfg.fake_bsod = fb;
                            self.state.lock().unwrap().fake_bsod = fb;
                            self.cfg.save();
                        }
                        if ui.small_button("Preview")
                            .on_hover_text("Show the selected crash screen for 2 seconds — no shutdown")
                            .clicked()
                        {
                            self.bsod_preview_until = Some(
                                std::time::Instant::now() + std::time::Duration::from_secs(2)
                            );
                        }
                    });

                    if self.cfg.fake_bsod {
                        ui.indent("bsod_opts", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Style:");
                                let styles = [("win10", "Windows 10"), ("win11", "Windows 11"), ("linux", "Linux kernel panic"), ("blank", "Blank screen")];
                                let current_label = styles.iter()
                                    .find(|(v, _)| *v == self.cfg.bsod_style)
                                    .map(|(_, l)| *l)
                                    .unwrap_or("Unknown");
                                egui::ComboBox::from_id_salt("bsod_style_combo")
                                    .selected_text(current_label)
                                    .show_ui(ui, |ui| {
                                        for (val, label) in styles {
                                            if ui.selectable_label(self.cfg.bsod_style == val, label).clicked() {
                                                self.cfg.bsod_style = val.into();
                                                self.state.lock().unwrap().bsod_style = val.into();
                                                self.cfg.save();
                                            }
                                        }
                                    });
                            });
                        });
                    }

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // ── Script hooks ──────────────────────────────────────
                    ui.strong("Script hooks");
                    ui.add_space(4.0);
                    ui.label("Add a rule to run a script or binary when a USB event fires.");
                    ui.label(RichText::new(
                        "Script receives three arguments:  VID:PID   device-name   event-type"
                    ).color(DIM).small());
                    ui.label(RichText::new(
                        "Example:  /usr/local/bin/alert.sh 1532:0A24 \"Razer Keyboard\" connected"
                    ).color(DIM).small().italics());
                    ui.add_space(8.0);

                    // Build the device list for the dropdowns: "Any device (*)" + known devices
                    let device_options: Vec<(String, String)> = {
                        let mut v = vec![("*".into(), "Any device (*)".into())];
                        v.extend(self.devices.iter().map(|d| {
                            let label = if d.name.is_empty() {
                                d.vid_pid.clone()
                            } else {
                                format!("{} — {}", d.vid_pid, d.name)
                            };
                            (d.vid_pid.clone(), label)
                        }));
                        v
                    };
                    let event_options = [
                        ("connected",    "connects (plugged in)"),
                        ("disconnected", "disconnects (unplugged)"),
                        ("triggered",    "triggers shutdown (key device removed)"),
                    ];

                    // Header row
                    egui::Grid::new("hooks_header").num_columns(6).spacing([8.0, 4.0]).show(ui, |ui| {
                        ui.label(RichText::new("On").color(DIM).small().strong());
                        ui.label(RichText::new("Device").color(DIM).small().strong());
                        ui.label(RichText::new("Event").color(DIM).small().strong());
                        ui.label(RichText::new("Script / binary to run").color(DIM).small().strong());
                        ui.label("");
                        ui.label("");
                        ui.end_row();
                    });

                    let mut to_delete: Option<usize> = None;
                    egui::ScrollArea::vertical().max_height(220.0).id_salt("hooks_scroll").show(ui, |ui| {
                        egui::Grid::new("hooks_grid").num_columns(6).spacing([8.0, 6.0]).show(ui, |ui| {
                            for (i, hook) in self.hooks_buf.iter_mut().enumerate() {
                                let dim = !hook.enabled;

                                // Enable/disable checkbox
                                ui.checkbox(&mut hook.enabled, "");

                                // Device dropdown — locked for "triggered" (always key device)
                                if hook.event == "triggered" {
                                    ui.add_enabled(false,
                                        egui::Button::new(
                                            RichText::new("Key device (always)")
                                                .color(if dim { DIM } else { Color32::from_rgb(200,200,210) })
                                        ).min_size(egui::vec2(160.0, 0.0))
                                    ).on_disabled_hover_text(
                                        "The shutdown trigger only fires for the mapped key device — no other device can trigger it"
                                    );
                                } else {
                                    ui.add_enabled_ui(!dim, |ui| {
                                        egui::ComboBox::from_id_salt(egui::Id::new("hook_dev").with(i))
                                            .selected_text(
                                                device_options.iter()
                                                    .find(|(v, _)| v == &hook.device)
                                                    .map(|(_, l)| l.as_str())
                                                    .unwrap_or(&hook.device)
                                            )
                                            .width(160.0)
                                            .show_ui(ui, |ui| {
                                                for (val, label) in &device_options {
                                                    ui.selectable_value(&mut hook.device, val.clone(), label);
                                                }
                                            });
                                    });
                                }

                                // Event dropdown
                                egui::ComboBox::from_id_salt(egui::Id::new("hook_ev").with(i))
                                    .selected_text(
                                        event_options.iter()
                                            .find(|(v, _)| *v == hook.event)
                                            .map(|(_, l)| *l)
                                            .unwrap_or(&hook.event)
                                    )
                                    .width(210.0)
                                    .show_ui(ui, |ui| {
                                        for (val, label) in &event_options {
                                            ui.selectable_value(&mut hook.event, val.to_string(), *label);
                                        }
                                    });

                                // Script path
                                ui.add_enabled(!dim,
                                    egui::TextEdit::singleline(&mut hook.script)
                                        .desired_width(200.0)
                                        .hint_text("path to script or binary…")
                                );

                                // Browse
                                if ui.add_enabled(!dim, egui::Button::new("Browse").small()).clicked() {
                                    if let Some(p) = rfd::FileDialog::new()
                                        .set_title("Select script or binary").pick_file()
                                    {
                                        hook.script = p.to_string_lossy().into_owned();
                                    }
                                }

                                // Delete row
                                if ui.small_button(RichText::new("✕").color(RED))
                                    .on_hover_text("Remove this rule")
                                    .clicked()
                                {
                                    to_delete = Some(i);
                                }

                                ui.end_row();
                            }
                        });
                    });

                    if let Some(i) = to_delete { self.hooks_buf.remove(i); }

                    ui.add_space(4.0);
                    if ui.button("+ Add rule").clicked() {
                        self.hooks_buf.push(Hook::default());
                    }

                    ui.add_space(10.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Cancel  ").clicked() {
                            self.hooks_buf = self.cfg.hooks.clone();
                            self.show_advanced = false;
                        }
                        if ui.button("  Save  ").clicked() {
                            // Drop rules with empty script path
                            self.hooks_buf.retain(|h| !h.script.trim().is_empty());
                            self.cfg.hooks = self.hooks_buf.clone();
                            self.cfg.save();
                            self.show_advanced = false;
                            push_log(&mut self.log, "Advanced settings saved.");
                        }
                    });
                }); // ScrollArea
                });
        }

        // ── Permissions warning ───────────────────────────────────────────

        if !self.shutdown_ok {
            egui::Panel::top("perms_warn")
                .exact_size(22.0)
                .show_inside(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new(
                            "⚠  Insufficient permissions to shut down — run as root or fix PolicyKit/sudo rights"
                        ).color(YELLOW).size(12.0));
                    });
                });
        }

        // ── Status header ─────────────────────────────────────────────────

        egui::Panel::top("header").show_inside(ui, |ui| {
            const ICON_W: f32 = 60.0;

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                let src = if armed {
                    egui::include_image!("../resources/guard-on.png")
                } else {
                    egui::include_image!("../resources/guard-off.png")
                };
                ui.add(egui::Image::new(src).fit_to_exact_size(egui::vec2(ICON_W, ICON_W)));
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    let (label, color) = if armed {
                        if test_mode {
                            ("[ ARMED — TEST MODE ]", YELLOW)
                        } else {
                            ("[ ARMED ]", GREEN)
                        }
                    } else if waiting {
                        ("[ Waiting for unplug... ]", YELLOW)
                    } else {
                        ("[ Disarmed ]", DIM)
                    };
                    ui.label(RichText::new(label).size(18.0).strong().color(color));
                    ui.add_space(4.0);
                    let key_text = if key_device.is_empty() {
                        RichText::new("Key device:  none").color(DIM)
                    } else {
                        RichText::new(format!("Key device:  {}", key_device))
                    };
                    ui.label(key_text);
                    ui.label(
                        RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .color(DIM)
                            .small(),
                    );
                });
            });
            ui.add_space(8.0);
        });

        // ── Toolbar ───────────────────────────────────────────────────────

        egui::Panel::top("toolbar")
            .show_inside(ui, |ui| {
                ui.add_space(2.0);
                ui.horizontal_wrapped(|ui| {
                    // Map / Cancel
                    let (map_txt, map_enabled) = if waiting {
                        ("Cancel Mapping", true)
                    } else {
                        ("Map Device", !armed)
                    };
                    let map_tooltip = if waiting {
                        "Cancel the current mapping operation"
                    } else {
                        "Click, then unplug a USB device to set it as the key device automatically"
                    };
                    if ui.add_enabled(map_enabled, egui::Button::new(map_txt))
                        .on_hover_text(map_tooltip)
                        .clicked()
                    {
                        let mut s = self.state.lock().unwrap();
                        if s.waiting {
                            s.waiting = false;
                            drop(s);
                            push_log(&mut self.log, "Mapping cancelled.");
                        } else {
                            s.armed   = false;
                            s.waiting = true;
                            drop(s);
                            push_log(&mut self.log, "Waiting — unplug the device to map as key...");
                        }
                    }

                    // Arm / Disarm
                    let (arm_txt, arm_color) = if armed {
                        ("Disarm", RED)
                    } else {
                        ("Arm Sentinel", GREEN)
                    };
                    let arm_tooltip = if armed {
                        "Disarm the sentinel — removing the key device will no longer shut down the PC"
                    } else if key_device.is_empty() {
                        "Set a key device first (click Set Key in the device list)"
                    } else {
                        "Arm the sentinel — removing the key device will immediately shut down the PC"
                    };
                    if ui.add_enabled(
                        !key_device.is_empty() && !waiting,
                        egui::Button::new(RichText::new(arm_txt).color(arm_color))
                            .stroke(Stroke::new(1.0, arm_color)),
                    )
                    .on_hover_text(arm_tooltip)
                    .clicked()
                    {
                        let mut s = self.state.lock().unwrap();
                        s.armed = !s.armed;
                        let msg = if s.armed { "Sentinel ARMED." } else { "Sentinel DISARMED." };
                        drop(s);
                        push_log(&mut self.log, msg);
                    }

                    // Test mode toggle
                    let mut tm = test_mode;
                    if ui.checkbox(&mut tm, RichText::new("Test mode")
                        .color(if test_mode { YELLOW } else { DIM }))
                        .on_hover_text("Safe dry-run: shows a popup instead of triggering a real shutdown when the key device is removed")
                        .changed()
                    {
                        self.state.lock().unwrap().test_mode = tm;
                        self.cfg.test_mode = tm;
                        self.cfg.save();
                    }

                    if ui.button(RichText::new("Advanced").color(DIM).small())
                        .on_hover_text("Hooks, autostart, shutdown on close, and other advanced settings")
                        .clicked()
                    {
                        self.show_advanced = true;
                    }

                });
                ui.separator();
                // Second row: right-aligned utility buttons
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Exit")
                        .on_hover_text("Quit the application completely")
                        .clicked() { std::process::exit(0); }
                    if ui.small_button("About")
                        .on_hover_text("Version and project information")
                        .clicked() { self.show_about = true; }
                    if ui.small_button("Help")
                        .on_hover_text("How to use xxUSBSentinel")
                        .clicked() { self.show_help  = true; }
                    if ui.small_button("Report Bug")
                        .on_hover_text("Open a pre-filled GitHub issue with your version and OS")
                        .clicked() { open_bug_report_url(); }
                    });
                });
            });

        // ── Body ─────────────────────────────────────────────────────────

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let body_frame = egui::Frame::NONE;
            let max_devices_w = (ui.available_width() - 200.0).max(180.0);
            egui::Panel::left("devices_panel")
                .resizable(true)
                .min_size(180.0)
                .max_size(max_devices_w)
                .default_size(300.0)
                .frame(body_frame)
                .show_inside(ui, |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.strong("Connected USB Devices");
                            ui.label(RichText::new(format!("({})", self.devices.len())).color(DIM).small());
                        });
                        ui.separator();

                        if self.devices.is_empty() {
                            ui.add_space(8.0);
                            ui.label(RichText::new("No USB devices detected").color(DIM).italics());
                        } else {
                            // Compute name column width from the panel's current available width
                            // (before entering the Grid, where available_width() would return cell width).
                            // Distribute all remaining width between name (55%) and comment (45%).
                            // Using min+max together forces the grid columns to fill the space.
                            let fixed = 75.0 + 58.0 + 32.0 + 16.0; // VID:PID + action + spacing + margins
                            let remaining = (ui.available_width() - fixed).max(120.0);
                            let name_col_w = (remaining * 0.55).floor();
                            let comment_w  = (remaining * 0.45).floor();

                            egui::ScrollArea::vertical().id_salt("devices").show(ui, |ui| {
                                egui::Grid::new("device_grid")
                                    .num_columns(4)
                                    .striped(true)
                                    .spacing([8.0, 4.0])
                                    .show(ui, |ui| {
                                        // Header row
                                        ui.label(RichText::new("VID:PID").color(DIM).small());
                                        ui.label(RichText::new("Device Name").color(DIM).small());
                                        ui.label(RichText::new("Comment").color(DIM).small());
                                        ui.label(RichText::new("").color(DIM).small());
                                        ui.end_row();

                                        for i in 0..self.devices.len() {
                                            let vid_pid = self.devices[i].vid_pid.clone();
                                            let name    = self.devices[i].name.clone();
                                            let is_key  = !key_device.is_empty() && vid_pid == key_device;
                                            let row_color = if is_key { GREEN } else { Color32::from_rgb(200, 200, 210) };

                                            // VID:PID column
                                            let id_resp = ui.add(
                                                egui::Label::new(RichText::new(&vid_pid).color(row_color).monospace())
                                                    .sense(egui::Sense::hover()),
                                            );
                                            let comment_for_menu = self.cfg.device_comments
                                                .get(&vid_pid).cloned().unwrap_or_default();
                                            let last_event = self.log.iter().rev()
                                                .find(|e| e.text.contains(&vid_pid))
                                                .map(|e| format!("[{}]  {}", e.time, e.text));
                                            let vid_only = vid_pid.split(':').next().unwrap_or("").to_string();
                                            let pid_only = vid_pid.split(':').nth(1).unwrap_or("").to_string();

                                            // Name column — width tracks panel size, truncates overflow
                                            let name_text = if name.is_empty() {
                                                RichText::new("Unknown device").color(row_color).italics()
                                            } else {
                                                RichText::new(&name).color(row_color)
                                            };
                                            let name_resp = ui.scope(|ui| {
                                                ui.set_min_width(name_col_w);
                                                ui.set_max_width(name_col_w);
                                                ui.add(egui::Label::new(name_text).truncate().sense(egui::Sense::hover()))
                                            }).inner;

                                            // Context menu on the whole row (VID:PID + Name columns)
                                            (id_resp | name_resp).context_menu(|ui| {
                                                ui.set_min_width(180.0);
                                                if ui.button("Copy VID:PID").clicked() {
                                                    copy_to_clipboard(&vid_pid);
                                                    ui.close();
                                                }
                                                if ui.button("Copy VID").clicked() {
                                                    copy_to_clipboard(&vid_only);
                                                    ui.close();
                                                }
                                                if ui.button("Copy PID").clicked() {
                                                    copy_to_clipboard(&pid_only);
                                                    ui.close();
                                                }
                                                if ui.button("Copy device info").clicked() {
                                                    let info = display_label(&vid_pid, &name, &comment_for_menu);
                                                    copy_to_clipboard(&info);
                                                    ui.close();
                                                }
                                                if let Some(ref ev) = last_event {
                                                    if ui.button("Copy last event").clicked() {
                                                        copy_to_clipboard(ev);
                                                        ui.close();
                                                    }
                                                }
                                                ui.separator();
                                                if ui.button("Resolve online").clicked() {
                                                    open_url(&format!(
                                                        "https://devicehunt.com/view/type/usb/vendor/{}/device/{}",
                                                        vid_only, pid_only
                                                    ));
                                                    ui.close();
                                                }
                                            });

                                            // Comment column — inline editable, saved to config
                                            let mut comment = self.cfg.device_comments
                                                .get(&vid_pid).cloned().unwrap_or_default();
                                            ui.scope(|ui| {
                                                ui.set_min_width(comment_w);
                                                ui.set_max_width(comment_w);
                                                let edit = egui::TextEdit::singleline(&mut comment)
                                                    .id(egui::Id::new("comment").with(&vid_pid))
                                                    .desired_width(comment_w)
                                                    .hint_text("add note…")
                                                    .font(egui::TextStyle::Small);
                                                if ui.add(edit).changed() {
                                                    if comment.is_empty() {
                                                        self.cfg.device_comments.remove(&vid_pid);
                                                    } else {
                                                        self.cfg.device_comments.insert(vid_pid.clone(), comment);
                                                    }
                                                    self.cfg.save();
                                                }
                                            });

                                            // Action column
                                            if is_key {
                                                ui.label(RichText::new("KEY").color(GREEN).small().strong());
                                            } else if !armed {
                                                if ui.small_button("Set Key").clicked() {
                                                    {
                                                        let mut s = self.state.lock().unwrap();
                                                        s.key_device = vid_pid.clone();
                                                        s.waiting = false;
                                                    }
                                                    self.cfg.key_device = vid_pid.clone();
                                                    self.cfg.save();
                                                    let label = if name.is_empty() {
                                                        vid_pid.clone()
                                                    } else {
                                                        format!("{} ({})", vid_pid, name)
                                                    };
                                                    push_log(&mut self.log, &format!("Key device set to {}.", label));
                                                }
                                            } else {
                                                ui.label("");
                                            }

                                            ui.end_row();
                                        }
                                    });
                            });
                        }
                    });
                });

            // Right: event log fills all remaining space
            egui::CentralPanel::default().frame(body_frame).show_inside(ui, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong("Event Log");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Export").clicked() { export_log(&self.log); }
                            if ui.small_button("Clear").clicked()  { self.log.clear(); }
                            if ui.small_button("Copy All").clicked() {
                                let text = self.log.iter()
                                    .map(|e| format!("[{}]  {}", e.time, e.text))
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                if let Ok(mut cb) = arboard::Clipboard::new() {
                                    let _ = cb.set_text(text);
                                }
                            }
                        });
                    });
                    ui.separator();

                    if self.log.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("No events yet").color(DIM).italics());
                    } else {
                        egui::ScrollArea::vertical()
                            .id_salt("log")
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                for entry in &self.log {
                                    let color = if entry.text.contains("ARMED") || entry.text.contains("mapped") {
                                        GREEN
                                    } else if entry.text.contains("DISARMED") || entry.text.contains("Disconnected") {
                                        RED
                                    } else if entry.text.contains("Test triggered") || entry.text.contains("Waiting") {
                                        YELLOW
                                    } else if entry.text.contains("Connected") {
                                        Color32::from_rgb(80, 160, 220)
                                    } else {
                                        Color32::from_rgb(180, 180, 180)
                                    };
                                    let text = format!("[{}]  {}", entry.time, entry.text);
                                    let resp = ui.add(
                                        egui::Label::new(RichText::new(&text).size(12.0).color(color))
                                            .selectable(true),
                                    );
                                    resp.context_menu(|ui| {
                                        if ui.button("Copy").clicked() {
                                            if let Ok(mut cb) = arboard::Clipboard::new() {
                                                let _ = cb.set_text(text.clone());
                                            }
                                            ui.close();
                                        }
                                    });
                                }
                            });
                    }
                });
            });
        });
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn dark_visuals() -> egui::Visuals {
    let mut v = egui::Visuals::dark();
    v.window_corner_radius = egui::CornerRadius::same(6);
    v.window_shadow = egui::Shadow::NONE;
    v.panel_fill = Color32::from_rgb(22, 22, 28);
    v.window_fill = Color32::from_rgb(30, 30, 38);
    v.extreme_bg_color = Color32::from_rgb(14, 14, 18);
    v.faint_bg_color = Color32::from_rgb(28, 28, 36);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(50, 50, 60));
    v
}

fn now() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

fn push_log(log: &mut Vec<LogEntry>, text: &str) {
    log.push(LogEntry {
        time: now(),
        text: text.to_string(),
    });
}

fn open_url(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(url).spawn();
    }
}

fn collect_cmd(args: &[&str]) -> String {
    std::process::Command::new(args[0])
        .args(&args[1..])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn open_bug_report_url() {
    let version = env!("CARGO_PKG_VERSION");
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let kernel = collect_cmd(&["uname", "-r"]);
    let de = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "unknown".into());
    let display = std::env::var("WAYLAND_DISPLAY")
        .map(|_| "Wayland")
        .unwrap_or_else(|_| {
            if std::env::var("DISPLAY").is_ok() {
                "X11"
            } else {
                "unknown"
            }
        });
    let title = format!("[Bug] - xxUSBSentinel v{} on {}/{}", version, os, arch);
    let body    = format!(
        "**Version:** {}\n**OS:** {}\n**Arch:** {}\n**Kernel:** {}\n**Desktop:** {}\n**Display server:** {}\n\n\
         **Describe the bug:**\n<!-- A clear description of what went wrong -->\n\n\
         **Steps to reproduce:**\n1. \n2. \n3. \n\n\
         **Expected behaviour:**\n\n\
         **Actual behaviour:**\n",
        version, os, arch, kernel, de, display
    );
    let url = format!(
        "https://github.com/thereisnotime/xxUSBSentinel/issues/new?title={}&body={}",
        url_encode(&title),
        url_encode(&body),
    );
    open_url(&url);
}

fn url_encode(s: &str) -> String {
    s.bytes()
        .flat_map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => vec![b as char],
            b' ' => vec!['+'],
            b => format!("%{:02X}", b).chars().collect(),
        })
        .collect()
}

fn run_hooks(hooks: &[Hook], vid_pid: &str, name: &str, event: &str) {
    for hook in hooks {
        if !hook.enabled {
            continue;
        }
        if hook.script.trim().is_empty() {
            continue;
        }
        if hook.event != event {
            continue;
        }
        // "triggered" always matches — it only ever fires for the key device
        if hook.event != "triggered" && hook.device != "*" && hook.device != vid_pid {
            continue;
        }
        let _ = std::process::Command::new(hook.script.trim())
            .args([vid_pid, name, event])
            .spawn();
    }
}

fn copy_to_clipboard(text: &str) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text.to_string());
    }
}

fn display_label(vid_pid: &str, name: &str, comment: &str) -> String {
    match (name.is_empty(), comment.is_empty()) {
        (true, true) => vid_pid.to_string(),
        (false, true) => format!("{} ({})", vid_pid, name),
        (true, false) => format!("{} [{}]", vid_pid, comment),
        (false, false) => format!("{} ({}) [{}]", vid_pid, name, comment),
    }
}

fn export_log(log: &[LogEntry]) {
    use std::io::Write;
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Export Log")
        .add_filter("Log files", &["log", "txt"])
        .save_file()
    {
        if let Ok(mut f) = std::fs::File::create(&path) {
            for e in log {
                let _ = writeln!(f, "[{}]  {}", e.time, e.text);
            }
        }
    }
}

// CentralPanel::show(ctx) is the only valid top-level panel API inside a viewport
// callback where no &mut Ui is available; show_inside() requires a pre-existing Ui.
#[allow(deprecated)]
fn bsod_panel(ctx: &egui::Context, img_bytes: Option<(&'static [u8], &'static str)>) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::BLACK))
        .show(ctx, |ui| {
            if let Some((bytes, uri)) = img_bytes {
                ui.add(egui::Image::from_bytes(uri, bytes).fit_to_exact_size(ui.available_size()));
            }
        });
}
