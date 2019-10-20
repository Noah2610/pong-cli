pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsChars;
    pub use super::SettingsPaddles;
    pub use super::SettingsRoom;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub room:    SettingsRoom,
    pub paddles: SettingsPaddles,
    pub chars:   SettingsChars,
}

#[derive(Clone, Deserialize)]
pub struct SettingsRoom {
    pub width:  u16,
    pub height: u16,
}

#[derive(Clone, Deserialize)]
pub struct SettingsPaddles {
    pub size: (f32, f32),
}

#[derive(Clone, Deserialize)]
pub struct SettingsChars {
    pub room_border_vertical:   char,
    pub room_border_horizontal: char,
    pub room_border_corner:     char,
}
