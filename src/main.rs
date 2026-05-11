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
