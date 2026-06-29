use eframe::egui;

use crate::app::TexpackApp;

pub fn render_central_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("central panel");
    });
}

pub fn render_top_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::Panel::top("my top panel").show(ui, |ui| {
        ui.label("top panel");
    });
}

pub fn render_left_panel(ui: &mut egui::Ui, app: & mut TexpackApp) {
    egui::Panel::left("my left panel").show(ui, |ui| {
        ui.label("left panel");
    });
}