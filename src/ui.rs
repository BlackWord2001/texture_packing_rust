// UI 布局，功能实现请放其他文件
use eframe::egui;

use crate::app::{AppPage, ChannelInput, TexToolApp};

// 绘制中央面板。app 使用可变引用，因为卡片中的控件会修改各通道状态。
pub fn render_central_panel(ui: &mut egui::Ui, app: &mut TexToolApp) {
    // CentralPanel 会使用顶部等其他面板绘制后剩下的区域。
    egui::CentralPanel::default().show(ui, |ui| {
        match &app.current_page {
            AppPage::Packing => {
                ui.horizontal(|ui| {
                    // &mut app.red 只把 red 字段可变借给函数，而不是转移字段所有权。
                    render_channel_card(ui, "R", &mut app.red);
                    render_channel_card(ui, "G", &mut app.green);
                    render_channel_card(ui, "B", &mut app.blue);
                    render_channel_card(ui, "A", &mut app.alpha);
                });

                ui.separator();

                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.heading("Preview");
                        ui.label("Output texture preview");
                    });
                    columns[1].group(|ui| {
                        ui.heading("Export");
                        ui.label("Output settings");

                        egui::ComboBox::from_id_salt("output_format")
                            .selected_text("PNG")
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut app.output_format,
                                    "PNG".to_string(),
                                    "PNG",
                                );
                            });

                        ui.label("Output path");

                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [180.0, 24.0],
                                egui::TextEdit::singleline(&mut app.output_path),
                            );
                            let _ = ui.button("...");
                        });

                        ui.separator();

                        if ui.button("Export").clicked() {
                            // 之后实现真正的图片导出
                        }
                    });
                });
            }
            AppPage::Splitting => {
                ui.heading("Texture Splitting");
            }
            AppPage::Sdf => {
                ui.heading("SDF");
            }
        }
    });
}

pub fn render_top_panel(ui: &mut egui::Ui, app: &mut TexToolApp) {
    // 字符串 "my top panel" 是面板的持久 ID，需要在界面中保持唯一和稳定。
    egui::Panel::top("top panel").show(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            // 文件菜单
            ui.menu_button("File", |ui| {
                if ui.button("Exit").clicked() {
                    #[cfg(debug_assertions)]
                    println!("Exit clicked");

                    // 向当前原生窗口发送关闭命令。
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            // 偏好设置
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

            // 分隔线
            ui.separator();

            // 页面切换
            ui.selectable_value(&mut app.current_page, AppPage::Packing, "Packing");
            ui.selectable_value(&mut app.current_page, AppPage::Splitting, "Splitting");
            ui.selectable_value(&mut app.current_page, AppPage::Sdf, "SDF");
        });
    });
}

fn render_channel_card(ui: &mut egui::Ui, channel_name: &str, channel: &mut ChannelInput) {
    // group 为内部控件添加卡片式背景、边框和内边距。
    ui.group(|ui| {
        // 明确让卡片内容从上到下排列，避免继承外层的 horizontal 横向布局。
        ui.vertical(|ui| {
            ui.set_width(160.0);

            // 显示当前输出通道的名称。
            ui.heading(channel_name);

            // 申请一个固定大小的预览区域。allocate_exact_size 返回区域 rect 和交互 response。
            let preview_size = egui::vec2(160.0, 160.0);
            let (rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());

            // 目前还没有加载图片，所以使用默认灰度填充预览区域。
            // from_gray 会生成 R、G、B 值相同的 Color32。
            ui.painter()
                .rect_filled(rect, 4.0, egui::Color32::from_gray(channel.fallback));

            // 暂时显示文件名占位文字，之后可以从 channel.path 中提取真实文件名。
            ui.label("file name");

            ui.separator();

            // 这个闭包只让两个按钮在卡片内部横向排列。
            ui.horizontal(|ui| {
                // 选择文件功能尚未实现；用 let _ 接收 Response，表示暂时忽略按钮结果。
                let _ = ui.button("select image");

                if ui.button("clear").clicked() {
                    // 清空保存的路径，恢复到“没有选择图片”的状态。
                    channel.path.clear();
                }
            });

            // 默认灰度控件和颜色预览在同一行显示。
            ui.horizontal(|ui| {
                ui.label("default color");

                // &mut channel.fallback 让 DragValue 可以直接修改状态中的 u8 值。
                ui.add(
                    egui::DragValue::new(&mut channel.fallback)
                        // 鼠标拖动和手动输入都限制在有效的 8 位灰度范围内。
                        .range(0..=255)
                        // 鼠标水平拖动一个逻辑点时，数值大约变化 1。
                        .speed(1.0),
                );

                // 根据当前灰度值计算小色块颜色，并申请 20 x 20 的绘制区域。
                let color = egui::Color32::from_gray(channel.fallback);
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());

                // 在申请到的区域内绘制圆角为 2 的实心颜色方块。
                ui.painter().rect_filled(rect, 2.0, color);
            });
        });
    });
}
