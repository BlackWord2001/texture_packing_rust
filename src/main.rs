mod app;
mod ui;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Texture Packing",
        native_options,
        Box::new(|cc| Ok(Box::new(app::TexpackApp::new(cc)))),
    )
    .unwrap();
}

