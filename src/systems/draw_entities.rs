use std::convert::TryInto;

use super::system_prelude::*;
use crate::round_down::RoundDown;

#[derive(Default)]
pub struct DrawEntitiesSystem;

impl<'a> System<'a> for DrawEntitiesSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadExpect<'a, TerminalCursor>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Drawable>,
    );

    fn run(
        &mut self,
        (settings, cursor, positions, sizes, drawables): Self::SystemData,
    ) {
        let room_size = (settings.room.width, settings.room.height);

        for (position, size, drawable) in
            (&positions, &sizes, &drawables).join()
        {
            let printable = drawable.to_string();

            let half_size = (size.w * 0.5, size.h * 0.5);
            let horz = (
                (position.x - half_size.0).round_lower(),
                (position.x + half_size.0).round_higher(),
            );
            let vert = (
                (position.y - half_size.1).round_lower(),
                (position.y + half_size.1).round_higher(),
            );

            for y in vert.0 .. vert.1 {
                if let Ok(y) = y.try_into() {
                    for x in horz.0 .. horz.1 as i32 {
                        if let Ok(x) = x.try_into() {
                            if x < room_size.0 && y < room_size.1 {
                                cursor.goto(x, y).unwrap();
                                print!("{}", printable);
                            }
                        }
                    }
                }
            }
        }
    }
}
