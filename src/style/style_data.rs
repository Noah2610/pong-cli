use std::fmt;

use crossterm::ObjectStyle;

use super::prelude::*;

// TODO:
// Look into using `crossterm::ObjectStyle` instead of this custom struct.
// Both structs do basically the same, only the `StyleData` implements `Deserialize`.
#[derive(Clone, Default, Deserialize)]
pub struct StyleData {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub attrs:    Option<Vec<Attribute>>,
}

impl StyleData {
    pub fn fg(&mut self, color: Color) {
        self.fg_color = Some(color);
    }

    pub fn bg(&mut self, color: Color) {
        self.bg_color = Some(color);
    }

    pub fn add_attr(&mut self, attr: Attribute) {
        let attrs = self.attrs.get_or_insert(Vec::new());
        if !attrs.contains(&attr) {
            attrs.push(attr);
        }
    }

    pub fn styled_object<T>(&self, value: T) -> StyledObject<T>
    where
        T: fmt::Display + Clone,
    {
        StyledObject {
            object_style: self.into(),
            content:      value,
        }
    }
}

impl Into<ObjectStyle> for &StyleData {
    fn into(self) -> ObjectStyle {
        ObjectStyle {
            fg_color: self.fg_color,
            bg_color: self.bg_color,
            attrs:    self.attrs.clone().unwrap_or(Vec::new()),
        }
    }
}
