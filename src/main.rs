use crate::driver::Driver;
use crate::engine::ThermiteEngine;

mod uci;
mod game;
mod driver;
mod engine;
mod engine_types;

fn main() {
    let engine = ThermiteEngine::new();

    if let Err(err) = Driver::start(engine) {
        eprintln!("Fatal Error: {}", err);
    }
}

/*
    uci -> uciok
    debug {on|off} -> ()
    isready -> readyok
    setoption name {name} [value {value}] -> ()
    ucinewgame -> ()
    position -> ()
    go [depth {N}] [nodes {N}] [infinite] [ponder] [searchmoves [...moves]] [wtime [N]] [btime [N]] [winc [N]] [binc [N]] [movestogo [N > 0]] [mate [N]] -> bestmove {move} [ponder {move}]
    stop -> ()
    ponderhit -> ()
 */