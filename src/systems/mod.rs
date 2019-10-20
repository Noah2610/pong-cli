pub mod prelude {
    pub use super::draw_room::DrawRoomSystem;
}

mod system_prelude {
    pub use specs::{Read, ReadStorage, System, Write, WriteStorage};
}

mod draw_room;
