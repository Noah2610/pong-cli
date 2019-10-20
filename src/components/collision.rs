use super::component_prelude::*;
use super::paddle::PaddleSide;

#[derive(Clone, PartialEq)]
pub enum WallSide {
    Top,
    Bottom,
}

#[derive(Clone, PartialEq)]
pub enum CollisionType {
    Paddle(PaddleSide),
    Ball,
    Wall(WallSide),
}

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Collider {
    collisions: Vec<CollisionType>,
}

impl Collider {
    pub fn in_collision_with(&self, collision_type: &CollisionType) -> bool {
        self.collisions.contains(collision_type)
    }

    pub fn clear(&mut self) {
        self.collisions.clear();
    }

    pub fn push(&mut self, collision_type: CollisionType) {
        if !self.collisions.contains(&collision_type) {
            self.collisions.push(collision_type);
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collision {
    pub collision_type: CollisionType,
}

impl Collision {
    pub fn new(collision_type: CollisionType) -> Self {
        Self { collision_type }
    }
}
