use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Paddle {
    pub side: Side,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Self { side }
    }
}
