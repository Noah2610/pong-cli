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
        let room_size_int = (settings.room.width, settings.room.height);
        let room_size = (room_size_int.0 as f32, room_size_int.1 as f32);
        let position_relative = settings.score.position_relative;
        let position = (
            room_size.0 * position_relative.0,
            room_size.1 * position_relative.1,
        );
        let position_round =
            (position.0.round() as u16, position.1.round() as u16);

        // Draw LEFT paddle's score
        cursor.goto(position_round.0, position_round.1).unwrap();
        print!("{}", scores.get(&Side::Left));

        // Draw RIGHT paddle's score
        cursor
            .goto(room_size_int.0 - position_round.0, position_round.1)
            .unwrap();
        print!("{}", scores.get(&Side::Right));
    }
}
