use crate::components::prelude::*;
use crate::geo::prelude::*;
use crate::settings::prelude::*;

pub fn flush_stdout() {
    use std::io::{stdout, Write};
    stdout().flush().expect("Should flush stdout");
}

pub fn position_for_paddle(
    settings: &Settings,
    paddle_side: &Side,
) -> Position {
    let paddle_x = 1.0 + settings.paddle.size.0 * 0.5;
    let paddle_y = settings.room.height as f32 * 0.5;
    Position::new(
        match paddle_side {
            Side::Left => paddle_x,
            Side::Right => settings.room.width as f32 - paddle_x,
            _ => panic!(
                "position_for_paddle only accepts a valid paddle Side, \
                 Side::Left or Side::Right"
            ),
        },
        paddle_y,
    )
}
