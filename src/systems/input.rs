use crossterm::{AsyncReader, InputEvent, KeyEvent};

use super::system_prelude::*;

#[derive(Default)]
pub struct InputSystem {
    input_reader: Option<AsyncReader>,
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        ReadExpect<'a, TerminalInput>,
        WriteExpect<'a, InputManager>,
        WriteExpect<'a, Running>,
    );

    fn run(
        &mut self,
        (terminal_input, mut input_manager, mut running): Self::SystemData,
    ) {
        input_manager.clear();

        for input_event in self
            .input_reader
            .get_or_insert_with(|| terminal_input.read_async())
        {
            if let InputEvent::Keyboard(key) = input_event {
                input_manager.input(key);
            }
        }

        // Quit
        if input_manager.is_pressed(InputKey::Quit) {
            running.0 = false;
        }
    }
}
