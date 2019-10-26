const DEFAULT_SETTINGS_RON: &str = include_str!("../../settings.ron");
const HOME_DIR_REPLACE: &str = "<HOME>";
const CONFIG_DIR_REPLACE: &str = "<CONFIG>";
const SETTINGS_FILE_LOCATIONS: [&str; 4] = [
    "./settings.ron",
    "./pong-cli.ron",
    "<HOME>/.pong-cli.ron",
    "<CONFIG>/pong-cli/settings.ron",
];

pub fn load_settings() -> Settings {
    #[cfg(feature = "style")]
    use crossterm::{style, Attribute, Color};
    use std::fmt::Display;
    use std::fs::File;
    use std::io::stdin;
    use std::path::PathBuf;

    fn static_settings() -> Settings {
        ron::de::from_str(DEFAULT_SETTINGS_RON).expect(
            "Failed parsing static settings string (should never happen)",
        )
    }

    fn eprint_and_read_line<S>(msg: S)
    where
        S: Display,
    {
        eprintln!("{}\n[ENTER TO CONTINUE]", msg);
        stdin().read_line(&mut String::new()).unwrap();
    }

    // First try to find a settings.ron file
    let home_dir = dirs::home_dir();
    let config_dir = dirs::config_dir();
    let path_opt = SETTINGS_FILE_LOCATIONS.into_iter().find_map(|path_str| {
        let mut path_s = path_str.to_string();
        if let Some(Some(home)) = home_dir.as_ref().map(|d| d.to_str()) {
            path_s = path_s.replace(HOME_DIR_REPLACE, home);
        }
        if let Some(Some(config)) = config_dir.as_ref().map(|d| d.to_str()) {
            path_s = path_s.replace(CONFIG_DIR_REPLACE, config);
        }
        let path = PathBuf::from(path_s);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    });

    if let Some(path) = path_opt.as_ref() {
        match File::open(path) {
            Ok(file) => match ron::de::from_reader(file) {
                Ok(settings) => settings,
                Err(e) => {
                    #[cfg(feature = "style")]
                    let errmsg = format!(
                        "{}\n{}\n{:#?}\nUsing default settings",
                        style("ERROR: Couldn't parse settings file:")
                            .with(Color::Red)
                            .attr(Attribute::Bold),
                        style(&path.display()).with(Color::Blue),
                        e,
                    );
                    #[cfg(not(feature = "style"))]
                    let errmsg = format!(
                        "ERROR: Couldn't parse settings file: \
                         reading:\n{}\n{:#?}\nUsing default settings",
                        path, e,
                    );
                    eprint_and_read_line(errmsg);
                    static_settings()
                }
            },
            Err(e) => {
                #[cfg(feature = "style")]
                let errmsg = format!(
                    "{}\n{}\n{:#?}\nUsing default settings",
                    style("ERROR: Couldn't open settings file for reading:")
                        .with(Color::Red)
                        .attr(Attribute::Bold),
                    style(&path.display()).with(Color::Blue),
                    e,
                );
                #[cfg(not(feature = "style"))]
                let errmsg = format!(
                    "ERROR: Couldn't open settings file for \
                     reading:\n{}\n{:#?}\nUsing default settings",
                    path, e,
                );
                eprint_and_read_line(errmsg);
                static_settings()
            }
        }
    } else {
        static_settings()
    }
}

pub mod prelude {
    pub use super::ball_spawn_directions::{
        BallSpawnDirectionX,
        BallSpawnDirectionY,
    };
    pub use super::char_data::SettingsCharData;
    pub use super::load_settings;
    pub use super::Settings;
    pub use super::SettingsBall;
    pub use super::SettingsCharRoom;
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
    pub left:                   bool,
    pub right:                  bool,
    pub follow_padding_percent: f32,
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
    pub score:  SettingsCharData,
    pub room:   SettingsCharRoom,
}

#[derive(Clone, Deserialize)]
pub struct SettingsCharRoom {
    pub border_horizontal: SettingsCharData,
    pub border_vertical:   SettingsCharData,
    pub border_corner:     SettingsCharData,
}
