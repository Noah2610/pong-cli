extern crate crossterm;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

use std::thread::sleep;
use std::time::Duration;

use crossterm::AlternateScreen;
use settings::prelude::*;
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

mod components;
mod settings;
mod systems;

const SLEEP_MS: u64 = 1000;

struct Running(pub bool);

fn main() {
    let (mut world, mut dispatcher) = setup();

    world.insert(Running(true));

    while world.read_resource::<Running>().0 {
        eprintln!("DISPATCH");
        dispatcher.dispatch(&mut world);
        sleep(Duration::from_millis(SLEEP_MS));
    }
    eprintln!("Clean exit!");
}

fn setup<'a, 'b>() -> (World, Dispatcher<'a, 'b>) {
    use systems::prelude::*;

    const RAW_MODE: bool = false;

    let mut world = World::new();
    let dispatcher = DispatcherBuilder::new()
        .with(DrawRoomSystem::default(), "draw_room_system", &[])
        .build();

    let settings = load_settings();
    world.insert(settings);

    AlternateScreen::to_alternate(RAW_MODE).unwrap();

    (world, dispatcher)
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file =
        File::open("./settings.ron").expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
