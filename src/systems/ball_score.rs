use super::system_prelude::*;

#[derive(Default)]
pub struct BallScoreSystem;

impl<'a> System<'a> for BallScoreSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Write<'a, Scores>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            mut scores,
            balls,
            positions,
            velocities,
        ): Self::SystemData,
    ) {
        let room_size =
            (settings.room.width as f32, settings.room.height as f32);

        for (ball_entity, _, ball_position, ball_velocity) in
            (&entities, &balls, &positions, &velocities).join()
        {
            let mut should_delete = false;

            // Check left side
            if ball_position.x < 0.0 && ball_velocity.x < 0.0 {
                scores.increase_for(&Side::Right);
                should_delete = true;
            } else
            // Check right side
            if ball_position.x > room_size.0 && ball_velocity.x > 0.0 {
                scores.increase_for(&Side::Left);
                should_delete = true;
            }

            if should_delete {
                entities
                    .delete(ball_entity)
                    .expect("Should delete ball entity (scored)");
            }
        }
    }
}
