use super::component_prelude::*;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}
