pub mod prelude {
    pub use crossterm::{style, Attribute, Color, StyledObject};

    pub use super::style_data::StyleData;
}

mod style_data;
