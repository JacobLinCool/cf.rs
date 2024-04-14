use crate::enums::CFRColor;

#[derive(Debug, Clone)]
pub struct CFRBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<CFRColor>,
}

impl CFRBuffer {
    pub fn new(width: u32, height: u32) -> CFRBuffer {
        CFRBuffer {
            width,
            height,
            data: vec![CFRColor::Black; (width * height) as usize],
        }
    }
}
