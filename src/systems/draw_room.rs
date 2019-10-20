use super::system_prelude::*;

#[derive(Default)]
pub struct DrawRoomSystem;

impl<'a> System<'a> for DrawRoomSystem {
    type SystemData =
        (ReadExpect<'a, Settings>, ReadExpect<'a, TerminalCursor>);

    fn run(&mut self, (settings, cursor): Self::SystemData) {
        clear_room(&settings, &cursor);
        draw_border(&settings, &cursor);
        flush_stdout();
    }
}

fn clear_room(settings: &Settings, cursor: &TerminalCursor) {
    let empty_row = settings
        .chars
        .empty
        .to_string()
        .repeat(settings.room.width as usize);
    for y in 0 .. settings.room.height {
        cursor.goto(0, y).unwrap();
        print!("{}", empty_row);
    }
}

fn draw_border(settings: &Settings, cursor: &TerminalCursor) {
    let room = &settings.room;
    let chars = &settings.chars;

    let draw_vertical = |y: u16| {
        for x in 0 .. room.width {
            cursor.goto(x, y).unwrap();
            print!("{}", chars.room_border_vertical);
        }
    };
    let draw_horizontal = |x: u16| {
        for y in 0 .. room.height {
            cursor.goto(x, y).unwrap();
            print!("{}", chars.room_border_horizontal);
        }
    };

    let right = room.width - 1;
    let bottom = room.height - 1;
    draw_vertical(0);
    draw_vertical(bottom);
    draw_horizontal(0);
    draw_horizontal(right);
    // Draw corners
    cursor.goto(0, 0).unwrap();
    print!("{}", chars.room_border_corner);
    cursor.goto(right, 0).unwrap();
    print!("{}", chars.room_border_corner);
    cursor.goto(0, bottom).unwrap();
    print!("{}", chars.room_border_corner);
    cursor.goto(right, bottom).unwrap();
    print!("{}", chars.room_border_corner);
}
