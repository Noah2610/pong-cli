extern crate crossterm;
#[cfg(feature = "random")]
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate specs;

mod components;
mod geo;
mod input;
mod pong;
mod resources;
mod settings;
mod systems;

pub fn flush_stdout() {
    use std::io::{stdout, Write};
    stdout().flush().expect("Should flush stdout");
}

fn main() {
    pong::run();
    eprintln!("Clean exit!");
}
