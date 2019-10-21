use super::system_prelude::*;

#[derive(Default)]
pub struct BallBounceSystem;

impl<'a> System<'a> for BallBounceSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (balls, colliders, mut velocities): Self::SystemData) {
        for (_, collider, velocity) in
            (&balls, &colliders, &mut velocities).join()
        {
            // Bounce off paddles
            if velocity.x < 0.0
                && collider
                    .in_collision_with(&CollisionType::Paddle(Side::Left))
            {
                velocity.x = velocity.x.abs();
            } else if velocity.x > 0.0
                && collider
                    .in_collision_with(&CollisionType::Paddle(Side::Right))
            {
                velocity.x = -velocity.x.abs();
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
        }
    }
}
