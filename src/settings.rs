pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsChars;
    pub use super::SettingsPaddle;
    pub use super::SettingsRoom;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub room:   SettingsRoom,
    pub paddle: SettingsPaddle,
    pub chars:  SettingsChars,
}

#[derive(Clone, Deserialize)]
pub struct SettingsRoom {
    pub width:  u16,
    pub height: u16,
}

#[derive(Clone, Deserialize)]
pub struct SettingsPaddle {
    pub size: (f32, f32),
}

#[derive(Clone, Deserialize)]
pub struct SettingsChars {
    pub empty:                  char,
    pub room_border_vertical:   char,
    pub room_border_horizontal: char,
    pub room_border_corner:     char,
    pub paddle:                 char,
}
