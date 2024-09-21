
pub mod parser;

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
