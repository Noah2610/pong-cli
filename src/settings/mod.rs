#[cfg(feature = "color")]
use crate::color::Color;

pub mod prelude {
    pub use super::ball_spawn_directions::{
        BallSpawnDirectionX,
        BallSpawnDirectionY,
    };
    pub use super::char_data::SettingsCharData;
    pub use super::Settings;
    pub use super::SettingsBall;
    pub use super::SettingsCharRoom;
    pub use super::SettingsCharScore;
    pub use super::SettingsChars;
    pub use super::SettingsInput;
    pub use super::SettingsInputPaddle;
    pub use super::SettingsPaddle;
    pub use super::SettingsRoom;
    pub use super::SettingsScores;
}

mod ball_spawn_directions;
mod char_data;

use ball_spawn_directions::{BallSpawnDirectionX, BallSpawnDirectionY};
use char_data::SettingsCharData;

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
    pub score:  Option<SettingsCharScore>,
    pub room:   SettingsCharRoom,
}

#[derive(Clone, Deserialize)]
pub struct SettingsCharScore {
    #[cfg(feature = "color")]
    pub fg_color: Option<Color>,
    #[cfg(feature = "color")]
    pub bg_color: Option<Color>,
}

#[derive(Clone, Deserialize)]
pub struct SettingsCharRoom {
    pub border_horizontal: SettingsCharData,
    pub border_vertical:   SettingsCharData,
    pub border_corner:     SettingsCharData,
}
