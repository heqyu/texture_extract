use clap::{arg, Command};
use plist::Value;
use std::fs::File;
use std::io::Read;
use std::path::Path;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("PngExtractor")
        .version("1.0")
        .about("Extract PNG images from a plist file")
        .arg(arg!(-r --rotate "是否进行旋转矫正").action(clap::ArgAction::SetTrue))
        .arg(
            arg!(-a --angle "旋转矫正角度")
                .action(clap::ArgAction::Append)
                .default_value("270"),
        )
        .arg(arg!(<input> "输入的 PNG 文件路径"))
        .arg(arg!(<output> "输出目录"))
        .get_matches();

    let rotate_output = matches.get_flag("rotate");
    let png_path = matches.get_one::<String>("input").unwrap();
    let output_folder = matches.get_one::<String>("output").unwrap();
    let angle = matches.get_one::<String>("angle").unwrap();
    let plist_path = Path::new(png_path).with_extension("plist");

    println!(
        "PNG 文件路径: {}, 输出目录: {}, 是否进行旋转矫正: {}, 旋转矫正角度: {}",
        png_path, output_folder, rotate_output, angle,
    );

    // 读取 plist 文件
    let mut file = File::open(plist_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // 解析 plist 文件
    let plist_value: Value = plist::from_bytes(&contents)?;

    // 读取整个图片
    let mut image = image::open(png_path)?;

    // 创建输出文件夹
    std::fs::create_dir_all(output_folder)?;

    // 解析 plist 文件中的帧信息
    if let Value::Dictionary(dict) = plist_value {
        if let Some(Value::Dictionary(frames)) = dict.get("frames") {
            for (frame_name, frame_info) in frames {
                if let Value::Dictionary(frame_dict) = frame_info {
                    if let Some(Value::String(frame_rect_str)) = frame_dict.get("frame") {
                        // 解析帧的矩形区域
                        let frame_rect = parse_rect(frame_rect_str)?;

                        // 获取偏移和旋转数据
                        let offset = if let Some(Value::String(offset_str)) = frame_dict.get("offset") {
                            parse_offset(offset_str)?
                        } else {
                            (0, 0)
                        };

                        let rotated = if let Some(Value::Boolean(is_rotated)) = frame_dict.get("rotated") {
                            *is_rotated
                        } else {
                            false
                        };

                        let (x, y, width, height) = if rotated {
                            // 如果旋转，交换宽度和高度
                            (
                                frame_rect.0 + offset.0,
                                frame_rect.1 + offset.1,
                                frame_rect.3,
                                frame_rect.2,
                            )
                        } else {
                            (
                                frame_rect.0 + offset.0,
                                frame_rect.1 + offset.1,
                                frame_rect.2,
                                frame_rect.3,
                            )
                        };

                        // 裁剪图像
                        let mut sub_image = image.crop(x, y, width, height);
                        if rotated && rotate_output {
                            match angle.as_str() {
                                "90" => sub_image = sub_image.rotate90(),
                                "180" => sub_image = sub_image.rotate180(),
                                "270" => sub_image = sub_image.rotate270(),
                                _ => println!("Invalid angle value"),
                            }
                        }

                        // 保存裁剪后的图像
                        sub_image.save(Path::new(output_folder).join(frame_name))?
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_rect(rect_str: &str) -> Result<(u32, u32, u32, u32), Box<dyn std::error::Error>> {
    let rect_str = rect_str.replace('{', "").replace('}', "");
    let parts: Vec<&str> = rect_str.split(',').collect();
    if parts.len() == 4 {
        let x = parts[0].trim().parse::<u32>()?;
        let y = parts[1].trim().parse::<u32>()?;
        let width = parts[2].trim().parse::<u32>()?;
        let height = parts[3].trim().parse::<u32>()?;
        Ok((x, y, width, height))
    } else {
        Err("Invalid rect string".into())
    }
}

// 函数解析偏移位置
fn parse_offset(offset_str: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let cleaned_str = offset_str.replace('{', "").replace('}', "");
    let parts: Vec<&str> = cleaned_str.split(',').collect();

    if parts.len() == 2 {
        let x = parts[0].trim().parse::<u32>()?;
        let y = parts[1].trim().parse::<u32>()?;
        Ok((x, y))
    } else {
        Err("Invalid offset string".into())
    }
}
