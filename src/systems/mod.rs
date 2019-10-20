pub mod prelude {
    pub use super::draw_room::DrawRoomSystem;
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
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
}

mod draw_room;
