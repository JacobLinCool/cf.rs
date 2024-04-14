use crate::buffer::CFRBuffer;
use crate::enums::{CFRColor, CFRDirection};

#[derive(Debug, Copy, Clone)]
pub struct CFRPainter {
    pub direction: CFRDirection,
    pub color: CFRColor,
    pub x: u32,
    pub y: u32,
}

impl Default for CFRPainter {
    fn default() -> Self {
        Self::new()
    }
}

impl CFRPainter {
    pub fn new() -> CFRPainter {
        CFRPainter {
            direction: CFRDirection::Up,
            color: CFRColor::White,
            x: 0,
            y: 0,
        }
    }

    pub fn change_color(&mut self) {
        self.color = match self.color {
            CFRColor::White => CFRColor::Black,
            CFRColor::Black => CFRColor::Blue,
            CFRColor::Blue => CFRColor::Green,
            CFRColor::Green => CFRColor::Cyan,
            CFRColor::Cyan => CFRColor::Red,
            CFRColor::Red => CFRColor::Magenta,
            CFRColor::Magenta => CFRColor::Yellow,
            CFRColor::Yellow => CFRColor::White,
        };
    }

    pub fn rotate(&mut self) {
        self.direction = match self.direction {
            CFRDirection::Up => CFRDirection::UpRight,
            CFRDirection::UpRight => CFRDirection::Right,
            CFRDirection::Right => CFRDirection::DownRight,
            CFRDirection::DownRight => CFRDirection::Down,
            CFRDirection::Down => CFRDirection::DownLeft,
            CFRDirection::DownLeft => CFRDirection::Left,
            CFRDirection::Left => CFRDirection::UpLeft,
            CFRDirection::UpLeft => CFRDirection::Up,
        };
    }

    pub fn move_forward_and_draw(&mut self, buffer: &mut CFRBuffer) {
        let mut dx = 0;
        let mut dy = 0;
        match self.direction {
            CFRDirection::Up => {
                dy = -1;
            }
            CFRDirection::UpRight => {
                dx = 1;
                dy = -1;
            }
            CFRDirection::Right => {
                dx = 1;
            }
            CFRDirection::DownRight => {
                dx = 1;
                dy = 1;
            }
            CFRDirection::Down => {
                dy = 1;
            }
            CFRDirection::DownLeft => {
                dx = -1;
                dy = 1;
            }
            CFRDirection::Left => {
                dx = -1;
            }
            CFRDirection::UpLeft => {
                dx = -1;
                dy = -1;
            }
        }

        if self.x == 0 && dx == -1 {
            self.x = buffer.width - 1;
        } else if self.x == buffer.width - 1 && dx == 1 {
            self.x = 0;
        } else {
            self.x = (self.x as i32 + dx) as u32;
        }

        if self.y == 0 && dy == -1 {
            self.y = buffer.height - 1;
        } else if self.y == buffer.height - 1 && dy == 1 {
            self.y = 0;
        } else {
            self.y = (self.y as i32 + dy) as u32;
        }

        let index = (self.y * buffer.width + self.x) as usize;
        buffer.data[index] = self.color;
    }
}
