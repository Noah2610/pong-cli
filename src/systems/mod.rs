pub mod prelude {
    pub use super::ball_bounce::BallBounceSystem;
    pub use super::control_paddles::ControlPaddlesSystem;
    pub use super::deltatime::DeltatimeSystem;
    pub use super::draw_entities::DrawEntitiesSystem;
    pub use super::draw_room::DrawRoomSystem;
    pub use super::input::InputSystem;
    pub use super::move_entities::MoveEntitiesSystem;
}

mod system_prelude {
    pub use specs::world::Index;
    pub use specs::{
        Entities,
        Entity,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteExpect,
        WriteStorage,
    };

    pub use crate::components::prelude::*;
    pub use crate::flush_stdout;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
}

mod ball_bounce;
mod control_paddles;
mod deltatime;
mod draw_entities;
mod draw_room;
mod input;
mod move_entities;
