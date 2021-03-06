extern crate crossterm;
extern crate dirs;
#[cfg(feature = "random")]
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

mod components;
mod geo;
mod helpers;
mod input;
mod pong;
mod resources;
mod round_down;
mod settings;
#[cfg(feature = "style")]
mod style;
mod systems;

pub fn run() {
    pong::run();
}
