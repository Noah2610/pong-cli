pub mod prelude {
    pub use crossterm::{style, StyledObject};

    pub use super::color::{Color, CrossColor};
    pub use super::style_data::StyleData;
}

mod color;
mod style_data;
