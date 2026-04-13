use clap::Parser;

// 输入指令处理
#[derive(Debug, Clone)]
pub enum ChannelInput {
    Image(String),
    Color(u8),
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PackMode {
    RGB,
    RGBA,
}

#[derive(Parser, Debug)]
#[command(name = "texpack")]
#[command(about = "轻量级 CLI 贴图打包工具")]
pub struct Args {
    #[arg(short, long)]
    pub mode: PackMode,
    pub args: Vec<String>,
}

pub fn parse_input(s: &str) -> Result<ChannelInput, String> {
    match s.to_lowercase().as_str() {
        "black" => return Ok(ChannelInput::Color(0)),
        "white" => return Ok(ChannelInput::Color(255)),
        _ => {}
    }

    if let Ok(v) = s.parse::<f64>() {
        if !(0.0..=1.0).contains(&v) {
            return Err(format!("数值{} 超出范围，必须在0-1之间", v));
        }
        return Ok(ChannelInput::Color((v * 255.0).round() as u8));
    }

    Ok(ChannelInput::Image(s.to_string()))
}
