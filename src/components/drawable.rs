use std::convert::Into;

use crossterm::{style, StyledObject};

use super::component_prelude::*;

pub type Char = char;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: Char,
    pub fg_color:  Option<CrossColor>,
    pub bg_color:  Option<CrossColor>,
}

impl Drawable {
    pub fn new(character: Char) -> Self {
        Self {
            character,
            fg_color: None,
            bg_color: None,
        }
    }

    pub fn add_fg_color<T>(&mut self, color: T)
    where
        T: Into<CrossColor>,
    {
        self.fg_color = Some(color.into());
    }

    pub fn add_bg_color<T>(&mut self, color: T)
    where
        T: Into<CrossColor>,
    {
        self.bg_color = Some(color.into());
    }
}

impl Into<StyledObject<Char>> for &Drawable {
    fn into(self) -> StyledObject<Char> {
        let mut styled = style(self.character);
        if let Some(fg_color) = self.fg_color {
            styled = styled.with(fg_color);
        }
        if let Some(bg_color) = self.bg_color {
            styled = styled.on(bg_color);
        }
        return styled;
    }
}
