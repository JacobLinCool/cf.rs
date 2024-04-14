use std::fmt::Display;
use std::str::FromStr;

/// Represents the direction in which the painter moves.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CFRDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Display for CFRDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CFRDirection::Up => "Up",
            CFRDirection::UpRight => "UpRight",
            CFRDirection::Right => "Right",
            CFRDirection::DownRight => "DownRight",
            CFRDirection::Down => "Down",
            CFRDirection::DownLeft => "DownLeft",
            CFRDirection::Left => "Left",
            CFRDirection::UpLeft => "UpLeft",
        };
        write!(f, "{}", s)
    }
}

/// Converts a string to a `CFRDirection` enum variant.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns a `Result` containing the converted `CFRDirection` enum variant if successful,
/// or an error message as a `String` if the conversion fails.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use cfrs::enums::CFRDirection;
///
/// let direction = CFRDirection::from_str("up");
/// assert_eq!(direction, Ok(CFRDirection::Up));
///
/// let invalid_direction = CFRDirection::from_str("invalid");
/// assert_eq!(invalid_direction, Err("Invalid direction: invalid".to_string()));
/// ```
impl FromStr for CFRDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "up" => Ok(CFRDirection::Up),
            "upright" => Ok(CFRDirection::UpRight),
            "right" => Ok(CFRDirection::Right),
            "downright" => Ok(CFRDirection::DownRight),
            "down" => Ok(CFRDirection::Down),
            "downleft" => Ok(CFRDirection::DownLeft),
            "left" => Ok(CFRDirection::Left),
            "upleft" => Ok(CFRDirection::UpLeft),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

/// Represents the color of the painter.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CFRColor {
    White,
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Yellow,
}

impl Display for CFRColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CFRColor::White => "White",
            CFRColor::Black => "Black",
            CFRColor::Blue => "Blue",
            CFRColor::Green => "Green",
            CFRColor::Cyan => "Cyan",
            CFRColor::Red => "Red",
            CFRColor::Magenta => "Magenta",
            CFRColor::Yellow => "Yellow",
        };
        write!(f, "{}", s)
    }
}

/// Converts a string to a `CFRColor` enum variant.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns a `Result` containing the converted `CFRColor` enum variant if successful,
/// or an error message as a `String` if the conversion fails.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use cfrs::enums::CFRColor;
///
/// let color = CFRColor::from_str("white");
/// assert_eq!(color, Ok(CFRColor::White));
///
/// let invalid_color = CFRColor::from_str("invalid");
/// assert_eq!(invalid_color, Err("Invalid color: invalid".to_string()));
/// ```
impl FromStr for CFRColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "white" => Ok(CFRColor::White),
            "black" => Ok(CFRColor::Black),
            "blue" => Ok(CFRColor::Blue),
            "green" => Ok(CFRColor::Green),
            "cyan" => Ok(CFRColor::Cyan),
            "red" => Ok(CFRColor::Red),
            "magenta" => Ok(CFRColor::Magenta),
            "yellow" => Ok(CFRColor::Yellow),
            _ => Err(format!("Invalid color: {}", s)),
        }
    }
}
