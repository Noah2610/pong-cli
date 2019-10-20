pub mod prelude {
    pub use super::collision::{Collider, Collision, CollisionType};
    pub use super::drawable::Drawable;
    pub use super::paddle::{Paddle, PaddleSide};
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
}

mod collision;
mod drawable;
mod paddle;
mod position;
mod size;
mod velocity;
