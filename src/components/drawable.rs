#[cfg(feature = "style")]
use std::convert::Into;
use std::fmt;

#[cfg(feature = "style")]
use crossterm::{style, StyledObject};

use super::component_prelude::*;
#[cfg(feature = "style")]
use crate::settings::prelude::StyleData;

pub type Char = char;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: Char,
    #[cfg(feature = "style")]
    pub style: StyleData,
}

impl Drawable {
    pub fn new(character: Char) -> Self {
        Self {
            character,
            #[cfg(feature = "style")]
            style: StyleData {
                fg_color: None,
                bg_color: None,
            },
        }
    }

    #[cfg(feature = "style")]
    pub fn add_style(&mut self, style: StyleData) {
        self.style = style;
    }
}

#[cfg(feature = "style")]
impl Into<StyledObject<Char>> for &Drawable {
    fn into(self) -> StyledObject<Char> {
        let mut styled = style(self.character);
        if let Some(fg_color) = &self.style.fg_color {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = &self.style.bg_color {
            styled = styled.on(bg_color.into());
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
