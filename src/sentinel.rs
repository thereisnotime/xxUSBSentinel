use std::sync::{Arc, Mutex};

use crate::config::Config;

/// State shared between the USB monitor thread and the GUI thread.
pub struct SharedState {
    pub armed: bool,
    pub test_mode: bool,
    /// True while waiting for a device to be unplugged for mapping.
    pub waiting: bool,
    /// VID:PID of the mapped key device (e.g. "046D:C52B").
    pub key_device: String,
    /// Trigger shutdown (or test-mode alert) if the app is closed while armed.
    pub shutdown_on_close: bool,
    /// Run wipe_swap() before shutdown on trigger.
    pub wipe_swap: bool,
    /// Run wipe_hiberfil() before shutdown on trigger.
    pub wipe_hiberfil: bool,
    /// Show fake BSOD overlay before shutdown.
    pub fake_bsod: bool,
    /// "win10" | "win11" | "linux"
    pub bsod_style: String,
}

impl SharedState {
    pub fn new_from_config(cfg: &Config) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            armed: false,
            test_mode: cfg.test_mode,
            waiting: false,
            key_device: cfg.key_device.clone(),
            shutdown_on_close: cfg.shutdown_on_close,
            wipe_swap: cfg.wipe_swap,
            wipe_hiberfil: cfg.wipe_hiberfil,
            fake_bsod: cfg.fake_bsod,
            bsod_style: cfg.bsod_style.clone(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub time: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub vid_pid: String,
    pub name: String,
}

/// Events sent from the USB monitor thread to the GUI.
pub enum GuiEvent {
    DeviceConnected(UsbDevice),
    DeviceDisconnected(String),
    DeviceMapped(String),
    TestTriggered,
    /// Full snapshot sent once on startup.
    InitialDevices(Vec<UsbDevice>),
    /// Real shutdown trigger — GUI thread handles wipe + fake BSOD + shutdown.
    ShutdownTriggered {
        wipe_swap: bool,
        wipe_hiberfil: bool,
        fake_bsod: bool,
        bsod_style: String,
    },
}
