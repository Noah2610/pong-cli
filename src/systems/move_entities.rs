use super::system_prelude::*;

#[derive(Default)]
pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (
        Read<'a, Deltatime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(
        &mut self,
        (deltatime, velocities, mut positions): Self::SystemData,
    ) {
        let dt = deltatime.delta_seconds();

        // TODO: Implement deltatime system
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x * dt;
            position.y += velocity.y * dt;
        }
    }
}
