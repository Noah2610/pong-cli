use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Into<(f32, f32)> for &Position {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}
