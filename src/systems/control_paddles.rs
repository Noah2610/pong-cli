use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPaddlesSystem;

impl<'a> System<'a> for ControlPaddlesSystem {
    type SystemData = (
        ReadExpect<'a, InputManager>,
        ReadStorage<'a, PaddleAi>,
        WriteStorage<'a, Paddle>,
    );

    fn run(
        &mut self,
        (input_manager, paddle_ais, mut paddles): Self::SystemData,
    ) {
        for (paddle, _) in (&mut paddles, !&paddle_ais).join() {
            if input_manager.is_pressed(InputKey::PaddleUp(paddle.side)) {
                paddle.move_up();
            }
            if input_manager.is_pressed(InputKey::PaddleDown(paddle.side)) {
                paddle.move_down();
            }
        }
    }
}
