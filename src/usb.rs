use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use chrono::Local;

use crate::sentinel::{GuiEvent, LogEntry, SharedState};
use crate::shutdown;

/// Starts the USB polling monitor on a background thread.
/// Polls every 500 ms via rusb, detecting connect/disconnect by (bus, address).
/// Shutdown is triggered directly from this thread so it fires even when the GUI is minimised.
pub fn start_monitor(state: Arc<Mutex<SharedState>>, tx: mpsc::Sender<GuiEvent>) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || {
            // key: (bus, address)  value: (vid, pid)
            let mut known: HashMap<(u8, u8), (u16, u16)> = HashMap::new();

            if let Ok(list) = rusb::devices() {
                for dev in list.iter() {
                    if let Ok(desc) = dev.device_descriptor() {
                        known.insert(
                            (dev.bus_number(), dev.address()),
                            (desc.vendor_id(), desc.product_id()),
                        );
                    }
                }
            }

            loop {
                std::thread::sleep(Duration::from_millis(500));

                let mut current: HashMap<(u8, u8), (u16, u16)> = HashMap::new();
                if let Ok(list) = rusb::devices() {
                    for dev in list.iter() {
                        if let Ok(desc) = dev.device_descriptor() {
                            current.insert(
                                (dev.bus_number(), dev.address()),
                                (desc.vendor_id(), desc.product_id()),
                            );
                        }
                    }
                }

                for (&key, &(vid, pid)) in &current {
                    if !known.contains_key(&key) {
                        let vp = vid_pid_str(vid, pid);
                        send_log(&tx, &format!("Connected: {}", vp));
                    }
                }

                for (&_key, &(vid, pid)) in &known {
                    if !current.contains_key(&_key) {
                        let vp = vid_pid_str(vid, pid);
                        send_log(&tx, &format!("Disconnected: {}", vp));

                        let action = {
                            let mut s = state.lock().unwrap();
                            if s.waiting {
                                s.key_device = vp.clone();
                                s.waiting = false;
                                Action::Mapped(vp.clone())
                            } else if s.armed && vp == s.key_device {
                                if s.test_mode {
                                    Action::TestTrigger
                                } else {
                                    Action::Shutdown
                                }
                            } else {
                                Action::None
                            }
                        };

                        match action {
                            Action::Mapped(v) => {
                                let _ = tx.send(GuiEvent::DeviceMapped(v));
                            }
                            Action::TestTrigger => {
                                let _ = tx.send(GuiEvent::TestTriggered);
                            }
                            Action::Shutdown => {
                                shutdown::execute();
                            }
                            Action::None => {}
                        }
                    }
                }

                known = current;
            }
        })
        .expect("failed to spawn usb-monitor thread");
}

fn vid_pid_str(vid: u16, pid: u16) -> String {
    format!("{:04X}:{:04X}", vid, pid)
}

fn send_log(tx: &mpsc::Sender<GuiEvent>, text: &str) {
    let entry = LogEntry {
        time: Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        text: text.to_string(),
    };
    let _ = tx.send(GuiEvent::Log(entry));
}

enum Action {
    None,
    Mapped(String),
    TestTrigger,
    Shutdown,
}
