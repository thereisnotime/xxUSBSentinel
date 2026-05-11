use std::sync::{Arc, Mutex};

/// State shared between the USB monitor thread and the GUI thread.
pub struct SharedState {
    pub armed: bool,
    pub test_mode: bool,
    /// True while waiting for a device to be unplugged for mapping.
    pub waiting: bool,
    /// VID:PID of the mapped key device (e.g. "046D:C52B").
    pub key_device: String,
}

impl SharedState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            armed: false,
            test_mode: false,
            waiting: false,
            key_device: String::new(),
        }))
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub time: String,
    pub text: String,
}

/// Events sent from the USB monitor thread to the GUI.
pub enum GuiEvent {
    Log(LogEntry),
    DeviceMapped(String),
    TestTriggered,
}
