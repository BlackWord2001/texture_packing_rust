use clap::Parser;
use image::ImageFormat;

#[derive(Parser, Debug)]
#[command(name = "texpack")]
#[command(about = "轻量级 CLI 贴图打包工具")]
struct Args {
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("输入文件：{}", args.input);
    println!("输出文件：{}", args.output);

    let img = image::open(&args.input).expect("无法读取输入图片");

    println!(
        "图片尺寸: {} x {}, 颜色类型:{:?}",
        img.width(),
        img.height(),
        img.color()
    );

    img.save_with_format(&args.output, ImageFormat::Png)
        .expect("无法保存输出图片");

    println!("✅ 已保存到：{}",args.output);
}
