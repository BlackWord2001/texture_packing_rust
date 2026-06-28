use eframe::egui;

use crate::ui;

#[derive(Default)]
pub struct TexpackApp {}

impl TexpackApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TexpackApp {
    fn ui(&mut self, ctx: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui::render_central_panel(ctx, self);
    }
}
