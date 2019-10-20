use super::system_prelude::*;

#[derive(Default)]
pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (velocities, mut positions): Self::SystemData) {
        // TODO: Implement deltatime system
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}
