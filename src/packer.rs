use crate::cli::*;
use image::{DynamicImage, GenericImageView, RgbaImage};

// 核心打包逻辑
pub fn pack_channels(inputs: &[ChannelInput], mode: &PackMode) -> Result<RgbaImage, String> {
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
