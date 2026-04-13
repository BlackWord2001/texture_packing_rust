mod cli;
mod packer;

use clap::Parser;

use cli::{Args, ChannelInput, PackMode, parse_input};
use packer::pack_channels;

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
