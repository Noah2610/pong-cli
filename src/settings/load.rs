use std::fmt::Display;
use std::fs::File;
use std::io::stdin;
use std::path::PathBuf;

#[cfg(feature = "style")]
use crossterm::{style, Attribute, Color};

use super::Settings;

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

fn static_settings() -> Settings {
    ron::de::from_str(DEFAULT_SETTINGS_RON)
        .expect("Failed parsing static settings string (should never happen)")
}

fn eprint_and_read_line<S>(msg: S)
where
    S: Display,
{
    eprintln!("{}\n[ENTER TO CONTINUE]", msg);
    stdin().read_line(&mut String::new()).unwrap();
}
