use std::convert::Into;

use super::component_prelude::*;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: char,
    pub fg_color:  Option<CrossColor>,
    pub bg_color:  Option<CrossColor>,
}

impl Drawable {
    pub fn new(character: char) -> Self {
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
