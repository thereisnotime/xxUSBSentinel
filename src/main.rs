// On Windows, prevent the OS from allocating a console window when the binary
// is launched.  Without this attribute, double-clicking the .exe (or having it
// run at startup) opens a black CMD window whose closure immediately kills the
// process.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod config;
mod sentinel;
mod shutdown;
mod tray;
mod usb;
mod wipe;

use config::Config;
use sentinel::SharedState;

fn main() {
    // Enforce single instance on Windows via a named kernel mutex.
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        let name: Vec<u16> = OsStr::new("Local\\xxUSBSentinel_SingleInstance")
            .encode_wide()
            .chain(once(0u16))
            .collect();
        // SAFETY: null-terminated wide string; NULL security attrs/owner are fine.
        let handle = unsafe {
            windows_sys::Win32::System::Threading::CreateMutexW(
                std::ptr::null(),
                0,
                name.as_ptr(),
            )
        };
        let last_err = unsafe { windows_sys::Win32::Foundation::GetLastError() };
        if last_err == windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS {
            std::process::exit(0);
        }
        std::mem::forget(handle);
    }

    let cfg = Config::load();

    let state = SharedState::new_from_config(&cfg);
    let (tx, rx) = std::sync::mpsc::channel();
    usb::start_monitor(state.clone(), tx);

    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("xxUSBSentinel")
            .with_inner_size([780.0, 560.0])
            .with_min_inner_size([560.0, 420.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "xxUSBSentinel",
        options,
        Box::new(move |cc| Ok(Box::new(app::SentinelApp::new(cc, state, rx, cfg)))),
    )
    .expect("eframe failed");
}

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../resources/guard-on.png");
    let img = image::load_from_memory(bytes).expect("icon").into_rgba8();
    let (width, height) = img.dimensions();
    egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    }
}
