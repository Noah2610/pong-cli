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
        let velocity_increase = settings.ball.velocity_increase;

        for (_, collider, velocity) in
            (&balls, &colliders, &mut velocities).join()
        {
            let mut should_increase_velocity = false;

            // Bounce off paddles
            if velocity.x < 0.0
                && collider
                    .in_collision_with(&CollisionType::Paddle(Side::Left))
            {
                velocity.x = velocity.x.abs();
                should_increase_velocity = true;
            } else if velocity.x > 0.0
                && collider
                    .in_collision_with(&CollisionType::Paddle(Side::Right))
            {
                velocity.x = -velocity.x.abs();
                should_increase_velocity = true;
            }

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

            // Increase velocity
            if should_increase_velocity {
                match velocity.x {
                    x if x > 0.0 => velocity.x += velocity_increase.0,
                    x if x < 0.0 => velocity.x -= velocity_increase.0,
                    _ => velocity.x = velocity_increase.0,
                }
                match velocity.y {
                    y if y > 0.0 => velocity.y += velocity_increase.1,
                    y if y < 0.0 => velocity.y -= velocity_increase.1,
                    _ => velocity.y = velocity_increase.1,
                }
            }
        }
    }
}
