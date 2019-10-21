use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPaddlesSystem;

impl<'a> System<'a> for ControlPaddlesSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadExpect<'a, InputManager>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, PaddleAi>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_manager,
            paddles,
            paddle_ais,
            mut velocities,
        ): Self::SystemData,
    ) {
        let paddle_speed = settings.paddle.speed;

        for (paddle, paddle_velocity, _) in
            (&paddles, &mut velocities, !&paddle_ais).join()
        {
            paddle_velocity.y = 0.0;

            if input_manager.is_pressed(InputKey::PaddleUp(paddle.side)) {
                paddle_velocity.y = -paddle_speed;
            }
            if input_manager.is_pressed(InputKey::PaddleDown(paddle.side)) {
                paddle_velocity.y = paddle_speed;
            }
        }
    }
}
