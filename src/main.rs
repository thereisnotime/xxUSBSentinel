mod app;
mod sentinel;
mod shutdown;
mod usb;

use sentinel::SharedState;

fn main() {
    let state = SharedState::new();
    let (tx, rx) = std::sync::mpsc::channel();

    usb::start_monitor(state.clone(), tx);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("xxUSBSentinel")
            .with_inner_size([640.0, 520.0])
            .with_min_inner_size([480.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "xxUSBSentinel",
        options,
        Box::new(move |cc| Ok(Box::new(app::SentinelApp::new(cc, state, rx)))),
    )
    .expect("failed to start eframe");
}
