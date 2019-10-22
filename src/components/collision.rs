use super::component_prelude::*;
use super::prelude::PaddleThird;

#[derive(Clone, PartialEq)]
pub enum CollisionType {
    Paddle(Side),
    Ball,
    Wall(Side),
}

#[derive(Clone, PartialEq)]
pub enum CollisionInfo {
    None,
    PaddleThird(PaddleThird),
}

impl Default for CollisionInfo {
    fn default() -> Self {
        Self::None
    }
}

#[derive(PartialEq)]
pub struct CollisionData {
    pub collision_type: CollisionType,
    pub collision_info: CollisionInfo,
}

impl CollisionData {
    pub fn new(collision_type: CollisionType) -> Self {
        Self {
            collision_type,
            collision_info: Default::default(),
        }
    }

    pub fn add_info(&mut self, collision_info: CollisionInfo) {
        self.collision_info = collision_info;
    }
}

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Collider {
    collisions: Vec<CollisionData>,
}

impl Collider {
    pub fn in_collision_with(&self, collision_type: &CollisionType) -> bool {
        self.collisions.iter().any(|collision_data| {
            &collision_data.collision_type == collision_type
        })
    }

    pub fn collision_info_with(
        &self,
        collision_type: &CollisionType,
    ) -> Option<CollisionInfo> {
        self.collisions.iter().find_map(|collision_data| {
            if &collision_data.collision_type == collision_type {
                Some(collision_data.collision_info.clone())
            } else {
                None
            }
        })
    }

    pub fn clear(&mut self) {
        self.collisions.clear();
    }

    pub fn push(&mut self, collision_data: CollisionData) {
        if !self.collisions.contains(&collision_data) {
            self.collisions.push(collision_data);
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
