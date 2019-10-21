pub mod prelude {
    pub use super::ball_bounce::BallBounceSystem;
    pub use super::confine_entities::ConfineEntitiesSystem;
    pub use super::control_paddles::ControlPaddlesSystem;
    pub use super::deltatime::DeltatimeSystem;
    pub use super::draw_entities::DrawEntitiesSystem;
    pub use super::draw_room::DrawRoomSystem;
    pub use super::input::InputSystem;
    pub use super::move_entities::MoveEntitiesSystem;
    pub use super::move_paddles::MovePaddlesSystem;
    pub use super::paddle_ai::PaddleAiSystem;
    pub use super::spawn_ball::SpawnBallSystem;
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
        World,
        WorldExt,
        Write,
        WriteExpect,
        WriteStorage,
    };

    pub use crate::components::prelude::*;
    pub use crate::flush_stdout;
    pub use crate::geo::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
}

mod ball_bounce;
mod confine_entities;
mod control_paddles;
mod deltatime;
mod draw_entities;
mod draw_room;
mod input;
mod move_entities;
mod move_paddles;
mod paddle_ai;
mod spawn_ball;
