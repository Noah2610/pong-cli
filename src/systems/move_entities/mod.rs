mod collision;

use super::system_prelude::*;
use collision::*;

#[derive(Default)]
pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Deltatime>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Collider>,
    );

    fn run(
        &mut self,
        (
            entities,
            deltatime,
            sizes,
            velocities,
            collisions,
            mut positions,
            mut colliders,
        ): Self::SystemData,
    ) {
        let dt = deltatime.delta_seconds();

        let collision_grid =
            gen_collision_grid(&entities, &positions, &sizes, &collisions);

        for (
            entity,
            mut position,
            velocity,
            size_opt,
            collider_opt,
            collision_opt,
        ) in (
            &entities,
            &mut positions,
            &velocities,
            sizes.maybe(),
            (&mut colliders).maybe(),
            collisions.maybe(),
        )
            .join()
        {
            if let (Some(size), Some(mut collider), Some(collision)) =
                (size_opt, collider_opt, collision_opt)
            {
                run_with_collision(
                    dt,
                    &collision_grid,
                    &entity,
                    &mut position,
                    &velocity,
                    &size,
                    &mut collider,
                    &collision,
                );
            } else {
                position.x += velocity.x * dt;
                position.y += velocity.y * dt;
            }
        }
    }
}
