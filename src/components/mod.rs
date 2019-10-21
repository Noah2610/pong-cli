pub mod prelude {
    pub use super::ball::Ball;
    pub use super::collision::{Collider, Collision, CollisionType};
    pub use super::drawable::Drawable;
    pub use super::paddle::{MoveDirection, Paddle};
    pub use super::paddle_ai::PaddleAi;
    pub use super::position::Position;
    pub use super::size::Size;
    pub use super::velocity::Velocity;
}

pub mod component_prelude {
    pub use specs::{
        Component,
        DenseVecStorage,
        HashMapStorage,
        NullStorage,
        Storage,
        VecStorage,
    };

    pub use crate::geo::prelude::*;
}

mod ball;
mod collision;
mod drawable;
mod paddle;
mod paddle_ai;
mod position;
mod size;
mod velocity;
