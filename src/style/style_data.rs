use std::fmt;

use super::prelude::*;

#[derive(Clone, Copy, Default, Deserialize)]
pub struct StyleData {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

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

    pub fn styled_object<T>(&self, value: T) -> StyledObject<T>
    where
        T: fmt::Display + Clone,
    {
        let mut styled = style(value);
        if let Some(fg_color) = self.fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.bg_color.as_ref() {
            styled = styled.on(bg_color.into());
        }
        styled
    }
}
