#[cfg(feature = "style")]
use std::convert::Into;
use std::fmt;

use super::component_prelude::*;

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
            style: StyleData::default(),
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
        return self.style.styled_object(self.character);
    }
}

impl fmt::Display for Drawable {
    #[cfg(feature = "style")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d: StyledObject<Char> = self.into();
        write!(f, "{}", d)
    }

    #[cfg(not(feature = "style"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
