use std::fmt;

#[cfg(feature = "color")]
use crate::color::CrossColor;
use crate::geo::Side;

#[derive(Default, Clone)]
pub struct Score {
    score: u32,
    #[cfg(feature = "color")]
    fg_color: Option<CrossColor>,
    #[cfg(feature = "color")]
    bg_color: Option<CrossColor>,
}

impl fmt::Display for Score {
    #[cfg(feature = "color")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crossterm::style;

        let mut styled = style(self.score);
        if let Some(fg_color) = self.fg_color {
            styled = styled.with(fg_color);
        }
        if let Some(bg_color) = self.bg_color {
            styled = styled.on(bg_color);
        }
        write!(f, "{}", styled)
    }

    #[cfg(not(feature = "color"))]
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

impl From<&Option<crate::settings::SettingsCharScore>> for Scores {
    #[cfg(feature = "color")]
    fn from(
        char_settings: &Option<crate::settings::SettingsCharScore>,
    ) -> Self {
        if let Some(char_settings) = char_settings {
            let fg_color = if let Some(fg) = char_settings.fg_color.as_ref() {
                Some(fg.into())
            } else {
                None
            };
            let bg_color = if let Some(bg) = char_settings.bg_color.as_ref() {
                Some(bg.into())
            } else {
                None
            };

            Self {
                left_paddle:  Score {
                    fg_color,
                    bg_color,
                    ..Default::default()
                },
                right_paddle: Score {
                    fg_color,
                    bg_color,
                    ..Default::default()
                },
            }
        } else {
            Self::default()
        }
    }

    #[cfg(not(feature = "color"))]
    fn from(_: &Option<crate::settings::SettingsCharScore>) -> Self {
        Self::default()
    }
}
