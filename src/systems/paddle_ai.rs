use super::system_prelude::*;

const FOLLOW_PADDING: f32 = 1.0;

#[derive(Default)]
pub struct PaddleAiSystem;

impl<'a> System<'a> for PaddleAiSystem {
    type SystemData = (
        ReadStorage<'a, PaddleAi>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Paddle>,
    );

    fn run(
        &mut self,
        (paddle_ais, balls, positions, mut paddles): Self::SystemData,
    ) {
        if let Some(ball_position) =
            (&balls, &positions).join().next().map(|(_, pos)| pos)
        {
            for (_, paddle, paddle_position) in
                (&paddle_ais, &mut paddles, &positions).join()
            {
                if ball_position.y < paddle_position.y - FOLLOW_PADDING {
                    paddle.move_up();
                } else if ball_position.y > paddle_position.y + FOLLOW_PADDING {
                    paddle.move_down();
                }
            }
        }
    }
}
