#[cfg(feature = "color")]
use crossterm::{style, StyledObject};

#[cfg(feature = "color")]
use crate::color::Color;
use crate::components::prelude::{Char, Drawable};

#[derive(Clone, Deserialize)]
pub struct SettingsCharData {
    pub character: Char,
    #[cfg(feature = "color")]
    pub fg_color: Option<Color>,
    #[cfg(feature = "color")]
    pub bg_color: Option<Color>,
}

#[cfg(feature = "color")]
impl Into<StyledObject<Char>> for &SettingsCharData {
    fn into(self) -> StyledObject<Char> {
        let mut styled = style(self.character);
        if let Some(fg_color) = self.fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            styled = styled.on(bg_color.into());
        }
        return styled;
    }
}

#[cfg(feature = "color")]
impl Into<StyledObject<String>> for &SettingsCharData {
    fn into(self) -> StyledObject<String> {
        let mut styled = style(self.character.to_string());
        if let Some(fg_color) = self.fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            styled = styled.on(bg_color.into());
        }
        return styled;
    }
}

impl Into<Drawable> for &SettingsCharData {
    #[cfg(feature = "color")]
    fn into(self) -> Drawable {
        let mut drawable = Drawable::new(self.character);
        if let Some(fg_color) = self.fg_color.as_ref() {
            drawable.add_fg_color(fg_color);
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            drawable.add_bg_color(bg_color);
        }
        drawable
    }

    #[cfg(not(feature = "color"))]
    fn into(self) -> Drawable {
        Drawable::new(self.character)
    }
}
