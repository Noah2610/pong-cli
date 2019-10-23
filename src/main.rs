extern crate crossterm;
#[cfg(feature = "random")]
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

mod color;
mod components;
mod geo;
mod helpers;
mod input;
mod pong;
mod resources;
mod settings;
mod systems;

fn main() {
    pong::run();
    eprintln!("Clean exit!");
}
