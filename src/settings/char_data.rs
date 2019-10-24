#[cfg(feature = "style")]
use std::fmt;

#[cfg(feature = "style")]
use crossterm::{style, StyledObject};

#[cfg(feature = "style")]
use crate::color::Color;
use crate::components::prelude::{Char, Drawable};

#[derive(Clone, Deserialize)]
pub struct SettingsCharData {
    pub character: Option<Char>,
    #[cfg(feature = "style")]
    pub fg_color: Option<Color>,
    #[cfg(feature = "style")]
    pub bg_color: Option<Color>,
}

impl SettingsCharData {
    pub fn character(&self) -> Char {
        self.character.unwrap_or(Char::default())
    }

    #[cfg(feature = "style")]
    fn styled_object<T>(&self, mut styled: StyledObject<T>) -> StyledObject<T>
    where
        T: Clone + fmt::Display,
    {
        if let Some(fg_color) = self.fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            styled = styled.on(bg_color.into());
        }
        styled
    }
}

#[cfg(feature = "style")]
impl Into<StyledObject<Char>> for &SettingsCharData {
    fn into(self) -> StyledObject<Char> {
        let styled = style(self.character());
        return self.styled_object(styled);
    }
}

#[cfg(feature = "style")]
impl Into<StyledObject<String>> for &SettingsCharData {
    fn into(self) -> StyledObject<String> {
        let styled = style(self.character().to_string());
        return self.styled_object(styled);
    }
}

impl Into<Drawable> for &SettingsCharData {
    #[cfg(feature = "style")]
    fn into(self) -> Drawable {
        let mut drawable = Drawable::new(self.character());
        if let Some(fg_color) = self.fg_color.as_ref() {
            drawable.add_fg_color(fg_color);
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            drawable.add_bg_color(bg_color);
        }
        drawable
    }

    #[cfg(not(feature = "style"))]
    fn into(self) -> Drawable {
        Drawable::new(self.character())
    }
}
