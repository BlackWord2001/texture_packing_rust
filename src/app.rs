use eframe::egui;

use crate::ui;

// 应用的长期状态。egui 每一帧都会重新绘制界面，因此需要保留的数据不能只放在
// UI 函数的局部变量中，而应放在 TexpackApp 里。
// derive(Default) 会依次调用每个 ChannelInput 字段的 Default 实现。
#[derive(Default)]
pub struct TexToolApp {
    // 四个字段分别保存输出图片 R、G、B、A 通道的输入状态。
    pub red: ChannelInput,
    pub green: ChannelInput,
    pub blue: ChannelInput,
    pub alpha: ChannelInput,
    pub current_page: AppPage,
    pub output_format: String,
    pub output_path: String,
}

impl TexToolApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.all_styles_mut(|style|{
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(8);
            style.wrap_mode = Some(egui::TextWrapMode::Truncate);
        });

        // 创建包含四个默认通道的应用状态。
        Self::default()
    }
}

// 实现 eframe::App 后，TexpackApp 才能交给 eframe 运行。
impl eframe::App for TexToolApp {
    // 此方法会在需要绘制界面时反复调用。
    // &mut self 允许控件修改应用状态，例如灰度值和文件路径。
    fn ui(&mut self, ctx: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 先绘制顶部面板，再绘制占据剩余空间的中央面板。
        ui::render_top_panel(ctx, self);
        ui::render_central_panel(ctx, self);
        // ui::render_left_panel(ctx, self);
    }
}

// 单个输出通道需要保存的数据。
pub struct ChannelInput {
    // 已选择图片的完整路径。即使界面不显示路径输入框，程序仍需要保存路径，
    // 以便之后加载图片、显示文件名和执行清除操作。
    pub path: String,
    // 没有输入图片时使用的默认灰度值。u8 的取值范围正好是 0..=255。
    pub fallback: u8,
}

// 定义新建 ChannelInput 时的初始值。
impl Default for ChannelInput {
    fn default() -> Self {
        Self {
            // String::new() 创建一个空字符串，表示尚未选择图片。
            path: String::new(),
            // 默认使用白色。以后可以根据不同通道设置不同初始值。
            fallback: 0,
        }
    }
}

#[derive(Default, PartialEq)]
pub enum AppPage {
    #[default]
    Packing,
    Splitting,
    Sdf,
}
