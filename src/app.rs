use std::sync::{mpsc, Arc, Mutex};

use eframe::egui;
use tray_icon::{TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::sentinel::{GuiEvent, LogEntry, SharedState};

const ICON_OFF: &[u8] = include_bytes!("../xxUSBSentinel/Resources/guard-off.png");
const ICON_ON: &[u8] = include_bytes!("../xxUSBSentinel/Resources/guard-on.png");

pub struct SentinelApp {
    state: Arc<Mutex<SharedState>>,
    rx: mpsc::Receiver<GuiEvent>,
    log: Vec<LogEntry>,
    tray: TrayIcon,
    prev_armed: bool,
    show_test_popup: bool,
    show_help: bool,
    show_about: bool,
}

impl SentinelApp {
    pub fn new(
        cc: &eframe::CreationContext,
        state: Arc<Mutex<SharedState>>,
        rx: mpsc::Receiver<GuiEvent>,
    ) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let tray = TrayIconBuilder::new()
            .with_tooltip("xxUSBSentinel — disarmed")
            .with_icon(decode_tray_icon(ICON_OFF))
            .build()
            .expect("failed to create tray icon");

        Self {
            state,
            rx,
            log: Vec::new(),
            tray,
            prev_armed: false,
            show_test_popup: false,
            show_help: false,
            show_about: false,
        }
    }
}

