use std::io::stdout;

use std::sync::Arc;
use std::time::Duration;
use tracing::Level;
use thermite_core::board::Board;
use thermite_search::Search;
use thermite_search::halt_flag::HaltFlag;
use thermite_search::search_constraints::SearchConstraints;
use thermite_search::search_inputs::SearchInputs;

fn search() {
    let halt_flag = Arc::new(HaltFlag::default());
    let board = Board::from_fen("r1b2r2/pp1nbpk1/1q2p1B1/3pP3/5P2/1P5R/PBPN2PP/R2Q3K w - - 1 1").unwrap();
    // let board = Board::from_fen("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4").unwrap();
    let inputs = SearchInputs {
        halt_flag,
        ..Default::default()
    };
    let constraints = SearchConstraints::new()
        .with_depth(8)
        .with_time(Duration::from_secs(30));
    let search = Search::create(board, inputs, 5);
    match search.start(constraints) {
        Ok(results) => println!("search completed: position evaluated as {results:?}"),
        Err(err) => eprintln!("search error: {:?}", err),
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_writer(stdout)
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .init();
    search();
}
