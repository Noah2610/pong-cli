use super::system_prelude::*;

#[derive(Default)]
pub struct DrawScoresSystem;

impl<'a> System<'a> for DrawScoresSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, Scores>,
        ReadExpect<'a, TerminalCursor>,
    );

    fn run(&mut self, (settings, scores, cursor): Self::SystemData) {
        let room_size =
            (settings.room.width as f32, settings.room.height as f32);
        let position_relative = settings.score.position_relative;
        let position = (
            room_size.0 * position_relative.0,
            room_size.1 * position_relative.1,
        );
        let position_round =
            (position.0.round() as u16, position.1.round() as u16);

        // Draw left paddle's score
        cursor.goto(position_round.0, position_round.1).unwrap();
        print!("{}", scores.get(&Side::Left));

        crate::flush_stdout();
    }
}
