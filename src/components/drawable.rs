use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub character: char,
}

impl Drawable {
    pub fn new(character: char) -> Self {
        Self { character }
    }
}
