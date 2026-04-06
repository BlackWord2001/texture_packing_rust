use std::fmt::format;

use clap::Parser;
use image::ImageFormat;

#[derive(Debug, Clone)]
enum ChannelInput {
    Image(String),
    Color(u8),
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum PackMode {
    RGB,
    RGBA,
}

#[derive(Parser, Debug)]
#[command(name = "texpack")]
#[command(about = "轻量级 CLI 贴图打包工具")]
struct Args {
    #[arg(short = 'R')]
    mode: PackMode,
    args: Vec<String>,
}

fn parse_input(s:&str) -> Result<ChannelInput, String> {
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

fn main() {
    let args = Args::parse();

    let expected_count = match args.mode {
        PackMode:: RGB => 4,
        PackMode:: RGBA => 5,
    };

    if args.args.len() != expected_count {
        eprintln!("错误: {} 模式需要 {} 个输入参数 + 1 个输出路径",
        format!("{:?}", args.mode), expected_count - 1);

        std::process::exit(1);
    }

    let input_strs: Vec<&String> = args.args.iter().take(expected_count - 1).collect();
    let output_path = args.args.last().unwrap();

    println!("模式：{:?}", args.mode);
    println!("输入：{:?}", input_strs);
    println!("输出：{}", output_path);

    
}
