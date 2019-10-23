use std::convert::Into;

pub use crossterm::Color as CrossColor;

/// Almost an exact copy of `crossterm::Color`.
/// The reason for this is to implement `Deserialize` for it,
/// so configuring color through the settings.ron file is more coherent.
#[derive(Clone, Deserialize)]
pub enum Color {
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl Into<CrossColor> for &Color {
    fn into(self) -> CrossColor {
        match self {
            Color::Black => CrossColor::Black,
            Color::DarkGrey => CrossColor::DarkGrey,
            Color::Red => CrossColor::Red,
            Color::DarkRed => CrossColor::DarkRed,
            Color::Green => CrossColor::Green,
            Color::DarkGreen => CrossColor::DarkGreen,
            Color::Yellow => CrossColor::Yellow,
            Color::DarkYellow => CrossColor::DarkYellow,
            Color::Blue => CrossColor::Blue,
            Color::DarkBlue => CrossColor::DarkBlue,
            Color::Magenta => CrossColor::Magenta,
            Color::DarkMagenta => CrossColor::DarkMagenta,
            Color::Cyan => CrossColor::Cyan,
            Color::DarkCyan => CrossColor::DarkCyan,
            Color::White => CrossColor::White,
            Color::Grey => CrossColor::Grey,
            Color::Rgb { r, g, b } => CrossColor::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
            Color::AnsiValue(n) => CrossColor::AnsiValue(*n),
        }
    }
}
