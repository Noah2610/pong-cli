use super::system_prelude::*;

#[derive(Default)]
pub struct DrawRoomSystem;

impl<'a> System<'a> for DrawRoomSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        println!("DRAW ROOM");
    }
}
