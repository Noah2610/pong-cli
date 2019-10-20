use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Size {
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}
