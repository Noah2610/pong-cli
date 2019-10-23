use std::convert::Into;

pub use crossterm::Color as CrossColor;

/// Almost an exact copy of `crossterm::Color`.
/// The reason for this is to implement `Deserialize` for it,
/// so configuring color through the settings.ron file is more coherent.
#[derive(Clone, Deserialize)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGray,
    DarkGreen,
    DarkGrey,
    DarkMagenta,
    DarkRed,
    DarkYellow,
    Gray,
    Green,
    Grey,
    Magenta,
    Red,
    White,
    Yellow,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl Into<CrossColor> for &Color {
    #[rustfmt::skip]
    fn into(self) -> CrossColor {
        match self {
            Color::Black                      => CrossColor::Black,
            Color::Blue                       => CrossColor::Blue,
            Color::Cyan                       => CrossColor::Cyan,
            Color::DarkBlue                   => CrossColor::DarkBlue,
            Color::DarkCyan                   => CrossColor::DarkCyan,
            Color::DarkGreen                  => CrossColor::DarkGreen,
            Color::DarkGrey | Color::DarkGray => CrossColor::DarkGrey,
            Color::DarkMagenta                => CrossColor::DarkMagenta,
            Color::DarkRed                    => CrossColor::DarkRed,
            Color::DarkYellow                 => CrossColor::DarkYellow,
            Color::Green                      => CrossColor::Green,
            Color::Grey | Color::Gray         => CrossColor::Grey,
            Color::Magenta                    => CrossColor::Magenta,
            Color::Red                        => CrossColor::Red,
            Color::White                      => CrossColor::White,
            Color::Yellow                     => CrossColor::Yellow,
            Color::Rgb { r, g, b } => CrossColor::Rgb {
                r: *r,
                g: *g,
                b: *b,
            },
            Color::AnsiValue(n) => CrossColor::AnsiValue(*n),
        }
    }
}
