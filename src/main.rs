use crate::driver::Driver;
use crate::engine::ThermiteEngine;

mod driver;
mod engine;
mod engine_types;
mod game;
mod uci;

fn main() {
    let engine = ThermiteEngine::new();

    if let Err(err) = Driver::start(engine) {
        eprintln!("Fatal Error: {}", err);
    }
}
