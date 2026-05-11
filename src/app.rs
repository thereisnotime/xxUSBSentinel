use std::sync::{mpsc, Arc, Mutex};

use eframe::egui::{self, Color32, RichText, Stroke};

use crate::config::{autostart_enabled, set_autostart, Config};
use crate::sentinel::{GuiEvent, LogEntry, SharedState, UsbDevice};
use crate::tray::{AppTray, TrayEvent};

const GREEN:  Color32 = Color32::from_rgb(80,  200, 100);
const RED:    Color32 = Color32::from_rgb(220,  60,  60);
const YELLOW: Color32 = Color32::from_rgb(230, 190,  50);
const DIM:    Color32 = Color32::from_rgb(130, 130, 130);

pub struct SentinelApp {
    state:            Arc<Mutex<SharedState>>,
    rx:               mpsc::Receiver<GuiEvent>,
    log:              Vec<LogEntry>,
    devices:          Vec<UsbDevice>,
    tray:             AppTray,
    cfg:              Config,
    autostart:        bool,
    shutdown_ok:      bool,
    prev_armed:       bool,
    prev_has_key:     bool,
    prev_test_mode:   bool,
    prev_soc:         bool,
    show_help:        bool,
    show_about:       bool,
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

        let autostart    = autostart_enabled();
        let shutdown_ok  = crate::shutdown::can_shutdown();
        Self {
            state,
            rx,
            log: Vec::new(),
            devices: Vec::new(),
            tray: AppTray::new(),
            cfg,
            autostart,
            shutdown_ok,
            prev_armed:      false,
            prev_has_key:    false,
            prev_test_mode:  false,
            prev_soc:        false,
            show_help:       false,
            show_about:      false,
        }
    }
}

