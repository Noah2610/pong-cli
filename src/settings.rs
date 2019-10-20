pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsChars;
    pub use super::SettingsInput;
    pub use super::SettingsInputPaddle;
    pub use super::SettingsPaddle;
    pub use super::SettingsRoom;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub room:     SettingsRoom,
    pub paddle:   SettingsPaddle,
    pub bindings: SettingsInput,
    pub chars:    SettingsChars,
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
    pub empty:                  char,
    pub room_border_vertical:   char,
    pub room_border_horizontal: char,
    pub room_border_corner:     char,
    pub paddle:                 char,
}
