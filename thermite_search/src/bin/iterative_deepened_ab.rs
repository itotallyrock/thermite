use std::sync::Arc;
use std::time::Duration;
use thermite_core::board::Board;
use thermite_search::Search;
use thermite_search::halt_flag::HaltFlag;
use thermite_search::search_constraints::SearchConstraints;
use thermite_search::search_inputs::SearchInputs;

fn search(halt_flag: Arc<HaltFlag>) {
    let board = Board::from_fen("r4rk1/1pq1bppp/p3bn2/2p1p3/4P1P1/P1N1BP2/1PPQ3P/2KR1B1R w - - 0 14").unwrap();
    let inputs = SearchInputs {
        halt_flag,
        #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "countermove_heuristic", feature = "piece_square_heuristic", feature = "static_exchange_eval"))]
        move_ordering_state: Default::default(),
        #[cfg(feature = "transposition_table")]
        transposition_table: Default::default(),
    };
    let constraints = SearchConstraints::new()
        .with_time(Duration::from_secs(300));
    let search = Search::create(board, inputs, 5);
    let result = search.start(constraints);

    println!("{result:?}");
}

fn main() {
    let halt_flag = Arc::new(HaltFlag::default());
    let thread_halt_flag = halt_flag.clone();
    let thread = std::thread::spawn(move || search(thread_halt_flag));

    std::thread::sleep(Duration::from_millis(300_000));
    halt_flag.halt();

    thread.join().expect("thread panicked");
}