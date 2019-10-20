use super::system_prelude::*;

const FOLLOW_PADDING: f32 = 1.0;

#[derive(Default)]
pub struct PaddleAiSystem;

impl<'a> System<'a> for PaddleAiSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadStorage<'a, PaddleAi>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            settings,
            paddle_ais,
            balls,
            positions,
            mut velocities,
        ): Self::SystemData,
    ) {
        let paddle_speed = settings.paddle.speed;

        if let Some(ball_position) =
            (&balls, &positions).join().next().map(|(_, pos)| pos)
        {
            for (_, paddle_position, paddle_velocity) in
                (&paddle_ais, &positions, &mut velocities).join()
            {
                if ball_position.y < paddle_position.y - FOLLOW_PADDING {
                    paddle_velocity.y = -paddle_speed;
                } else if ball_position.y > paddle_position.y + FOLLOW_PADDING {
                    paddle_velocity.y = paddle_speed;
                }
            }
        }
    }
}
