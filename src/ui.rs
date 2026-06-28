use eframe::egui;

use crate::app::TexpackApp;

pub fn render_central_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("hello world");
    });
}
