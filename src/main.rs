extern crate crossterm;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

use std::thread::sleep;
use std::time::Duration;

use crossterm::AlternateScreen;
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

use resources::prelude::*;
use settings::prelude::*;

mod components;
mod resources;
mod settings;
mod systems;

const SLEEP_MS: u64 = 1000;

fn main() {
    let (mut world, mut dispatcher) = setup();

    world.insert(Running(true));

    while world.read_resource::<Running>().0 {
        dispatcher.dispatch(&mut world);
        sleep(Duration::from_millis(SLEEP_MS));
    }

    cleanup(world);
    eprintln!("Clean exit!");
}

fn setup<'a, 'b>() -> (World, Dispatcher<'a, 'b>) {
    use systems::prelude::*;

    const RAW_MODE: bool = false;

    let mut world = World::new();
    let dispatcher = DispatcherBuilder::new()
        .with(DrawRoomSystem::default(), "draw_room_system", &[])
        .build();

    world.insert(load_settings());
    world.insert(TerminalCursor::new());
    world.insert(AlternateScreen::to_alternate(RAW_MODE).unwrap());

    (world, dispatcher)
}

fn cleanup(world: World) {
    world.write_resource::<AlternateScreen>().to_main().unwrap();
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file =
        File::open("./settings.ron").expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
