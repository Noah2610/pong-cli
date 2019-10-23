use super::system_prelude::*;

#[derive(Default)]
pub struct PaddleAiSystem;

impl<'a> System<'a> for PaddleAiSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadStorage<'a, PaddleAi>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Paddle>,
    );

    fn run(
        &mut self,
        (
            settings,
            paddle_ais,
            balls,
            positions,
            sizes,
            velocities,
            mut paddles,
        ): Self::SystemData,
    ) {
        for (_, paddle, paddle_position, paddle_size) in
            (&paddle_ais, &mut paddles, &positions, &sizes).join()
        {
            let follow_padding =
                paddle_size.h * settings.paddle.ai.follow_padding_percent;

            if let Some(ball_position) = (&balls, &positions, &velocities)
                .join()
                .fold(None, |nearest_opt: Option<(f32, f32)>, (_, pos, vel)| {
                    if (&paddle.side == &Side::Left && vel.x < 0.0)
                        || (&paddle.side == &Side::Right && vel.x > 0.0)
                    {
                        if let Some(nearest) = nearest_opt {
                            match &paddle.side {
                                Side::Left if pos.x < nearest.0 => {
                                    Some(pos.into())
                                }
                                Side::Right if pos.x > nearest.0 => {
                                    Some(pos.into())
                                }
                                _ => Some(nearest),
                            }
                        } else {
                            Some(pos.into())
                        }
                    } else {
                        nearest_opt
                    }
                })
            {
                if ball_position.1 < paddle_position.y - follow_padding {
                    paddle.move_up();
                } else if ball_position.1 > paddle_position.y + follow_padding {
                    paddle.move_down();
                }
            }
        }
    }
}
