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
mod input;
mod resources;
mod settings;
mod systems;

const SLEEP_MS: u64 = 50;

pub fn flush_stdout() {
    use std::io::{stdout, Write};
    stdout().flush().expect("Should flush stdout");
}

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
        .with(DeltatimeSystem::default(), "deltatime_system", &[])
        .with(InputSystem::default(), "input_system", &[])
        .with(MoveEntitiesSystem::default(), "move_entities_system", &[
            "deltatime_system",
        ])
        .with(DrawRoomSystem::default(), "draw_room_system", &[
            "move_entities_system",
        ])
        .with(DrawEntitiesSystem::default(), "draw_entities_system", &[
            "move_entities_system",
            "draw_room_system",
        ])
        .with(
            ControlPaddlesSystem::default(),
            "control_paddles_system",
            &["input_system"],
        )
        .with(BallBounceSystem::default(), "ball_bounce_system", &[
            "move_entities_system",
        ])
        .build();

    let cursor = TerminalCursor::new();
    cursor.hide().unwrap();

    let settings = load_settings();
    world.insert(Deltatime::default());
    world.insert(InputManager::new(settings.bindings.clone()));
    world.insert(load_settings());
    world.insert(AlternateScreen::to_alternate(RAW_MODE).unwrap());
    world.insert(cursor);
    world.insert(TerminalInput::new());

    create_paddles(&mut world);
    create_ball(&mut world);

    (world, dispatcher)
}

fn create_paddles(world: &mut World) {
    use components::prelude::*;
    use specs::Builder;

    world.register::<Paddle>();
    world.register::<Position>();
    world.register::<Size>();
    world.register::<Drawable>();
    world.register::<Velocity>();
    world.register::<Collider>();
    world.register::<Collision>();

    let settings = (*world.read_resource::<Settings>()).clone();

    let paddle_x = settings.paddle.size.0 * 0.5;
    let paddle_y = settings.room.height as f32 * 0.5;
    let paddle_size = Size::new(settings.paddle.size.0, settings.paddle.size.1);
    let paddle_char = settings.chars.paddle;

    // Left paddle
    world
        .create_entity()
        .with(Paddle::new(PaddleSide::Left))
        .with(Drawable::new(paddle_char))
        .with(Position::new(paddle_x, paddle_y))
        .with(paddle_size.clone())
        .with(Velocity::default())
        .with(Collision::new(CollisionType::Paddle(PaddleSide::Left)))
        .build();

    // Right paddle
    world
        .create_entity()
        .with(Paddle::new(PaddleSide::Right))
        .with(Drawable::new(paddle_char))
        .with(Position::new(
            settings.room.width as f32 - paddle_x,
            paddle_y,
        ))
        .with(paddle_size.clone())
        .with(Velocity::default())
        .with(Collision::new(CollisionType::Paddle(PaddleSide::Right)))
        .build();
}

fn create_ball(world: &mut World) {
    use components::prelude::*;
    use specs::Builder;

    world.register::<Ball>();

    let settings = (*world.read_resource::<Settings>()).clone();

    world
        .create_entity()
        .with(Ball::default())
        .with(Drawable::new('O'))
        .with(Position::new(
            settings.room.width as f32 * 0.5,
            settings.room.height as f32 * 0.5,
        ))
        .with(Size::new(2.0, 2.0))
        .with(Velocity::new(8.0, 0.1))
        .with(Collision::new(CollisionType::Ball))
        .with(Collider::default())
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
