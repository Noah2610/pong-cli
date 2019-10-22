use super::system_prelude::*;

#[derive(Default)]
pub struct ResetSystem;

impl<'a> System<'a> for ResetSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Write<'a, ShouldReset>,
        Write<'a, ShouldResetBallSpawns>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            mut should_reset,
            mut should_reset_ball_spawns,
            paddles,
            balls,
            mut positions,
            mut velocities,
        ): Self::SystemData,
    ) {
        if !should_reset.0 {
            return;
        }

        // Reset velocities for everything
        for velocity in (&mut velocities).join() {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        // Reset paddle positions
        for (paddle, paddle_position) in (&paddles, &mut positions).join() {
            *paddle_position = position_for_paddle(&settings, &paddle.side);
        }

        // Remove all balls
        for (ball_entity, _) in (&entities, &balls).join() {
            entities.delete(ball_entity).expect("Should delete Ball");
        }

        // Reset reset-resources
        should_reset.0 = false;
        should_reset_ball_spawns.0 = true;
    }
}
