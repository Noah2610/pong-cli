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
        for (_, paddle, paddle_position) in
            (&paddle_ais, &mut paddles, &positions).join()
        {
            if let Some(ball_position) = (&balls, &positions).join().fold(
                None,
                |nearest_opt: Option<(f32, f32)>, (ball, pos)| {
                    if let Some(nearest) = nearest_opt {
                        match &paddle.side {
                            Side::Left if pos.x < nearest.0 => Some(pos.into()),
                            Side::Right if pos.x > nearest.0 => {
                                Some(pos.into())
                            }
                            _ => Some(nearest),
                        }
                    } else {
                        Some(pos.into())
                    }
                },
            ) {
                if ball_position.1 < paddle_position.y - FOLLOW_PADDING {
                    paddle.move_up();
                } else if ball_position.1 > paddle_position.y + FOLLOW_PADDING {
                    paddle.move_down();
                }
            }
        }
    }
}
