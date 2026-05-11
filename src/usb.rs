use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use crate::sentinel::{GuiEvent, SharedState, UsbDevice};
use crate::shutdown;

/// Starts the USB polling monitor on a background thread.
/// Polls every 500 ms via rusb; triggers shutdown directly from this thread
/// so the sentinel fires even when the GUI window is minimised.
pub fn start_monitor(state: Arc<Mutex<SharedState>>, tx: mpsc::Sender<GuiEvent>) {
    std::thread::Builder::new()
        .name("usb-monitor".into())
        .spawn(move || {
            // key: (bus, address)  value: (vid, pid)
            let mut known: HashMap<(u8, u8), (u16, u16)> = HashMap::new();

            if let Ok(list) = rusb::devices() {
                let mut initial: Vec<UsbDevice> = Vec::new();
                for dev in list.iter() {
                    if let Ok(desc) = dev.device_descriptor() {
                        let vid = desc.vendor_id();
                        let pid = desc.product_id();
                        known.insert((dev.bus_number(), dev.address()), (vid, pid));
                        let vp = vid_pid_str(vid, pid);
                        // Deduplicate by VID:PID — the sentinel keys on VID:PID, not bus/address
                        if !initial.iter().any(|d| d.vid_pid == vp) {
                            initial.push(make_device(vid, pid, &dev));
                        }
                    }
                }
                let _ = tx.send(GuiEvent::InitialDevices(initial));
            }

            loop {
                std::thread::sleep(Duration::from_millis(500));

                // Build current map and detect new devices in one pass so we have
                // the rusb::Device object available for make_device() name lookup.
                let mut current: HashMap<(u8, u8), (u16, u16)> = HashMap::new();
                if let Ok(list) = rusb::devices() {
                    for dev in list.iter() {
                        if let Ok(desc) = dev.device_descriptor() {
                            let key = (dev.bus_number(), dev.address());
                            let vid = desc.vendor_id();
                            let pid = desc.product_id();
                            current.insert(key, (vid, pid));

                            if !known.contains_key(&key) {
                                let device = make_device(vid, pid, &dev);
                                let _ = tx.send(GuiEvent::DeviceConnected(device));
                            }
                        }
                    }
                }

                // Detect removed devices
                for (&_key, &(vid, pid)) in &known {
                    if !current.contains_key(&_key) {
                        let vp = vid_pid_str(vid, pid);
                        let _ = tx.send(GuiEvent::DeviceDisconnected(vp.clone()));

                        let action = {
                            let mut s = state.lock().unwrap();
                            if s.waiting {
                                s.key_device = vp.clone();
                                s.waiting = false;
                                Action::Mapped(vp.clone())
                            } else if s.armed && vp == s.key_device {
                                if s.test_mode { Action::TestTrigger } else { Action::Shutdown }
                            } else {
                                Action::None
                            }
                        };

                        match action {
                            Action::Mapped(v) => { let _ = tx.send(GuiEvent::DeviceMapped(v)); }
                            Action::TestTrigger => {
                                crate::shutdown::notify(
                                    "xxUSBSentinel — Test triggered",
                                    "Key device removed. Shutdown suppressed (test mode).",
                                );
                                let _ = tx.send(GuiEvent::TestTriggered);
                            }
                            Action::Shutdown => {
                                crate::shutdown::notify(
                                    "xxUSBSentinel — SHUTDOWN",
                                    "Key device removed. Shutting down now.",
                                );
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

fn make_device(vid: u16, pid: u16, dev: &rusb::Device<rusb::GlobalContext>) -> UsbDevice {
    let vp = vid_pid_str(vid, pid);

    // 1. Try the device descriptor string (needs open, may require permissions)
    let descriptor_name = dev.open().ok().and_then(|h| {
        let timeout = Duration::from_millis(100);
        let desc = dev.device_descriptor().ok()?;
        let lang = h.read_languages(timeout).ok()?.into_iter().next()?;
        h.read_product_string(lang, &desc, timeout).ok()
    });

    // 2. Fall back to the embedded USB ID database
    let name = descriptor_name.unwrap_or_else(|| {
        usb_ids::Device::from_vid_pid(vid, pid)
            .map(|d| format!("{} {}", d.vendor().name(), d.name()))
            .unwrap_or_default()
    });

    UsbDevice { vid_pid: vp, name }
}

fn vid_pid_str(vid: u16, pid: u16) -> String {
    format!("{:04X}:{:04X}", vid, pid)
}


enum Action { None, Mapped(String), TestTrigger, Shutdown }