impl eframe::App for SentinelApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain USB events
        while let Ok(event) = self.rx.try_recv() {
            match event {
                GuiEvent::Log(e)               => self.log.push(e),
                GuiEvent::InitialDevices(list) => self.devices = list,
                GuiEvent::DeviceConnected(d)   => {
                    if !self.devices.iter().any(|x| x.vid_pid == d.vid_pid) {
                        let comment = self.cfg.device_comments.get(&d.vid_pid).cloned().unwrap_or_default();
                        let label = display_label(&d.vid_pid, &d.name, &comment);
                        push_log(&mut self.log, &format!("Connected: {}", label));
                        self.devices.push(d);
                    }
                }
                GuiEvent::DeviceDisconnected(vp) => {
                    let name = self.devices.iter().find(|d| d.vid_pid == vp)
                        .map(|d| d.name.clone()).unwrap_or_default();
                    let comment = self.cfg.device_comments.get(&vp).cloned().unwrap_or_default();
                    let label = display_label(&vp, &name, &comment);
                    push_log(&mut self.log, &format!("Disconnected: {}", label));
                    self.devices.retain(|d| d.vid_pid != vp);
                }
                GuiEvent::DeviceMapped(vp) => {
                    self.cfg.key_device = vp.clone();
                    self.cfg.save();
                    let name = self.devices.iter().find(|d| d.vid_pid == vp)
                        .map(|d| d.name.clone()).unwrap_or_default();
                    let comment = self.cfg.device_comments.get(&vp).cloned().unwrap_or_default();
                    let label = display_label(&vp, &name, &comment);
                    push_log(&mut self.log, &format!("Key device mapped: {}", label));
                }
                GuiEvent::TestTriggered => {
                    push_log(&mut self.log, "Test triggered — key device removed (shutdown suppressed).");
                }
            }
        }

        // Sync tray state whenever any relevant field changes
        let (armed, has_key, test_mode_s, soc_s) = {
            let s = self.state.lock().unwrap();
            (s.armed, !s.key_device.is_empty(), s.test_mode, s.shutdown_on_close)
        };
        if armed != self.prev_armed || has_key != self.prev_has_key
            || test_mode_s != self.prev_test_mode || soc_s != self.prev_soc
        {
            self.prev_armed      = armed;
            self.prev_has_key    = has_key;
            self.prev_test_mode  = test_mode_s;
            self.prev_soc        = soc_s;
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
                        let msg = if s.armed { "Sentinel ARMED (via tray)." } else { "Sentinel DISARMED (via tray)." };
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
                    push_log(&mut self.log, if new_val { "Test mode ON (via tray)." } else { "Test mode OFF (via tray)." });
                }
                TrayEvent::ToggleShutdownOnClose => {
                    let new_val = {
                        let mut s = self.state.lock().unwrap();
                        s.shutdown_on_close = !s.shutdown_on_close;
                        s.shutdown_on_close
                    };
                    self.cfg.shutdown_on_close = new_val;
                    self.cfg.save();
                    push_log(&mut self.log, if new_val { "Shutdown on close ON (via tray)." } else { "Shutdown on close OFF (via tray)." });
                }
            }
        }

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Close button: either minimise to tray, or shut down if armed and shutdown_on_close
        if ctx.input(|i| i.viewport().close_requested()) {
            let s = self.state.lock().unwrap();
            let should_shutdown = s.armed && s.shutdown_on_close;
            let is_test = s.test_mode;
            drop(s);
            if should_shutdown {
                if is_test {
                    push_log(&mut self.log, "Test triggered — window closed while armed (shutdown suppressed).");
                } else {
                    crate::shutdown::execute();
                }
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        let (armed, test_mode, waiting, key_device, shutdown_on_close) = {
            let s = self.state.lock().unwrap();
            (s.armed, s.test_mode, s.waiting, s.key_device.clone(), s.shutdown_on_close)
        };

        // ── Popups ────────────────────────────────────────────────────────

        if self.show_help {
            egui::Window::new("Help")
                .collapsible(false).resizable(false).max_width(460.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&ctx, |ui| {
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
                });
        }

        if self.show_about {
            egui::Window::new("About")
                .collapsible(false).resizable(false).min_width(280.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&ctx, |ui| {
                    ui.heading("xxUSBSentinel  v2.0");
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

        egui::Panel::top("header")
            .show_inside(ui, |ui| {
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
                            if test_mode { ("[ ARMED — TEST MODE ]", YELLOW) }
                            else         { ("[ ARMED ]",             GREEN)  }
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
                        ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).color(DIM).small());
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

                    // Shutdown on close
                    let mut soc = shutdown_on_close;
                    if ui.checkbox(&mut soc, RichText::new("Shutdown on close")
                        .color(if shutdown_on_close { RED } else { DIM }))
                        .on_hover_text("When armed, closing the window triggers shutdown instead of minimising to tray. Respects Test mode.")
                        .changed()
                    {
                        self.state.lock().unwrap().shutdown_on_close = soc;
                        self.cfg.shutdown_on_close = soc;
                        self.cfg.save();
                    }

                    // Autostart toggle
                    let mut ast = self.autostart;
                    if ui.checkbox(&mut ast, RichText::new("Autostart")
                        .color(if self.autostart { GREEN } else { DIM }))
                        .on_hover_text("Launch xxUSBSentinel automatically when you log in")
                        .changed()
                    {
                        set_autostart(ast);
                        self.autostart = ast;
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
            let body_frame = egui::Frame::none();
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
                                            id_resp.context_menu(|ui| {
                                                if ui.button("Copy VID:PID").clicked() {
                                                    if let Ok(mut cb) = arboard::Clipboard::new() {
                                                        let _ = cb.set_text(vid_pid.clone());
                                                    }
                                                    ui.close();
                                                }
                                            });

                                            // Name column — width tracks panel size, truncates overflow
                                            let name_text = if name.is_empty() {
                                                RichText::new("Unknown device").color(row_color).italics()
                                            } else {
                                                RichText::new(&name).color(row_color)
                                            };
                                            ui.scope(|ui| {
                                                ui.set_min_width(name_col_w);
                                                ui.set_max_width(name_col_w);
                                                ui.add(egui::Label::new(name_text).truncate());
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
    v.window_shadow        = egui::Shadow::NONE;
    v.panel_fill           = Color32::from_rgb(22, 22, 28);
    v.window_fill          = Color32::from_rgb(30, 30, 38);
    v.extreme_bg_color     = Color32::from_rgb(14, 14, 18);
    v.faint_bg_color       = Color32::from_rgb(28, 28, 36);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(50, 50, 60));
    v
}

fn now() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

fn push_log(log: &mut Vec<LogEntry>, text: &str) {
    log.push(LogEntry { time: now(), text: text.to_string() });
}

fn open_url(url: &str) {
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(url).spawn(); }
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("explorer").arg(url).spawn(); }
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
    let os      = std::env::consts::OS;
    let arch    = std::env::consts::ARCH;
    let kernel  = collect_cmd(&["uname", "-r"]);
    let de      = std::env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| std::env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "unknown".into());
    let display = std::env::var("WAYLAND_DISPLAY")
        .map(|_| "Wayland")
        .unwrap_or_else(|_| if std::env::var("DISPLAY").is_ok() { "X11" } else { "unknown" });
    let title   = format!("[Bug] - xxUSBSentinel v{} on {}/{}", version, os, arch);
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
    s.bytes().flat_map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
        | b'-' | b'_' | b'.' | b'~' => vec![b as char],
        b' ' => vec!['+'],
        b => format!("%{:02X}", b).chars().collect(),
    }).collect()
}

fn display_label(vid_pid: &str, name: &str, comment: &str) -> String {
    match (name.is_empty(), comment.is_empty()) {
        (true,  true)  => vid_pid.to_string(),
        (false, true)  => format!("{} ({})", vid_pid, name),
        (true,  false) => format!("{} [{}]", vid_pid, comment),
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
            for e in log { let _ = writeln!(f, "[{}]  {}", e.time, e.text); }
        }
    }
}
