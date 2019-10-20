use super::component_prelude::*;

pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Paddle {
    pub side: PaddleSide,
}

impl Paddle {
    pub fn new(side: PaddleSide) -> Self {
        Self { side }
    }
}
