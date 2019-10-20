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
            if collider
                .in_collision_with(&CollisionType::Paddle(PaddleSide::Left))
            {
                velocity.x = velocity.x.abs();
            } else if collider
                .in_collision_with(&CollisionType::Paddle(PaddleSide::Right))
            {
                velocity.x = -velocity.x.abs();
            }
        }
    }
}
