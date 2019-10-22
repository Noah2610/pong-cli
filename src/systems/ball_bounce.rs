use super::system_prelude::*;

#[derive(Default)]
pub struct BallBounceSystem;

impl<'a> System<'a> for BallBounceSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (settings, balls, colliders, mut velocities): Self::SystemData,
    ) {
        let velocity_increase = (
            settings.ball.velocity_increase.0.abs(),
            settings.ball.velocity_increase.1.abs(),
        );

        for (_, collider, velocity) in
            (&balls, &colliders, &mut velocities).join()
        {
            let mut should_increase_x_velocity = false;

            // Bounce off paddles
            let paddle_collision_info = if velocity.x < 0.0 {
                if let Some(collision_info) = collider
                    .collision_info_with(&CollisionType::Paddle(Side::Left))
                {
                    velocity.x = velocity.x.abs();
                    should_increase_x_velocity = true;
                    Some(collision_info)
                } else {
                    None
                }
            } else if velocity.x > 0.0 {
                if let Some(collision_info) = collider
                    .collision_info_with(&CollisionType::Paddle(Side::Right))
                {
                    velocity.x = -velocity.x.abs();
                    should_increase_x_velocity = true;
                    Some(collision_info)
                } else {
                    None
                }
            } else {
                None
            };

            // Bounce off vertical walls
            if velocity.y < 0.0
                && collider.in_collision_with(&CollisionType::Wall(Side::Top))
            {
                velocity.y = velocity.y.abs();
            } else if velocity.y > 0.0
                && collider
                    .in_collision_with(&CollisionType::Wall(Side::Bottom))
            {
                velocity.y = -velocity.y.abs();
            }

            // Manipulate y velocity if collision with paddle top/bottom thirds
            if let Some(collision_info) = paddle_collision_info {
                match collision_info {
                    CollisionInfo::PaddleThird(PaddleThird::Top) => {
                        velocity.y -= velocity_increase.1;
                    }
                    CollisionInfo::PaddleThird(PaddleThird::Middle) => {
                        // Do nothing...
                    }
                    CollisionInfo::PaddleThird(PaddleThird::Bottom) => {
                        velocity.y += velocity_increase.1;
                    }
                    CollisionInfo::None => panic!(
                        "CollisionType::Paddle shoudn't have \
                         CollisionInfo::None"
                    ),
                }
            }

            // Increase x velocity
            if should_increase_x_velocity {
                match velocity.x {
                    x if x > 0.0 => velocity.x += velocity_increase.0,
                    x if x < 0.0 => velocity.x -= velocity_increase.0,
                    _ => velocity.x = velocity_increase.0,
                }
            }
        }
    }
}
