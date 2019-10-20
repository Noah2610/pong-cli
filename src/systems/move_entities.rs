use super::system_prelude::*;

/// Just a plain `Axis` enum with `X` and `Y` variants.
#[derive(Clone, Debug, PartialEq)]
enum Axis {
    X,
    Y,
}

impl Axis {
    /// This iterator-like method simply executes the passed closure with
    /// the `X` variant and then for the `Y` variant.
    pub fn for_each<C>(mut iterate: C)
    where
        C: FnMut(Self),
    {
        iterate(Axis::X);
        iterate(Axis::Y);
    }

    /// Returns `true` if this is the `X` variant.
    pub fn is_x(&self) -> bool {
        Axis::X == *self
    }

    /// Returns `true` if this is the `Y` variant.
    pub fn is_y(&self) -> bool {
        Axis::Y == *self
    }
}

#[derive(Default)]
struct CollisionGrid {
    rects: Vec<CollisionRect>,
}

impl CollisionGrid {
    pub fn push(&mut self, rect: CollisionRect) {
        self.rects.push(rect);
    }

    pub fn collisions_with(
        &self,
        target: &CollisionRect,
    ) -> Vec<CollisionType> {
        self.rects
            .iter()
            .filter_map(|rect| {
                if rect.id != target.id
                    && Self::do_rects_intersect(rect, target)
                {
                    Some(rect.collision_type.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns `true` if the two passed `Rect`s intersect with each other.
    #[rustfmt::skip]
    fn do_rects_intersect(rect_one: &CollisionRect, rect_two: &CollisionRect) -> bool {
        (
            (
                   rect_one.left >= rect_two.left
                && rect_one.left <  rect_two.right
            ) || (
                   rect_one.left  <= rect_two.left
                && rect_one.right >  rect_two.left
            )
        ) && (
            (
                   rect_one.top >= rect_two.top
                && rect_one.top <  rect_two.bottom
            ) || (
                   rect_one.top    <= rect_two.top
                && rect_one.bottom >  rect_two.top
            )
        )
    }
}

struct CollisionRect {
    pub id:             Index,
    pub collision_type: CollisionType,
    pub top:            f32,
    pub bottom:         f32,
    pub left:           f32,
    pub right:          f32,
}

fn new_collision_rect_and_position(
    id: Index,
    collision_type: &CollisionType,
    position: &Position,
    size: &Size,
    axis: &Axis,
    step: f32,
) -> (CollisionRect, (f32, f32)) {
    // Calculate new position
    let new_position = (
        position.x + if axis.is_x() { step } else { 0.0 },
        position.y + if axis.is_y() { step } else { 0.0 },
    );
    let half_size = (size.w * 0.5, size.h * 0.5);
    (
        CollisionRect {
            id:             id,
            collision_type: collision_type.clone(),
            top:            new_position.1 - half_size.1,
            bottom:         new_position.1 + half_size.1,
            left:           new_position.0 - half_size.0,
            right:          new_position.0 + half_size.0,
        },
        new_position,
    )
}

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

        let mut collision_grid = CollisionGrid::default();
        for (entity, position, size, collision) in
            (&entities, &positions, &sizes, &collisions).join()
        {
            let half_size = (size.w * 0.5, size.h * 0.5);
            collision_grid.push(CollisionRect {
                id:             entity.id(),
                collision_type: collision.collision_type.clone(),
                top:            position.y - half_size.1,
                bottom:         position.y + half_size.1,
                left:           position.x - half_size.0,
                right:          position.x + half_size.0,
            });
        }

        for (
            entity,
            position,
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
            let entity_id = entity.id();

            if let (Some(size), Some(collider), Some(collision)) =
                (size_opt, collider_opt, collision_opt)
            {
                collider.clear();

                Axis::for_each(|axis| {
                    let vel = match axis {
                        Axis::X => velocity.x * dt,
                        Axis::Y => velocity.y * dt,
                    };
                    let abs = vel.abs() as usize;
                    let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
                    let rem = vel % 1.0;

                    // Try to move by one absolute unit
                    for _ in 0 .. abs {
                        let (collision_rect, new_position) =
                            new_collision_rect_and_position(
                                entity_id,
                                &collision.collision_type,
                                position,
                                size,
                                &axis,
                                sign,
                            );
                        // Check for collision in newly calculated position
                        for collision_type in
                            collision_grid.collisions_with(&collision_rect)
                        {
                            collider.push(collision_type);
                        }
                        position.x = new_position.0;
                        position.y = new_position.1;
                    }
                    // Try to move by the floating point remainder
                    // Calculate new position
                    let (collision_rect, new_position) =
                        new_collision_rect_and_position(
                            entity_id,
                            &collision.collision_type,
                            position,
                            size,
                            &axis,
                            rem,
                        );
                    // Check for collision in newly calculated position
                    for collision_type in
                        collision_grid.collisions_with(&collision_rect)
                    {
                        collider.push(collision_type);
                    }
                    position.x = new_position.0;
                    position.y = new_position.1;
                });
            } else {
                position.x += velocity.x * dt;
                position.y += velocity.y * dt;
            }
        }
    }
}
