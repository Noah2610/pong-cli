use std::fmt;

use super::prelude::*;

#[derive(Clone, Copy, Default, Deserialize)]
pub struct StyleData {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    // pub attributes: Vec<Attribute>,
}

impl StyleData {
    pub fn add_fg_color(&mut self, color: Color) {
        self.fg_color = Some(color);
    }

    pub fn add_bg_color(&mut self, color: Color) {
        self.bg_color = Some(color);
    }

    pub fn styled_object<T>(&self, value: T) -> StyledObject<T>
    where
        T: fmt::Display + Clone,
    {
        let mut styled = style(value);
        if let Some(fg_color) = self.fg_color {
            styled = styled.with(fg_color);
        }
        if let Some(bg_color) = self.bg_color {
            styled = styled.on(bg_color);
        }
        styled
    }
}
