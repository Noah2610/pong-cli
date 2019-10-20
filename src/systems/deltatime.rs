use super::system_prelude::*;

#[derive(Default)]
pub struct DeltatimeSystem;

impl<'a> System<'a> for DeltatimeSystem {
    type SystemData = Write<'a, Deltatime>;

    fn run(&mut self, mut deltatime: Self::SystemData) {
        deltatime.update();
    }
}
