use crate::enums::CFRColor;
#[cfg(feature = "image")]
use image::{ImageBuffer, Rgb, Rgba};

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

    #[cfg(feature = "image")]
    /// Get the color at the specified coordinates as an `Rgb<u8>` value.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// An `Rgb<u8>` value representing the color at the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::buffer::CFRBuffer;
    /// use image::Rgb;
    ///
    /// let buffer = CFRBuffer::new(256, 256);
    /// let color = buffer.get_rgb(0, 0);
    /// assert_eq!(color, Rgb([0, 0, 0]));
    /// ```
    pub fn get_rgb(&self, x: u32, y: u32) -> Rgb<u8> {
        let color = self.data[(y * self.width + x) as usize];
        match color {
            CFRColor::White => Rgb([255, 255, 255]),
            CFRColor::Black => Rgb([0, 0, 0]),
            CFRColor::Blue => Rgb([0, 0, 255]),
            CFRColor::Green => Rgb([0, 255, 0]),
            CFRColor::Cyan => Rgb([0, 255, 255]),
            CFRColor::Red => Rgb([255, 0, 0]),
            CFRColor::Magenta => Rgb([255, 0, 255]),
            CFRColor::Yellow => Rgb([255, 255, 0]),
        }
    }

    #[cfg(feature = "image")]
    /// Get the color at the specified coordinates as an `Rgba<u8>` value.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    ///
    /// # Returns
    ///
    /// An `Rgba<u8>` value representing the color at the specified coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::buffer::CFRBuffer;
    /// use image::Rgba;
    ///
    /// let buffer = CFRBuffer::new(256, 256);
    /// let color = buffer.get_rgba(0, 0);
    /// assert_eq!(color, Rgba([0, 0, 0, 255]));
    /// ```
    pub fn get_rgba(&self, x: u32, y: u32) -> Rgba<u8> {
        let color = self.data[(y * self.width + x) as usize];
        match color {
            CFRColor::White => Rgba([255, 255, 255, 255]),
            CFRColor::Black => Rgba([0, 0, 0, 255]),
            CFRColor::Blue => Rgba([0, 0, 255, 255]),
            CFRColor::Green => Rgba([0, 255, 0, 255]),
            CFRColor::Cyan => Rgba([0, 255, 255, 255]),
            CFRColor::Red => Rgba([255, 0, 0, 255]),
            CFRColor::Magenta => Rgba([255, 0, 255, 255]),
            CFRColor::Yellow => Rgba([255, 255, 0, 255]),
        }
    }

    #[cfg(feature = "image")]
    /// Convert the buffer to image crate's `ImageBuffer<Rgb<u8>, Vec<u8>>` value.
    ///
    /// # Returns
    ///
    /// An `ImageBuffer<Rgb<u8>, Vec<u8>>` value representing the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::buffer::CFRBuffer;
    ///
    /// let buffer = CFRBuffer::new(256, 256);
    /// let image = buffer.to_rgb_image();
    /// image.save("test-results/image.jpg").unwrap();
    /// ```
    pub fn to_rgb_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(self.width, self.height, |x, y| self.get_rgb(x, y))
    }

    #[cfg(feature = "image")]
    /// Convert the buffer to image crate's `ImageBuffer<Rgba<u8>, Vec<u8>>` value.
    ///
    /// # Returns
    ///
    /// An `ImageBuffer<Rgba<u8>, Vec<u8>>` value representing the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cfrs::buffer::CFRBuffer;
    ///
    /// let buffer = CFRBuffer::new(256, 256);
    /// let image = buffer.to_rgba_image();
    /// image.save("test-results/image.png").unwrap();
    /// ````
    pub fn to_rgba_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_fn(self.width, self.height, |x, y| self.get_rgba(x, y))
    }
}
