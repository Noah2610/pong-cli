use std::fmt;

use crate::geo::Side;
use crate::settings::prelude::*;
#[cfg(feature = "style")]
use crate::style::prelude::*;

#[derive(Default, Clone)]
pub struct Score {
    score: u32,
    #[cfg(feature = "style")]
    style: StyleData,
}

impl fmt::Display for Score {
    #[cfg(feature = "style")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut styled = style(self.score);
        if let Some(fg_color) = self.style.fg_color.as_ref() {
            styled = styled.with(fg_color.into());
        }
        if let Some(bg_color) = self.style.bg_color.as_ref() {
            styled = styled.on(bg_color.into());
        }
        write!(f, "{}", styled)
    }

    #[cfg(not(feature = "style"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.score)
    }
}

#[derive(Default, Clone)]
pub struct Scores {
    left_paddle:  Score,
    right_paddle: Score,
}

impl Scores {
    pub fn increase_for(&mut self, paddle_side: &Side) {
        self.get_mut(paddle_side).score += 1;
    }

    pub fn get(&self, paddle_side: &Side) -> &Score {
        match paddle_side {
            Side::Left => &self.left_paddle,
            Side::Right => &self.right_paddle,
            Side::Top | Side::Bottom => {
                panic!("Paddle sides can only be Left or Right (Scores::get)")
            }
        }
    }

    fn get_mut(&mut self, paddle_side: &Side) -> &mut Score {
        match paddle_side {
            Side::Left => &mut self.left_paddle,
            Side::Right => &mut self.right_paddle,
            Side::Top | Side::Bottom => {
                panic!("Paddle sides can only be Left or Right (Scores::get)")
            }
        }
    }
}

impl From<&SettingsCharData> for Scores {
    #[cfg(feature = "style")]
    fn from(char_data: &SettingsCharData) -> Self {
        Self {
            left_paddle:  Score {
                style: char_data.style(),
                ..Default::default()
            },
            right_paddle: Score {
                style: char_data.style(),
                ..Default::default()
            },
        }
    }

    #[cfg(not(feature = "style"))]
    fn from(_: &SettingsCharData) -> Self {
        Self::default()
    }
}
