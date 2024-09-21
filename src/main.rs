use atlas_parser::parse_atlas_file;
use clap::{arg, Command};
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::vec;
use texture_extract::*;

// 等待用户按任意键的函数
fn wait_for_user() {
    print!("按任意键以退出...");
    let _ = io::stdout().flush(); // 刷新输出
    let mut _n = String::new();
    let _ = io::stdin().read_line(&mut _n); // 等待用户输入
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("PngExtractor")
        .version("1.0")
        .about("Extract frames from a PNG file and save them as separate images")
        .arg(arg!(-r --rotate "是否进行旋转矫正").action(clap::ArgAction::SetTrue))
        .arg(
            arg!(-a --angle "旋转矫正角度 90 180 270")
                .action(clap::ArgAction::Append)
                .default_value("270"),
        )
        .arg(arg!(<input> "输入的 PNG 文件路径"))
        .arg(arg!(<output> "输出目录"))
        .try_get_matches().unwrap_or_else(|e| {
            eprintln!("参数错误: {}", e);
            wait_for_user();
            process::exit(1);
        });


    let rotate_output = matches.get_flag("rotate");
    let png_path = matches.get_one::<String>("input").unwrap_or_else(|| {
        eprintln!("参数错误: ");
        wait_for_user();
        process::exit(1);
    });

    let output_folder = matches.get_one::<String>("output").unwrap();
    let angle = matches.get_one::<String>("angle").unwrap().parse::<u32>().unwrap();

    println!(
        "PNG 文件路径: {}, 输出目录: {}, 是否进行旋转矫正: {}, 旋转矫正角度: {:?}",
        png_path, output_folder, rotate_output, angle,
    );

    let plist_path = Path::new(png_path).with_extension("plist");
    let mut frames = vec![];
    if plist_path.exists() {
        frames = parse_plist_frames(plist_path)?;
    } else {
        let atlast_path = Path::new(png_path).with_extension("atlas");
        if atlast_path.exists() {
            frames = parse_atlas_file(atlast_path)?;
        }
    }

    // 读取整个图片
    let mut image = image::open(png_path)?;

    // 创建输出文件夹
    std::fs::create_dir_all(output_folder)?;

    // 接下来可以使用 frames 进行后续操作
    for frame in frames {
        // println!("{:?}", frame);

        let rotated = frame.rotated;
        let frame = frame.into_rotated_rect();

        // 裁剪图像
        let (x, y, width, height) = frame.rect;
        let mut sub_image = image.crop(x, y, width, height);
        if rotated && rotate_output {
            match angle {
                90 => sub_image = sub_image.rotate90(),
                180 => sub_image = sub_image.rotate180(),
                270 => sub_image = sub_image.rotate270(),
                _ => println!("Invalid angle value"),
            }
        }

        // 保存裁剪后的图像
        // let mut path = Path::new(output_folder);
        let mut path = PathBuf::from(output_folder);
        let mut name = frame.name.clone();
        if !name.ends_with(".png") {
            name.push_str(".png");
        }
        // sd/swll_zm_1  如果包含 / 则会创建多级目录
        if name.contains("/") {
            for part in name.split("/") {
                path = path.join(part);
            }
            // 如果没有文件夹则创建
            if !path.exists() {
                std::fs::create_dir_all(path.parent().unwrap())?;
            }
        } else {
            path.push(name);
        }

        sub_image.save(path)?
    }

    Ok(())
}
