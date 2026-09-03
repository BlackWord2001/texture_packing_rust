use eframe::egui;

use crate::app::TexpackApp;

pub fn render_central_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("central panel");

        
    });
}

pub fn render_top_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::Panel::top("my top panel").show(ui, |ui| {
        
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    println!("Open clicked");
                }
    
                if ui.button("Save").clicked() {
                    println!("Save clicked");
                }
    
                if ui.button("Exit").clicked() {
                    println!("Exit clicked");
                }
            });

            ui.menu_button("Preferences", |ui| {
                if ui.button("Theme").clicked() {
                    println!("select theme");
                }
            });
        });

    });
}

// pub fn render_left_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
//     egui::Panel::left("my left panel").show(ui, |ui| {
//         ui.label("left panel");
//     });
// }
