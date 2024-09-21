// use std::process::Command;
use std::fs;

fn main() {
    // 指定输出目录
    let output_dir = r"G:\我写的工具";
    // 获取可执行文件的路径
    let exe_name = if cfg!(target_os = "windows") {
        "texture_extract.exe"
    } else {
        "texture_extract"
    };

    // 拷贝可执行文件
    let source_path = format!("target/{}/{}", std::env::var("PROFILE").unwrap(), exe_name);
    let destination_path = format!("{}/{}", output_dir, exe_name);

    // 使用 fs 进行文件拷贝
    match fs::copy(&source_path, &destination_path) {
        Ok(_) => println!("Successfully copied {} to {}", source_path, destination_path),
        Err(e) => eprintln!("Failed to copy file: {}", e),
    }
}
