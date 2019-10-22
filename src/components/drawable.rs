use crossterm::Color;

use super::component_prelude::*;

const DEFAULT_COLOR: Color = Color::White;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: char,
    pub fg_color:  Option<Color>,
    pub bg_color:  Option<Color>,
}

impl Drawable {
    pub fn new(character: char) -> Self {
        Self {
            character,
            fg_color: None,
            bg_color: None,
        }
    }

    pub fn add_fg_color(&mut self, color: Color) {
        self.fg_color = Some(color);
    }

    pub fn add_bg_color(&mut self, color: Color) {
        self.bg_color = Some(color);
    }
}
