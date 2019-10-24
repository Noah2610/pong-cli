#[cfg(feature = "style")]
use std::fmt;

#[cfg(feature = "style")]
use crossterm::{style, StyledObject};

#[cfg(feature = "style")]
use crate::color::Color;
use crate::components::prelude::{Char, Drawable};

#[cfg(feature = "style")]
#[derive(Clone, Copy, Default, Deserialize)]
pub struct StyleData {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

#[cfg(feature = "style")]
impl StyleData {
    pub fn add_fg_color<T>(&mut self, color: T)
    where
        T: Into<Color>,
    {
        self.fg_color = Some(color.into());
    }

    pub fn add_bg_color<T>(&mut self, color: T)
    where
        T: Into<Color>,
    {
        self.bg_color = Some(color.into());
    }
}

#[derive(Clone, Deserialize)]
pub struct SettingsCharData {
    pub character: Option<Char>,
    #[cfg(feature = "style")]
    pub style: Option<StyleData>,
}

impl SettingsCharData {
    pub fn character(&self) -> Char {
        self.character.unwrap_or(Char::default())
    }

    #[cfg(feature = "style")]
    pub fn style(&self) -> StyleData {
        self.style.unwrap_or(StyleData::default())
    }

    #[cfg(feature = "style")]
    pub fn styled_object<T>(
        &self,
        mut styled: StyledObject<T>,
    ) -> StyledObject<T>
    where
        T: Clone + fmt::Display,
    {
        if let Some(fg_color) = self.style().fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.style().bg_color.as_ref() {
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
        drawable.add_style(self.style().clone());
        drawable
    }

    #[cfg(not(feature = "style"))]
    fn into(self) -> Drawable {
        Drawable::new(self.character())
    }
}
