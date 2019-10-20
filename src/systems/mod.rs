pub mod prelude {
    pub use super::control_paddles::ControlPaddlesSystem;
    pub use super::draw_entities::DrawEntitiesSystem;
    pub use super::draw_room::DrawRoomSystem;
    pub use super::input::InputSystem;
    pub use super::move_entities::MoveEntitiesSystem;
}

mod system_prelude {
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

mod control_paddles;
mod draw_entities;
mod draw_room;
mod input;
mod move_entities;
