//! # About
//! I'm creating Thermite as an attempt to design, create, and polish a full-scale rust project.
//! The end-goal being a semi-competitive analysis [chess engine](https://en.wikipedia.org/wiki/Chess_engine), similar to [Stockfish](https://github.com/official-stockfish/Stockfish).
//!
//! That means sometimes [prioritizing speed over stability](#priorities).
//! That also means Thermite must support [a bunch of features found in modern chess engines](#features) in order to be competitive.
//!
//! # Priorities
//!
//! ## Objectives
//! The hope is to eventually create a correct, performant, sophisticated, and reliable chess search engine.
//! To obtain all of these goals, [some compromises will need to be made](#library-stability).
//!
//! #### Correctness
//! Correctness is the combination of properly implementing the game logic and avoiding logic errors within the rest of the project.
//! Rust lends itself well to avoiding logic errors, which makes it a great choice for this project.
//!
//! This stems from a strong type system, fearless concurrency, and explicit error handling.
//!
//! When it comes to implementing game logic, unit tests should be implemented to cover the sneaky edge cases, likely using [proptest](https://docs.rs/proptest/latest/proptest/).
//! However, the best method for game logic correctness is [perf-testing](https://www.chessprogramming.org/Perft).  Which can quickly find issues with move generation, or faulty game logic.
//!
//! #### Performance
//! Engine performance is the most relevant when it comes to competitive viability.  Whichever chess engine searches more positions usually wins, so optimizing the search is a priority.
//!
//! That being said, optimization requires validation through benchmarks.  I hope to implement a suite of benchmarks for the foundations and hot-paths within the crates.
//! Generally, more emphasis should be on correctness for initial implementation before benchmarking and finally benchmark-backed optimization.
//!
//! #### Sophistication
//! Despite performance being the most important factor, an engine can still be limited by it's ability to properly take advantage of known search techniques.
//! One example of an important technique might be [iterative deepening](https://www.chessprogramming.org/Iterative_Deepening) as it's well known that it can quickly find [PV Moves](https://www.chessprogramming.org/PV-Move) for move ordering.
//!
//! The outline of supported search features can be seen [here](#search-features).
//!
//! #### Reliability
//! For Thermite, reliability means never crashing, it behaves deterministically, and properly that it correctly and fully implements the UCI protocol.
//!
//! ### Library Stability
//! Features and library stability are subject to frequent changes, some of which might be backwards incompatible.
//! These changes will likely be scoped to [major semver changes](https://doc.rust-lang.org/cargo/reference/semver.html), but might still cause issues.
//! However, I expect this engine to be used primarily as a binary not a library.
//!
//! That being said, if you intend to use this as a library please feel free to open an issue outlining your situation.
//!
//! # Features
//!
//! Starting out, the root workspace-level-crate won't support features.
//! This is to try and design it to be the most optimal for general use on all systems.
//! But mainly to keep things simple.  It's a huge time-sink to optimize one chess engine's performance, let alone a bunch of permutations of that engine.
//!
//! ## Core Features
//! - `chess960` - [Chess variant with shuffled back rank](https://en.wikipedia.org/wiki/Fischer_random_chess).
//! - `repetitions` - Keep track of the last 50 moves to properly evaluate against [threefold_repetitions](https://www.chessprogramming.org/Repetitions).
//! - `zobrist` - [Zobrist incremental chess positional hashing](https://www.chessprogramming.org/Zobrist_Hashing).
//!
//! ## Search Features
//! - `pondering` - [UCI search mode](https://backscattering.de/chess/uci/#gui-go-ponder) allowing the engine to search for a best move during the opponents turn.
//! - `q_search` - [Quiescent search](https://www.chessprogramming.org/Quiescence_Search) for searching horizon nodes.
//! - `transposition_table` - [Cached search history](https://www.chessprogramming.org/Transposition_Table), useful with iterative deepening or multithreaded search.
//! - `killer_heuristic` - Move ordering heuristic that [prioritizes moves that recently caused a beta cutoff in sibling nodes](https://www.chessprogramming.org/Killer_Heuristic).
//! - `history_heuristic` - Move ordering technique for [prioritizing moves that frequently result in a beta cutoff](https://www.chessprogramming.org/History_Heuristic).
//! - `countermove_heuristic` - Move ordering taking advantage of [most moves having a single natural response](https://www.chessprogramming.org/Countermove_Heuristic).
//! - `piece_square_heuristic` - Move ordering that uses [piece square tables](https://www.chessprogramming.org/Piece-Square_Tables) to evaluate moves quickly.
//! - `static_exchange_eval` - Move ordering using a [miniature alpha-beta search supporting only captures](https://www.chessprogramming.org/Static_Exchange_Evaluation) and pruning moves that don't yield material.
//! - `progress_reporting` - Periodically provide search debug information ([for UCI support](https://backscattering.de/chess/uci/#engine-info)).
//! - `opening_book` - Move dictionary for looking up [known best-moves for opening positions](https://www.chessprogramming.org/Opening_Book).
//! - `multipv` - Support searching multiple root moves and ranking them by score ([for UCI support](https://backscattering.de/chess/uci/#engine-option-multipv)).
//! - `aspiration_windows` - [Restricted search window](https://www.chessprogramming.org/Aspiration_Windows) to obtain more beta cutoffs.
//! - `late_move_reduction` - [Reduce the search depth for remaining moves if no beta-cutoff has been found](https://www.chessprogramming.org/Late_Move_Reductions) for the first few moves (assuming well ordered moves),
//! - `razoring` - Assuming the opponent can always make a good move if we pass (the [null move hypothesis](https://www.chessprogramming.org/Null_Move_Observation)), we can [prune a move if it doesn't immediately grant an advantage](https://www.chessprogramming.org/Razoring).
//! - `futility_pruning` - When the search is nearing its maximum depth ([frontier nodes](https://www.chessprogramming.org/Frontier_Nodes)) the score is unlikely to change much. Knowing this [we can prune frontier nodes that are below beta by a threshold](https://www.chessprogramming.org/Futility_Pruning)
//! - `null_move_pruning` - Applying the [null move observation](https://www.chessprogramming.org/Null_Move_Observation), we can [search using a more advantageous lower-bound, relative to the current evaluation, in order to prune moves](https://www.chessprogramming.org/Null_Move_Pruning)
//!
//! ## Evaluation Features
//! - `piece_square_evaluation` - [Quick and dirty constant evaluation](https://www.chessprogramming.org/Piece-Square_Tables) based on placing specific pieces on certain squares.
//! - `evaluation_table` - [Evaluation cache](https://www.chessprogramming.org/Evaluation_Hash_Table) to avoid recomputing frequently visited nodes.
//!
//! ## Move Generation Features
//! - `q_moves` - [Quiescent search](https://www.chessprogramming.org/Quiescence_Search) move generation.

/// Run thermite search using a UCI engine driver based on stdin/stdout/stderr
fn main() {
    println!("Hello, Thermite!");
}
