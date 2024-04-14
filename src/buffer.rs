use crate::enums::CFRColor;

/// The `CFRBuffer` struct represents a buffer that stores color data.
///
/// It contains the width and height of the buffer, as well as the color data.
///
/// # Examples
///
/// ```
/// use cfrs::buffer::CFRBuffer;
///
/// let buffer = CFRBuffer::new(256, 256);
/// assert_eq!(buffer.width, 256);
/// assert_eq!(buffer.height, 256);
/// assert_eq!(buffer.data.len(), 256 * 256);
/// ```
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
