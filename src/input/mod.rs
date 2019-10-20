use crossterm::KeyEvent;

use crate::components::prelude::PaddleSide;
use crate::settings::SettingsInput;

pub mod prelude {
    pub use super::InputKey;
    pub use super::InputManager;
}

pub struct InputManager {
    settings: SettingsInput,
    pressed:  Vec<InputKey>,
}

impl InputManager {
    pub fn new(settings: SettingsInput) -> Self {
        Self {
            settings,
            pressed: Vec::new(),
        }
    }

    pub fn input(&mut self, key_event: KeyEvent) {
        let key_string = key_event_to_string(key_event);

        // Check Quit
        if self.settings.quit.contains(&key_string) {
            self.insert_pressed(InputKey::Quit);
        }
        // Check left paddle's binds
        if self.settings.paddle_left.up.contains(&key_string) {
            self.insert_pressed(InputKey::PaddleUp(PaddleSide::Left));
        }
        if self.settings.paddle_left.down.contains(&key_string) {
            self.insert_pressed(InputKey::PaddleDown(PaddleSide::Left));
        }
        // Check right paddle's binds
        if self.settings.paddle_right.up.contains(&key_string) {
            self.insert_pressed(InputKey::PaddleUp(PaddleSide::Right));
        }
        if self.settings.paddle_right.down.contains(&key_string) {
            self.insert_pressed(InputKey::PaddleDown(PaddleSide::Right));
        }
    }

    fn insert_pressed(&mut self, key: InputKey) {
        if !self.pressed.contains(&key) {
            self.pressed.push(key);
        }
    }
}

#[derive(PartialEq)]
pub enum InputKey {
    Quit,
    PaddleUp(PaddleSide),
    PaddleDown(PaddleSide),
}

#[rustfmt::skip]
fn key_event_to_string(key_event: KeyEvent) -> String {
    match key_event {
        KeyEvent::Backspace  => "Backspace".to_string(),
        KeyEvent::Enter      => "Enter".to_string(),
        KeyEvent::Left       => "Left".to_string(),
        KeyEvent::Right      => "Right".to_string(),
        KeyEvent::Up         => "Up".to_string(),
        KeyEvent::Down       => "Down".to_string(),
        KeyEvent::Home       => "Home".to_string(),
        KeyEvent::End        => "End".to_string(),
        KeyEvent::PageUp     => "PageUp".to_string(),
        KeyEvent::PageDown   => "PageDown".to_string(),
        KeyEvent::Tab        => "Tab".to_string(),
        KeyEvent::BackTab    => "BackTab".to_string(),
        KeyEvent::Delete     => "Delete".to_string(),
        KeyEvent::Insert     => "Insert".to_string(),
        KeyEvent::F(n)       => format!("F{}", n),
        KeyEvent::Char(c)    => c.to_string(),
        KeyEvent::Alt(c)     => format!("Alt+{}", c),
        KeyEvent::Ctrl(c)    => format!("Ctrl+{}", c),
        KeyEvent::Null       => "Null".to_string(),
        KeyEvent::Esc        => "Esc".to_string(),
        KeyEvent::CtrlUp     => "CtrlUp".to_string(),
        KeyEvent::CtrlDown   => "CtrlDown".to_string(),
        KeyEvent::CtrlRight  => "CtrlRight".to_string(),
        KeyEvent::CtrlLeft   => "CtrlLeft".to_string(),
        KeyEvent::ShiftUp    => "ShiftUp".to_string(),
        KeyEvent::ShiftDown  => "ShiftDown".to_string(),
        KeyEvent::ShiftRight => "ShiftRight".to_string(),
        KeyEvent::ShiftLeft  => "ShiftLeft".to_string(),
    }
}
