// 发布版本在 Windows 上运行时不显示额外的命令行窗口；调试版本仍保留终端，
// 这样 println! 输出和报错信息更容易查看。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 声明当前 crate 中的两个模块，对应 src/app.rs 和 src/ui.rs。
mod app;
mod ui;

fn main() {
    // NativeOptions 保存原生窗口配置。这里先使用 eframe 提供的默认配置。
    let native_options = eframe::NativeOptions::default();

    // 启动 eframe 的事件循环并创建应用窗口。
    eframe::run_native(
        // 原生窗口标题。
        "Texture Packing",
        native_options,
        // cc 是创建应用时的上下文。闭包返回装箱后的 TexpackApp，
        // Box 用于让 eframe 通过统一的 App trait 管理具体应用类型。
        Box::new(|cc| Ok(Box::new(app::TexpackApp::new(cc)))),
    )
    // run_native 返回 Result；如果启动失败，unwrap 会立即显示错误并结束程序。
    .unwrap();
}
