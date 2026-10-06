#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

// 声明当前 crate 中的两个模块，对应 src/app.rs 和 src/ui.rs。
mod app;
mod ui;

fn main() {
    // NativeOptions 保存原生窗口配置。这里先使用 eframe 提供的默认配置。
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([735.0, 720.0]),
        ..Default::default()
    };

    // 启动 eframe 的事件循环并创建应用窗口。
    eframe::run_native(
        // 原生窗口标题。
        "Texture tools",
        native_options,
        // cc 是创建应用时的上下文。闭包返回装箱后的 TexpackApp，
        // Box 用于让 eframe 通过统一的 App trait 管理具体应用类型。
        Box::new(|cc| Ok(Box::new(app::TexToolApp::new(cc)))),
    )
    // run_native 返回 Result；如果启动失败，unwrap 会立即显示错误并结束程序。
    .unwrap();
}
