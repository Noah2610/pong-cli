use crossterm::{AsyncReader, InputEvent, KeyEvent};

use super::system_prelude::*;

#[derive(Default)]
pub struct InputSystem {
    input_reader: Option<AsyncReader>,
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadExpect<'a, TerminalInput>, WriteExpect<'a, Running>);

    fn run(&mut self, (terminal_input, mut running): Self::SystemData) {
        for input_event in self
            .input_reader
            .get_or_insert_with(|| terminal_input.read_async())
        {
            match input_event {
                InputEvent::Keyboard(key_event) => match key_event {
                    KeyEvent::Char('q')
                    | KeyEvent::Char('Q')
                    | KeyEvent::Ctrl('c') => running.0 = false,
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
