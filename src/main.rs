extern crate crossterm;
extern crate ron;
#[macro_use]
extern crate serde;
#[macro_use]
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

    const RAW_MODE: bool = true;

    let mut world = World::new();
    let dispatcher = DispatcherBuilder::new()
        .with(InputSystem::default(), "input_system", &[])
        .with(DrawRoomSystem::default(), "draw_room_system", &[])
        .build();

    let cursor = TerminalCursor::new();
    cursor.hide().unwrap();

    world.insert(load_settings());
    world.insert(AlternateScreen::to_alternate(RAW_MODE).unwrap());
    world.insert(cursor);
    world.insert(TerminalInput::new());

    create_paddles(&mut world);

    (world, dispatcher)
}

fn create_paddles(world: &mut World) {
    use components::prelude::*;
    use specs::Builder;

    // TODO remove
    world.register::<Paddle>();
    world.register::<Position>();
    world.register::<Size>();

    let settings = (*world.read_resource::<Settings>()).clone();

    let paddle_x = settings.paddles.size.0 * 0.5;
    let paddle_y = settings.room.height as f32 * 0.5;
    let paddle_size =
        Size::new(settings.paddles.size.0, settings.paddles.size.1);

    // Left paddle
    world
        .create_entity()
        .with(Paddle::new(PaddleSide::Left))
        .with(Position::new(paddle_x, paddle_y))
        .with(paddle_size.clone())
        .build();

    // Right paddle
    world
        .create_entity()
        .with(Paddle::new(PaddleSide::Right))
        .with(Position::new(
            settings.room.width as f32 - paddle_x,
            paddle_y,
        ))
        .with(paddle_size.clone())
        .build();
}

fn cleanup(world: World) {
    world.read_resource::<AlternateScreen>().to_main().unwrap();
    world.read_resource::<TerminalCursor>().show().unwrap();
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file =
        File::open("./settings.ron").expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
