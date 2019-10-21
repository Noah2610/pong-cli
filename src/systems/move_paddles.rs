use super::system_prelude::*;

#[derive(Default)]
pub struct MovePaddlesSystem;

impl<'a> System<'a> for MovePaddlesSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        WriteStorage<'a, Paddle>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (settings, mut paddles, mut velocities): Self::SystemData,
    ) {
        let speed = settings.paddle.speed;

        for (paddle, velocity) in (&mut paddles, &mut velocities).join() {
            match &paddle.moving {
                Some(MoveDirection::Up) => velocity.y = -speed,
                Some(MoveDirection::Down) => velocity.y = speed,
                None => velocity.y = 0.0,
            }
            if paddle.moving.is_some() {
                paddle.moving = None;
            }
        }
    }
}
