use crossterm::StyledObject;

use super::system_prelude::*;

#[derive(Default)]
pub struct DrawRoomSystem;

impl<'a> System<'a> for DrawRoomSystem {
    type SystemData =
        (ReadExpect<'a, Settings>, ReadExpect<'a, TerminalCursor>);

    fn run(&mut self, (settings, cursor): Self::SystemData) {
        clear_room(&settings, &cursor);
        draw_border(&settings, &cursor);
    }
}

fn clear_room(settings: &Settings, cursor: &TerminalCursor) {
    let empty_char = &settings.chars.empty;
    let mut printable: StyledObject<String> = empty_char.into();
    printable.content = printable.content.repeat(settings.room.width as usize);
    for y in 0 .. settings.room.height {
        cursor.goto(0, y).unwrap();
        print!("{}", printable);
    }
}

fn draw_border(settings: &Settings, cursor: &TerminalCursor) {
    let room = &settings.room;
    let chars = &settings.chars;

    let draw_horizontal = |y: u16| {
        let border_char = &chars.room.border_horizontal;
        let mut printable: StyledObject<String> = border_char.into();
        printable.content = printable.content.repeat(room.width as usize);
        cursor.goto(0, y).unwrap();
        print!("{}", printable);
    };
    let draw_vertical = |x: u16| {
        let border_char = &chars.room.border_vertical;
        let printable: StyledObject<Char> = border_char.into();
        for y in 0 .. room.height {
            cursor.goto(x, y).unwrap();
            print!("{}", printable);
        }
    };

    let right = room.width - 1;
    let bottom = room.height - 1;
    draw_horizontal(0);
    draw_horizontal(bottom);
    draw_vertical(0);
    draw_vertical(right);
    // Draw corners
    let corner_printable: StyledObject<Char> =
        (&chars.room.border_corner).into();
    cursor.goto(0, 0).unwrap();
    print!("{}", corner_printable);
    cursor.goto(right, 0).unwrap();
    print!("{}", corner_printable);
    cursor.goto(0, bottom).unwrap();
    print!("{}", corner_printable);
    cursor.goto(right, bottom).unwrap();
    print!("{}", corner_printable);
}
