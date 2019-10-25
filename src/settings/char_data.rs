use crate::components::prelude::{Char, Drawable};
#[cfg(feature = "style")]
use crate::style::prelude::*;

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
        self.style.clone().unwrap_or(StyleData::default())
    }
}

#[cfg(feature = "style")]
impl Into<StyledObject<Char>> for &SettingsCharData {
    fn into(self) -> StyledObject<Char> {
        return self.style().styled_object(self.character());
    }
}

#[cfg(feature = "style")]
impl Into<StyledObject<String>> for &SettingsCharData {
    fn into(self) -> StyledObject<String> {
        return self.style().styled_object(self.character().to_string());
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
