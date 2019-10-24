extern crate crossterm;
#[cfg(feature = "random")]
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

#[cfg(feature = "color")]
mod color;
mod components;
mod geo;
mod helpers;
mod input;
mod pong;
mod resources;
mod round_down;
mod settings;
mod systems;

pub fn run() {
    pong::run();
}
