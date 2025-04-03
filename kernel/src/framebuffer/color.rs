use alloc::format;
use alloc::string::String;
use embedded_graphics::pixelcolor::Rgb888;

/// Color enumeration for text and UI elements
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ConsoleColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl ConsoleColor {
    /// Convert ConsoleColor to Rgb888
    pub fn to_rgb888(&self) -> Rgb888 {
        match self {
            ConsoleColor::Black => Rgb888::new(0, 0, 0),
            ConsoleColor::Red => Rgb888::new(170, 0, 0),
            ConsoleColor::Green => Rgb888::new(0, 170, 0),
            ConsoleColor::Yellow => Rgb888::new(170, 85, 0),
            ConsoleColor::Blue => Rgb888::new(0, 0, 170),
            ConsoleColor::Magenta => Rgb888::new(170, 0, 170),
            ConsoleColor::Cyan => Rgb888::new(0, 170, 170),
            ConsoleColor::White => Rgb888::new(170, 170, 170),
            ConsoleColor::BrightBlack => Rgb888::new(85, 85, 85),
            ConsoleColor::BrightRed => Rgb888::new(255, 85, 85),
            ConsoleColor::BrightGreen => Rgb888::new(85, 255, 85),
            ConsoleColor::BrightYellow => Rgb888::new(255, 255, 85),
            ConsoleColor::BrightBlue => Rgb888::new(85, 85, 255),
            ConsoleColor::BrightMagenta => Rgb888::new(255, 85, 255),
            ConsoleColor::BrightCyan => Rgb888::new(85, 255, 255),
            ConsoleColor::BrightWhite => Rgb888::new(255, 255, 255),
        }
    }
}

/// Helper to map ANSI color codes to ConsoleColor.
/// The `is_background` flag indicates if the code applies to the background.
pub fn ansi_color_to_console_color(code: u8, is_background: bool) -> Option<ConsoleColor> {
    match (code, is_background) {
        (30, false) | (40, true) => Some(ConsoleColor::Black),
        (31, false) | (41, true) => Some(ConsoleColor::Red),
        (32, false) | (42, true) => Some(ConsoleColor::Green),
        (33, false) | (43, true) => Some(ConsoleColor::Yellow),
        (34, false) | (44, true) => Some(ConsoleColor::Blue),
        (35, false) | (45, true) => Some(ConsoleColor::Magenta),
        (36, false) | (46, true) => Some(ConsoleColor::Cyan),
        (37, false) | (47, true) => Some(ConsoleColor::White),
        (90, false) | (100, true) => Some(ConsoleColor::BrightBlack),
        (91, false) | (101, true) => Some(ConsoleColor::BrightRed),
        (92, false) | (102, true) => Some(ConsoleColor::BrightGreen),
        (93, false) | (103, true) => Some(ConsoleColor::BrightYellow),
        (94, false) | (104, true) => Some(ConsoleColor::BrightBlue),
        (95, false) | (105, true) => Some(ConsoleColor::BrightMagenta),
        (96, false) | (106, true) => Some(ConsoleColor::BrightCyan),
        (97, false) | (107, true) => Some(ConsoleColor::BrightWhite),
        _ => None,
    }
}

pub trait ColoredWriting {
    fn fg(&self, _: ConsoleColor) -> String;
}

impl ColoredWriting for str {
    fn fg(&self, color: ConsoleColor) -> String {
        let fmt_label = match color {
            ConsoleColor::Black => "\x1b[30m",
            ConsoleColor::Red => "\x1b[31m",
            ConsoleColor::Green => "\x1b[32m",
            ConsoleColor::Yellow => "\x1b[33m",
            ConsoleColor::Blue => "\x1b[34m",
            ConsoleColor::Magenta => "\x1b[35m",
            ConsoleColor::Cyan => "\x1b[36m",
            ConsoleColor::White => "\x1b[37m",
            ConsoleColor::BrightBlack => "\x1b[90m",
            ConsoleColor::BrightRed => "\x1b[91m",
            ConsoleColor::BrightGreen => "\x1b[92m",
            ConsoleColor::BrightYellow => "\x1b[93m",
            ConsoleColor::BrightBlue => "\x1b[94m",
            ConsoleColor::BrightMagenta => "\x1b[95m",
            ConsoleColor::BrightCyan => "\x1b[96m",
            ConsoleColor::BrightWhite => "\x1b[97m",
        };
        let str = format!("{}{}\x1b[0m", fmt_label, self);
        str
    }
}
