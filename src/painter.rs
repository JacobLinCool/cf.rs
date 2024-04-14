use crate::buffer::CFRBuffer;
use crate::enums::{CFRColor, CFRDirection};

/// The CFRPainter struct represents a painter that moves around a buffer and draws points.
/// It keeps track of the painter's direction, color, and position.
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
    /// Creates a new instance of CFRPainter with default values.
    pub fn new() -> CFRPainter {
        CFRPainter {
            direction: CFRDirection::Up,
            color: CFRColor::White,
            x: 0,
            y: 0,
        }
    }

    /// Changes the color of the painter.
    /// The color changes in the following order: White -> Black -> Blue -> Green -> Cyan -> Red -> Magenta -> Yellow -> White.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::painter::CFRPainter;
    /// use cfrs::enums::CFRColor;
    ///
    /// let mut painter = CFRPainter::new();
    /// assert_eq!(painter.color, CFRColor::White);
    /// painter.change_color();
    /// assert_eq!(painter.color, CFRColor::Black);
    /// painter.change_color();
    /// assert_eq!(painter.color, CFRColor::Blue);
    /// ```
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

    /// Rotates the painter's direction.
    /// The direction rotates in the following order: Up -> UpRight -> Right -> DownRight -> Down -> DownLeft -> Left -> UpLeft -> Up.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::painter::CFRPainter;
    /// use cfrs::enums::CFRDirection;
    ///
    /// let mut painter = CFRPainter::new();
    /// assert_eq!(painter.direction, CFRDirection::Up);
    /// painter.rotate();
    /// assert_eq!(painter.direction, CFRDirection::UpRight);
    /// painter.rotate();
    /// assert_eq!(painter.direction, CFRDirection::Right);
    /// ```
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

    /// Moves the painter forward and draws a point in the buffer.
    /// The painter moves one step in the current direction and draws a point with the current color.
    /// If the painter reaches the edge of the buffer, it wraps around to the opposite edge.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to the `CFRBuffer` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::{CFRBuffer, CFRPainter};
    ///
    /// let mut buffer = CFRBuffer::new(256, 256);
    /// let mut painter = CFRPainter::new();
    /// painter.x = 128;
    /// painter.y = 128;
    /// painter.move_forward_and_draw(&mut buffer);
    /// assert_eq!(buffer.data[(127 * 256 + 128) as usize], painter.color);
    /// ```
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

mod tests {
    #[test]
    fn test_change_color() {
        use crate::painter::CFRPainter;
        use crate::enums::CFRColor;

        let mut painter = CFRPainter::new();
        assert_eq!(painter.color, CFRColor::White);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Black);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Blue);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Green);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Cyan);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Red);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Magenta);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::Yellow);
        painter.change_color();
        assert_eq!(painter.color, CFRColor::White);
    }

    #[test]
    fn test_rotate() {
        use crate::painter::CFRPainter;
        use crate::enums::CFRDirection;

        let mut painter = CFRPainter::new();
        assert_eq!(painter.direction, CFRDirection::Up);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::UpRight);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::Right);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::DownRight);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::Down);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::DownLeft);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::Left);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::UpLeft);
        painter.rotate();
        assert_eq!(painter.direction, CFRDirection::Up);
    }

    #[test]
    fn move_over_upper_edge() {
        use crate::{CFRBuffer, CFRPainter};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 128;
        painter.y = 0;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 128);
        assert_eq!(painter.y, 255);
    }

    #[test]
    fn move_over_lower_edge() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 128;
        painter.y = 255;
        painter.direction = CFRDirection::Down;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 128);
        assert_eq!(painter.y, 0);
    }

    #[test]
    fn move_over_left_edge() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 0;
        painter.y = 128;
        painter.direction = CFRDirection::Left;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 255);
        assert_eq!(painter.y, 128);
    }

    #[test]
    fn move_over_right_edge() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 255;
        painter.y = 128;
        painter.direction = CFRDirection::Right;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 0);
        assert_eq!(painter.y, 128);
    }

    #[test]
    fn move_over_upper_left_corner() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 0;
        painter.y = 0;
        painter.direction = CFRDirection::UpLeft;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 255);
        assert_eq!(painter.y, 255);
    }

    #[test]
    fn move_over_upper_right_corner() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 255;
        painter.y = 0;
        painter.direction = CFRDirection::UpRight;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 0);
        assert_eq!(painter.y, 255);
    }

    #[test]
    fn move_over_lower_left_corner() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 0;
        painter.y = 255;
        painter.direction = CFRDirection::DownLeft;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 255);
        assert_eq!(painter.y, 0);
    }

    #[test]
    fn move_over_lower_right_corner() {
        use crate::{CFRBuffer, CFRPainter, CFRDirection};

        let mut buffer = CFRBuffer::new(256, 256);
        let mut painter = CFRPainter::new();
        painter.x = 255;
        painter.y = 255;
        painter.direction = CFRDirection::DownRight;
        painter.move_forward_and_draw(&mut buffer);
        assert_eq!(painter.x, 0);
        assert_eq!(painter.y, 0);
    }
}
