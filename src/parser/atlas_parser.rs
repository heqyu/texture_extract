use crate::Frame;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn parse<P: AsRef<Path>>(file_path: P) -> Result<Vec<Frame>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let lines : Vec<&str> = contents.lines().skip(3).collect(); // 跳过前三行

    let mut frames: Vec<Frame> = Vec::new();
    let mut current_name = String::new();

    for line in lines {
        // let line = line.trim();
        if line.is_empty() {
            continue; // 跳过空行
        }

        // 不包含 ：
        if line.find(':').is_none() {
            // 处理新的帧名称
            current_name = line.trim_end_matches(':').to_string();
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "rotate" => {
                    let rotated = value == "true";
                    frames.push(Frame {
                        name: current_name.clone(),
                        rotated,
                        offset: (0, 0),
                        rect: (0, 0, 0, 0),
                    });
                }
                "xy" => {
                    if let Some(pos) = frames.last_mut() {
                        let coords: Vec<u32> = value.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                        pos.rect.0 = coords[0];
                        pos.rect.1 = coords[1];
                    }
                }
                "size" => {
                    if let Some(pos) = frames.last_mut() {
                        let sizes: Vec<u32> = value.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                        pos.rect.2 = sizes[0];
                        pos.rect.3 = sizes[1];
                    }
                }
                // "orig" => {
                //     if let Some(pos) = frames.last_mut() {
                //         let orig_sizes: Vec<u32> = value.split(',').filter_map(|s| s.parse().ok()).collect();
                //         pos.orig = (orig_sizes[0], orig_sizes[1]);
                //     }
                // }
                "offset" => {
                    if let Some(pos) = frames.last_mut() {
                        let offsets: Vec<u32> = value.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                        pos.offset = (offsets[0], offsets[1]);
                    }
                }
                // "index" => {
                //     if let Some(pos) = frames.last_mut() {
                //         pos.index = value.parse().unwrap_or(-1);
                //     }
                // }
                _ => {}
            }
        }
    }

    // println!("frames: {:#?}", frames);

    Ok(frames)
}