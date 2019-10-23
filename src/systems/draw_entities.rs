use std::convert::TryInto;

use super::system_prelude::*;

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

            let pos_rounded =
                (position.x.round() as i32, position.y.round() as i32);
            let size_rounded = (size.w.round() as i32, size.h.round() as i32);
            let half_size = (size_rounded.0 / 2, size_rounded.1 / 2);

            for x in pos_rounded.0 - half_size.0 .. pos_rounded.0 + half_size.0
            {
                for y in
                    pos_rounded.1 - half_size.1 .. pos_rounded.1 + half_size.1
                {
                    if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
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
