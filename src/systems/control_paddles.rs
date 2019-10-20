use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPaddlesSystem;

impl<'a> System<'a> for ControlPaddlesSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadExpect<'a, InputManager>,
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (settings, input_manager, paddles, mut velocities): Self::SystemData,
    ) {
        let paddle_speed = settings.paddle.speed;

        for (paddle, paddle_velocity) in (&paddles, &mut velocities).join() {
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
