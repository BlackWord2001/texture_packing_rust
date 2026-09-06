// UI 布局，功能实现请放其他文件
use eframe::egui;

use crate::app::TexpackApp;

// 中央面板
pub fn render_central_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("texture packing");
    });
}

// 顶部面板
pub fn render_top_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
    egui::Panel::top("my top panel").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    #[cfg(debug_assertions)]
                    println!("Open clicked");
                }
    
                if ui.button("Save").clicked() {
                    #[cfg(debug_assertions)]
                    println!("Save clicked");
                }
    
                if ui.button("Exit").clicked() {
                    #[cfg(debug_assertions)]
                    println!("Exit clicked");

                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });


            ui.menu_button("Preferences", |ui| {
                ui.menu_button("Theme", |ui| {
                    if ui.button("Auto").clicked() {
                        #[cfg(debug_assertions)]
                        println!("Auto Theme");
                    }
                    
                    if ui.button("Light Theme").clicked() {
                        #[cfg(debug_assertions)]
                        println!("select light theme");
                    }

                    if ui.button("Dark Theme").clicked() {
                        #[cfg(debug_assertions)]
                        println!("select drak theme");
                    }
                });
                
            });
        });
    });
}

// pub fn render_left_panel(ui: &mut egui::Ui, app: &mut TexpackApp) {
//     egui::Panel::left("my left panel").show(ui, |ui| {
//         ui.label("left panel");
//     });
// }
