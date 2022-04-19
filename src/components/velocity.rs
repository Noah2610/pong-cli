use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
