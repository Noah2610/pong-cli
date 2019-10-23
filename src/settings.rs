#[cfg(feature = "color")]
use crossterm::{style, StyledObject};

#[cfg(feature = "color")]
use crate::color::Color;
use crate::components::prelude::{Char, Drawable};

pub mod prelude {
    pub use super::BallSpawnDirectionX;
    pub use super::BallSpawnDirectionY;
    pub use super::Settings;
    pub use super::SettingsBall;
    pub use super::SettingsCharData;
    pub use super::SettingsCharRoom;
    pub use super::SettingsChars;
    pub use super::SettingsInput;
    pub use super::SettingsInputPaddle;
    pub use super::SettingsPaddle;
    pub use super::SettingsRoom;
    pub use super::SettingsScores;
}

#[derive(Clone, Deserialize)]
pub enum BallSpawnDirectionX {
    Left,
    Right,
    #[cfg(feature = "random")]
    Random,
}

impl BallSpawnDirectionX {
    pub fn number(&self, num: f32) -> f32 {
        match &self {
            Self::Left => -num,
            Self::Right => num,
            #[cfg(feature = "random")]
            Self::Random => Self::sample().number(num),
        }
    }

    #[cfg(feature = "random")]
    fn sample() -> Self {
        use rand::Rng;
        const COUNT: u8 = 2;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("Random value should never get here"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub enum BallSpawnDirectionY {
    None,
    Up,
    Down,
    #[cfg(feature = "random")]
    RandomUpOrDown,
    #[cfg(feature = "random")]
    Random,
}

impl BallSpawnDirectionY {
    pub fn number(&self, num: f32) -> f32 {
        match &self {
            Self::None => 0.0,
            Self::Up => -num,
            Self::Down => num,
            #[cfg(feature = "random")]
            Self::RandomUpOrDown => Self::sample_up_or_down().number(num),
            #[cfg(feature = "random")]
            Self::Random => Self::sample().number(num),
        }
    }

    #[cfg(feature = "random")]
    fn sample() -> Self {
        use rand::Rng;
        const COUNT: u8 = 3;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::None,
            1 => Self::Up,
            2 => Self::Down,
            _ => panic!("Random value should never get here"),
        }
    }

    #[cfg(feature = "random")]
    fn sample_up_or_down() -> Self {
        use rand::Rng;
        const COUNT: u8 = 2;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, COUNT) {
            0 => Self::Up,
            1 => Self::Down,
            _ => panic!("Random value should never get here"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub update_delay_ms: u64,
    pub room:            SettingsRoom,
    pub paddle:          SettingsPaddle,
    pub ball:            SettingsBall,
    pub score:           SettingsScores,
    pub bindings:        SettingsInput,
    pub chars:           SettingsChars,
}

#[derive(Clone, Deserialize)]
pub struct SettingsRoom {
    pub width:  u16,
    pub height: u16,
}

#[derive(Clone, Deserialize)]
pub struct SettingsPaddle {
    pub size:  (f32, f32),
    pub speed: f32,
    pub ai:    SettingsPaddleAi,
}

#[derive(Clone, Deserialize)]
pub struct SettingsPaddleAi {
    pub left:  bool,
    pub right: bool,
}

#[derive(Clone, Deserialize)]
pub struct SettingsBall {
    pub size:                  (f32, f32),
    pub velocity:              (f32, f32),
    pub velocity_increase:     (f32, f32),
    pub spawn_delay_ms:        u64,
    pub balls_amount:          u16,
    pub spawn_next_ball_in_ms: u64,
    pub spawn_direction:       (BallSpawnDirectionX, BallSpawnDirectionY),
}

#[derive(Clone, Deserialize)]
pub struct SettingsScores {
    pub position_relative: (f32, f32),
    pub reset_on_score:    bool,
}

#[derive(Clone, Deserialize)]
pub struct SettingsInput {
    pub quit:         Vec<String>,
    pub paddle_left:  SettingsInputPaddle,
    pub paddle_right: SettingsInputPaddle,
}

#[derive(Clone, Deserialize)]
pub struct SettingsInputPaddle {
    pub up:   Vec<String>,
    pub down: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct SettingsChars {
    pub empty:  SettingsCharData,
    pub paddle: SettingsCharData,
    pub ball:   SettingsCharData,
    pub room:   SettingsCharRoom,
}

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
    fn into(self) -> Drawable {
        let mut drawable = Drawable::new(self.character);
        #[cfg(feature = "color")]
        {
            if let Some(fg_color) = self.fg_color.as_ref() {
                drawable.add_fg_color(fg_color);
            }
            if let Some(bg_color) = self.bg_color.as_ref() {
                drawable.add_bg_color(bg_color);
            }
        }
        drawable
    }
}

#[derive(Clone, Deserialize)]
pub struct SettingsCharRoom {
    pub border_horizontal: SettingsCharData,
    pub border_vertical:   SettingsCharData,
    pub border_corner:     SettingsCharData,
}
