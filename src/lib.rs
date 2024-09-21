use plist::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod atlas_parser;

#[derive(Debug)]
pub struct Frame {
    pub name: String,
    pub rect: (u32, u32, u32, u32),
    pub offset: (u32, u32),
    pub rotated: bool,
}

impl Frame {
    pub fn into_rotated_rect(self) -> Frame {
        // 如果旋转，交换宽度和高度
        if self.rotated {
            Frame {
                rect: (self.rect.0, self.rect.1, self.rect.3, self.rect.2),
                rotated: false,
                ..self
            }
        } else {
            self
        }
    }
}

pub fn parse_plist_frames<P: AsRef<Path>>(file_path: P) -> Result<Vec<Frame>, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let plist_value: Value = plist::from_bytes(&contents)?;
    let mut frames_list: Vec<Frame> = Vec::new();

    if let Value::Dictionary(dict) = plist_value {
        if let Some(Value::Dictionary(frames)) = dict.get("frames") {
            for (frame_name, frame_info) in frames {
                if let Value::Dictionary(frame_dict) = frame_info {
                    if let Some(Value::String(frame_rect_str)) = frame_dict.get("frame") {
                        let frame_rect = parse_rect(frame_rect_str)?;

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

                        let frame = Frame {
                            name: frame_name.clone(),
                            rect: frame_rect,
                            offset,
                            rotated,
                        };

                        frames_list.push(frame);
                    }
                }
            }
        }
    }

    Ok(frames_list)
}

// 解析矩形区域
fn parse_rect(rect_str: &str) -> Result<(u32, u32, u32, u32), Box<dyn Error>> {
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

// 解析偏移位置
fn parse_offset(offset_str: &str) -> Result<(u32, u32), Box<dyn Error>> {
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
