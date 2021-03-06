//! Most of this module is copied from my deathframe crate.

use super::*;

/// Just a plain `Axis` enum with `X` and `Y` variants.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Axis {
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
pub(super) struct CollisionGrid {
    rects: Vec<CollisionRect>,
}

impl CollisionGrid {
    pub fn push(&mut self, rect: CollisionRect) {
        self.rects.push(rect);
    }

    pub fn collisions_with(
        &self,
        target: &CollisionRect,
    ) -> Vec<CollisionData> {
        self.rects
            .iter()
            .filter_map(|rect| {
                if rect.id != target.id
                    && Self::do_rects_intersect(&rect.rect, &target.rect)
                {
                    let mut collision_data =
                        CollisionData::new(rect.collision_type.clone());
                    if let CollisionType::Paddle(_) =
                        collision_data.collision_type
                    {
                        let target_center_y = target.rect.top
                            + (target.rect.bottom - target.rect.top) * 0.5;
                        let rect_third =
                            (rect.rect.bottom - rect.rect.top) / 3.0;
                        let first_separator = rect.rect.top + rect_third;
                        let second_separator = rect.rect.bottom - rect_third;
                        collision_data.add_info(CollisionInfo::PaddleThird(
                            if (.. first_separator).contains(&target_center_y) {
                                PaddleThird::Top
                            } else if (first_separator .. second_separator)
                                .contains(&target_center_y)
                            {
                                PaddleThird::Middle
                            } else if (second_separator ..)
                                .contains(&target_center_y)
                            {
                                PaddleThird::Bottom
                            } else {
                                panic!(
                                    "Ball's center should be in a third of a \
                                     Paddle"
                                )
                            },
                        ));
                    }
                    Some(collision_data)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns `true` if the two passed `Rect`s intersect with each other.
    #[rustfmt::skip]
    fn do_rects_intersect(rect_one: &Rect, rect_two: &Rect) -> bool {
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

pub(super) struct CollisionRect {
    pub id:             Index,
    pub collision_type: CollisionType,
    pub rect:           Rect,
}

pub(super) fn new_collision_rect_and_position(
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
            rect:           Rect {
                top:    new_position.1 - half_size.1,
                bottom: new_position.1 + half_size.1,
                left:   new_position.0 - half_size.0,
                right:  new_position.0 + half_size.0,
            },
        },
        new_position,
    )
}

pub(super) fn gen_collision_grid(
    entities: &Entities,
    positions: &WriteStorage<Position>,
    sizes: &ReadStorage<Size>,
    collisions: &ReadStorage<Collision>,
) -> CollisionGrid {
    let mut collision_grid = CollisionGrid::default();
    for (entity, position, size, collision) in
        (entities, positions, sizes, collisions).join()
    {
        let half_size = (size.w * 0.5, size.h * 0.5);
        collision_grid.push(CollisionRect {
            id:             entity.id(),
            collision_type: collision.collision_type.clone(),
            rect:           Rect {
                top:    position.y - half_size.1,
                bottom: position.y + half_size.1,
                left:   position.x - half_size.0,
                right:  position.x + half_size.0,
            },
        });
    }
    collision_grid
}

pub(super) fn run_with_collision(
    dt: f32,
    collision_grid: &CollisionGrid,
    entity: &Entity,
    position: &mut Position,
    velocity: &Velocity,
    size: &Size,
    collider: &mut Collider,
    collision: &Collision,
) {
    let entity_id = entity.id();
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
            for collision_data in
                collision_grid.collisions_with(&collision_rect)
            {
                collider.push(collision_data);
            }
            position.x = new_position.0;
            position.y = new_position.1;
        }
        // Try to move by the floating point remainder
        // Calculate new position
        let (collision_rect, new_position) = new_collision_rect_and_position(
            entity_id,
            &collision.collision_type,
            position,
            size,
            &axis,
            rem,
        );
        // Check for collision in newly calculated position
        for collision_data in collision_grid.collisions_with(&collision_rect) {
            collider.push(collision_data);
        }
        position.x = new_position.0;
        position.y = new_position.1;
    });
}
