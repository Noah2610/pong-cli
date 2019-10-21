use super::component_prelude::*;

pub enum MoveDirection {
    Up,
    Down,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Paddle {
    pub side:   Side,
    pub moving: Option<MoveDirection>,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Self { side, moving: None }
    }

    pub fn move_up(&mut self) {
        self.moving = Some(MoveDirection::Up);
    }

    pub fn move_down(&mut self) {
        self.moving = Some(MoveDirection::Down);
    }
}
