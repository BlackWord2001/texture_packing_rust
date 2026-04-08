use clap::Parser;
use image::{DynamicImage, GenericImageView, RgbaImage};

// 输入指令处理
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
    #[arg(short, long)]
    mode: PackMode,
    args: Vec<String>,
}

fn parse_input(s: &str) -> Result<ChannelInput, String> {
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

// 核心打包逻辑
fn pack_channels(inputs: &[ChannelInput], mode: &PackMode) -> Result<RgbaImage, String> {
    let mut images: Vec<DynamicImage> = Vec::new();
    let mut base_size: Option<(u32, u32)> = None;

    for inp in inputs {
        if let ChannelInput::Image(path) = inp {
            let img = image::open(path).map_err(|e| format!("无法读取图片 {}: {}", path, e))?;
            let size = (img.width(), img.height());

            match base_size {
                None => {
                    base_size = Some(size);
                }
                Some(bs) if bs != size => {
                    return Err(format!("图片尺寸不匹配：{:?} vs {:?}", bs, size));
                }
                _ => {}
            }

            images.push(img);
        } else {
            images.push(DynamicImage::new_luma8(0, 0));
        }
    }

    let (width, height) = base_size.ok_or("没有提供任何图片输入")?;

    // 创建输出画布
    let mut output = RgbaImage::new(width, height);

    // 把 alpha 通道填充为不透明255
    for pixel in output.pixels_mut() {
        pixel.0[3] = 255;
    }

    // 逐通道填充
    let channel_count = match mode {
        PackMode::RGB => 3,
        PackMode::RGBA => 4,
    };

    // 建立图片索引（只统计 Image 类型的）
    let mut img_index = 0;
    for (ch, inp) in inputs.iter().enumerate() {
        if ch >= channel_count {
            break; // 安全保护
        }

        match inp {
            ChannelInput::Color(val) => {
                for y in 0..height {
                    for x in 0..width {
                        let pixel = output.get_pixel_mut(x, y);
                        pixel.0[ch] = *val; // 0=R, 1=G, 2=B, 3=A
                    }
                }
            }
            ChannelInput::Image(_) => {
                let src = &images[img_index];
                img_index += 1;

                // 取源图的第一个通道（灰度/红色）
                for y in 0..height {
                    for x in 0..width {
                        let p = src.get_pixel(x, y);
                        let gray = p.0[0]; // 取 R 通道作为单通道值
                        let pixel = output.get_pixel_mut(x, y);
                        pixel.0[ch] = gray;
                    }
                }
            }
        }
    }

    Ok(output)
}

fn main() {
    let args = Args::parse();

    let expected_count = match args.mode {
        PackMode::RGB => 4,
        PackMode::RGBA => 5,
    };

    if args.args.len() != expected_count {
        eprintln!(
            "错误: {:?} 模式需要 {} 个输入 + 输出路径，共 {} 个参数，但你给了 {} 个",
            args.mode,
            expected_count - 1,
            expected_count,
            args.args.len()
        );
        std::process::exit(1);
    }

    let output_path = args.args.last().unwrap();
    let input_strs: Vec<&String> = args.args.iter().take(expected_count - 1).collect();

    let inputs: Vec<ChannelInput> = input_strs
        .iter()
        .map(|s| parse_input(s))
        .collect::<Result<_, _>>()
        .expect("参数解析失败");

    println!("📦 开始打包...");
    println!("模式：{:?}", args.mode);

    for (i, inp) in inputs.iter().enumerate() {
        println!("通道 {}：{:?}", i, inp);
    }
    println!(" 输出：{}", output_path);

    let result = pack_channels(&inputs, &args.mode).expect("打包失败");

    result.save(output_path).expect("保存失败");

    println!("👌 完成！已经保存到 {}", output_path);
}
