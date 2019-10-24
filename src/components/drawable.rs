#[cfg(feature = "style")]
use std::convert::Into;
use std::fmt;

#[cfg(feature = "style")]
use crossterm::{style, StyledObject};

use super::component_prelude::*;

pub type Char = char;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: Char,
    #[cfg(feature = "style")]
    pub fg_color: Option<CrossColor>,
    #[cfg(feature = "style")]
    pub bg_color: Option<CrossColor>,
}

impl Drawable {
    pub fn new(character: Char) -> Self {
        Self {
            character,
            #[cfg(feature = "style")]
            fg_color: None,
            #[cfg(feature = "style")]
            bg_color: None,
        }
    }

    #[cfg(feature = "style")]
    pub fn add_fg_color<T>(&mut self, color: T)
    where
        T: Into<CrossColor>,
    {
        self.fg_color = Some(color.into());
    }

    #[cfg(feature = "style")]
    pub fn add_bg_color<T>(&mut self, color: T)
    where
        T: Into<CrossColor>,
    {
        self.bg_color = Some(color.into());
    }
}

#[cfg(feature = "style")]
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

#[cfg(feature = "style")]
impl fmt::Display for Drawable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d: StyledObject<Char> = self.into();
        write!(f, "{}", d)
    }
}

#[cfg(not(feature = "style"))]
impl fmt::Display for Drawable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
