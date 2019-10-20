use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
