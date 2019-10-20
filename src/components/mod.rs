pub mod prelude {
    pub use super::drawable::Drawable;
    pub use super::paddle::{Paddle, PaddleSide};
    pub use super::position::Position;
    pub use super::size::Size;
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

mod drawable;
mod paddle;
mod position;
mod size;