impl eframe::App for SentinelApp {
    /// Runs before every `ui` call, and also when the window is minimised.
    /// Used to drain USB events so the sentinel stays active even when hidden.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                GuiEvent::Log(entry) => self.log.push(entry),
                GuiEvent::DeviceMapped(vp) => {
                    self.log.push(LogEntry {
                        time: now_str(),
                        text: format!("Key device mapped: {}", vp),
                    });
                }
                GuiEvent::TestTriggered => {
                    self.show_test_popup = true;
                }
            }
        }

        // Keep waking up so we process events even when minimised
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        // Tray click → restore window
        while let Ok(event) = TrayIconEvent::receiver().try_recv() {
            match event {
                TrayIconEvent::Click { .. } | TrayIconEvent::DoubleClick { .. } => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                }
                _ => {}
            }
        }

        // Update tray icon when armed state changes
        let armed = self.state.lock().unwrap().armed;
        if armed != self.prev_armed {
            self.prev_armed = armed;
            let icon = if armed { decode_tray_icon(ICON_ON) } else { decode_tray_icon(ICON_OFF) };
            let tip = if armed { "xxUSBSentinel — ARMED" } else { "xxUSBSentinel — disarmed" };
            let _ = self.tray.set_icon(Some(icon));
            let _ = self.tray.set_tooltip(Some(tip));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Intercept window close → minimise to tray
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }

        // --- Popups ---

        if self.show_test_popup {
            egui::Window::new("Test Mode")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&ctx, |ui| {
                    ui.label("Good thing this is just a test.");
                    ui.add_space(8.0);
                    if ui.button("  OK  ").clicked() {
                        self.show_test_popup = false;
                    }
                });
        }

        if self.show_help {
            egui::Window::new("Help")
                .collapsible(false)
                .resizable(false)
                .max_width(440.0)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&ctx, |ui| {
                    ui.label("1.  Map a Key USB device (mouse, keyboard, flash drive, etc.).");
                    ui.label("2.  Arm the Sentinel.");
                    ui.add_space(4.0);
                    ui.label(
                        "If the Key device is unplugged while armed, the PC shuts down \
                         immediately — making encrypted drive key recovery almost impossible.",
                    );
                    ui.add_space(8.0);
                    ui.label("⚠  This does not encrypt your data. Use VeraCrypt for that.");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_help = false;
                    }
                });
        }

        if self.show_about {
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(&ctx, |ui| {
                    ui.heading("xxUSBSentinel v2.0");
                    ui.add_space(4.0);
                    ui.label("Rewritten in Rust — cross-platform (Linux & Windows).");
                    ui.label("https://github.com/thereisnotime");
                    ui.add_space(8.0);
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }

        // --- Layout ---

        let (armed, test_mode, waiting, key_device) = {
            let s = self.state.lock().unwrap();
            (s.armed, s.test_mode, s.waiting, s.key_device.clone())
        };

        egui::Panel::top("controls").show_inside(ui, |ui| {
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                let guard_src = if armed {
                    egui::include_image!("../xxUSBSentinel/Resources/guard-on.png")
                } else {
                    egui::include_image!("../xxUSBSentinel/Resources/guard-off.png")
                };
                ui.add(egui::Image::new(guard_src).max_size(egui::vec2(56.0, 56.0)));

                ui.add_space(8.0);

                ui.vertical(|ui| {
                    let status = if armed {
                        if test_mode { "ARMED  (test mode)" } else { "ARMED" }
                    } else if waiting {
                        "Waiting — unplug the desired USB device..."
                    } else {
                        "Disarmed"
                    };
                    ui.heading(status);

                    let key_label = if key_device.is_empty() { "none" } else { &key_device };
                    ui.label(format!("Key device:  {}", key_label));
                });
            });

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                let map_text = if waiting { "Unplug desired USB..." } else { "Map USB Device" };
                if ui
                    .add_enabled(!armed && !waiting, egui::Button::new(map_text))
                    .clicked()
                {
                    let mut s = self.state.lock().unwrap();
                    s.armed = false;
                    s.waiting = true;
                    drop(s);
                    self.log.push(LogEntry {
                        time: now_str(),
                        text: "Waiting for device to be unplugged for mapping...".into(),
                    });
                }

                let arm_text = if armed { "Disarm Sentinel" } else { "Arm Sentinel" };
                let has_key = !key_device.is_empty();
                if ui
                    .add_enabled(has_key && !waiting, egui::Button::new(arm_text))
                    .clicked()
                {
                    let mut s = self.state.lock().unwrap();
                    s.armed = !s.armed;
                    let msg = if s.armed { "Sentinel ARMED." } else { "Sentinel disarmed." };
                    drop(s);
                    self.log.push(LogEntry { time: now_str(), text: msg.into() });
                }

                let test_text = if test_mode { "Disable Test Mode" } else { "Enable Test Mode" };
                if ui.button(test_text).clicked() {
                    self.state.lock().unwrap().test_mode = !test_mode;
                }
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("Export Log").clicked() {
                    export_log(&self.log);
                }
                if ui.button("Clear Log").clicked() {
                    self.log.clear();
                }
                if ui.button("Help").clicked() {
                    self.show_help = true;
                }
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
            });

            ui.add_space(4.0);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for entry in &self.log {
                        let text = format!("{}: {}", entry.time, entry.text);
                        let response = ui.selectable_label(false, &text);
                        response.context_menu(|ui| {
                            if ui.button("Copy line").clicked() {
                                if let Ok(mut cb) = arboard::Clipboard::new() {
                                    let _ = cb.set_text(text.clone());
                                }
                                ui.close();
                            }
                        });
                    }
                });
        });
    }
}

fn decode_tray_icon(bytes: &[u8]) -> tray_icon::Icon {
    let img = image::load_from_memory(bytes)
        .expect("failed to decode tray icon")
        .into_rgba8();
    let (w, h) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), w, h).expect("failed to create tray icon")
}

fn now_str() -> String {
    chrono::Local::now()
        .format("%d/%m/%Y %H:%M:%S")
        .to_string()
}

fn export_log(log: &[LogEntry]) {
    use std::io::Write;
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Export Log")
        .add_filter("Log / text files", &["log", "txt"])
        .save_file()
    {
        if let Ok(mut f) = std::fs::File::create(&path) {
            for entry in log {
                let _ = writeln!(f, "{}: {}", entry.time, entry.text);
            }
        }
    }
}
