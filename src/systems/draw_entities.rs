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

            // let pos_rounded =
            //     (position.x.round() as i32, position.y.round() as i32);
            // let size_rounded = (size.w.round() as i32, size.h.round() as i32);
            // let half_size_rounded = (size_rounded.0 / 2, size_rounded.1 / 2);
            // let on_lower_bound_xy = (
            //     (position.x % 1.0).round() >= 0.5,
            //     (position.y % 1.0).round() >= 0.5,
            // );
            // let has_rem_xy =
            //     ((size.w % 1.0).round() >= 0.5, (size.h % 1.0).round() >= 0.5);
            // let horz_rounded = (
            //     pos_rounded.0
            //         - half_size_rounded.0
            //         - if on_lower_bound_xy.0 && has_rem_xy.0 {
            //             1
            //         } else {
            //             0
            //         },
            //     pos_rounded.0
            //         + half_size_rounded.0
            //         + if !on_lower_bound_xy.0 && has_rem_xy.0 {
            //             1
            //         } else {
            //             0
            //         },
            // );
            // let vert_rounded = (
            //     pos_rounded.1
            //         - half_size_rounded.1
            //         - if on_lower_bound_xy.1 && has_rem_xy.1 {
            //             1
            //         } else {
            //             0
            //         },
            //     pos_rounded.1
            //         + half_size_rounded.1
            //         + if !on_lower_bound_xy.1 && has_rem_xy.1 {
            //             1
            //         } else {
            //             0
            //         },
            // );

            let size_rounded = (size.w.round() as i32, size.h.round() as i32);
            let half_size = (size.w * 0.5, size.h * 0.5);
            let half_size_rounded = (half_size.0 as i32, half_size.1 as i32);

            let halfs_x = {
                let size_is_even = (size.w % 2.0) == 0.0;
                let pos_fract = position.x.fract().abs();
                let pos_fract_rounded = pos_fract.round() as i32;
                let size_rem_rounded = half_size.0.fract().abs().round() as i32;

                if size_is_even {
                    (half_size_rounded.0, half_size_rounded.0)
                } else {
                    if pos_fract < 0.5 {
                        // (
                        //     half_size_rounded.0,
                        //     half_size_rounded.0 + size_rem_rounded,
                        // )
                        (
                            size_rounded.0 - half_size_rounded.0,
                            half_size_rounded.0,
                        )
                    } else {
                        // (
                        //     half_size_rounded.0 + size_rem_rounded,
                        //     half_size_rounded.0,
                        // )
                        (
                            half_size_rounded.0,
                            size_rounded.0 - half_size_rounded.0,
                        )
                    }
                }
            };
            let halfs_y = {
                let fract = half_size.1.fract().abs();
                if fract == 0.0 {
                    (half_size.1 as i32, half_size.1 as i32)
                } else if fract < 0.5 {
                    (half_size.1 as i32 - 1, half_size.1 as i32)
                } else {
                    (half_size.1 as i32, half_size.1 as i32 + 1)
                }
            };

            let pos_rounded =
                (position.x.round() as i32, position.y.round() as i32);
            // let pos_rounded = (position.x as i32, position.y as i32);
            let horz = (pos_rounded.0 - halfs_x.0, pos_rounded.0 + halfs_x.1);
            let vert = (pos_rounded.1 - halfs_y.0, pos_rounded.1 + halfs_y.1);

            // let horz = (position.x - half_size.0, position.x + half_size.0);
            // let vert = (position.y - half_size.1, position.y + half_size.1);
            // let horz_rounded = (horz.0.round() as i32, horz.1.round() as i32);
            // let vert_rounded = (vert.0.round() as i32, vert.1.round() as i32);

            for y in vert.0 .. vert.1 {
                if let Ok(y) = y.try_into() {
                    for x in horz.0 .. horz.1 {
                        if let Ok(x) = x.try_into() {
                            cursor.goto(x, y).unwrap();
                            print!("{}", printable);
                        }
                    }
                }
            }

            continue;

            let pos_rounded =
                (position.x.round() as i32, position.y.round() as i32);
            let half_size = (size.w * 0.5, size.h * 0.5);
            let half_size_rounded = (half_size.0 as i32, half_size.1 as i32);
            let has_rem_size =
                (half_size.0 % 1.0 >= 0.5, half_size.1 % 1.0 >= 0.5);
            // let size_rounded = (size.w.round() as i32, size.h.round() as i32);
            // let half_size = (size_rounded.0 / 2, size_rounded.1 / 2);

            for x in pos_rounded.0 - half_size_rounded.0
                .. pos_rounded.0 + half_size_rounded.0
            {
                let is_first_x = x == pos_rounded.0 - half_size_rounded.0;
                if let Ok(x) = x.try_into() {
                    for y in pos_rounded.1 - half_size_rounded.1
                        .. pos_rounded.1 + half_size_rounded.1
                    {
                        if let Ok(y) = y.try_into() {
                            // Print extra x at beginning if there is a remaining size
                            if has_rem_size.0 && is_first_x {
                                cursor.goto(x - 1, y).unwrap();
                                // print!("{}", printable);
                                print!("X");
                            }

                            if x < room_size.0 && y < room_size.1 {
                                cursor.goto(x, y).unwrap();
                                // print!("{}", printable);
                                print!("{}", if is_first_x { 1 } else { 0 });
                            }
                        }
                    }
                }
            }
        }
    }
}
