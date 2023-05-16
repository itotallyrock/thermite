#![feature(const_option)]
#![feature(let_chains)]
#![feature(box_patterns)]
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
//! ## Thermite
//! Starting out, the root workspace-level-crate won't support features.
//! This is to try and design it to be the most optimal for general use on all systems.
//! But mainly to keep things simple.
//! 1. It's a huge time-sink to optimize one chess engine's performance, let alone a bunch of permutations of that engine.
//! 2. As the root binary will be the primary crate, features should be [more stable](#library-stability).
//! 3. Can require sizable refactoring or the near duplication of massive chunks of code.
//!
//! It's possible some features might be added for my own convenience while working on the engine:
//! - `file_io` - supporting reading from and writing to files from args instead of stdin/stderr/stdout
//! - `binary_info` - get information on which CPU architecture and feature-sets, optimization level, debug symbols, etc
//! - `persist_tt` - export the transposition table to a file (maybe binary format but likely JSON)
//!
//! Eventually, it would be nice to support some features beneficial to end-user, but **these aren't likely to happen soon or at all**:
//! - `std` - to support `#[no_std]` environments
//! - [more chess variants](https://en.wikipedia.org/wiki/List_of_chess_variants).
//!
//! ## Core Features
//! - `chess960` - [Chess variant with shuffled back rank](https://en.wikipedia.org/wiki/Fischer_random_chess).
//! - `repetitions` - Keep track of the last 50 moves to properly evaluate against [threefold_repetitions](https://www.chessprogramming.org/Repetitions).
//! - `zobrist` - [Zobrist incremental chess positional hashing](https://www.chessprogramming.org/Zobrist_Hashing).
//! - `move_generation` - Keep track of state necessary for [generating moves](https://www.chessprogramming.org/Move_Generation).
//! - `nnue_accumulator` - Keep track of state necessary for evaluating a position using [NNUE](https://www.chessprogramming.org/NNUE).
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
//! - `chess_960` - Allow properly searching (generating moves and evaluating) [chess 960](https://en.wikipedia.org/wiki/Fischer_random_chess) positions.
//!
//! ## Evaluation Features
//! - `piece_square_evaluation` - [Quick and dirty constant evaluation](https://www.chessprogramming.org/Piece-Square_Tables) based on placing specific pieces on certain squares.
//! - `evaluation_table` - [Evaluation cache](https://www.chessprogramming.org/Evaluation_Hash_Table) to avoid recomputing frequently visited nodes.
//! - `nnue_eval` - Evaluate positions using an [efficiently updated neural network](https://www.chessprogramming.org/NNUE) instead of a simple classic evaluation.
//!
//! ## Move Generation Features
//! - `q_moves` - [Quiescent search](https://www.chessprogramming.org/Quiescence_Search) move generation.
//! - `chess_960` - Support generating castles for [Fischer random chess](https://en.wikipedia.org/wiki/Fischer_random_chess).

use std::clone::Clone;
use std::io::{BufReader, Write};
use std::num::NonZeroU16;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{available_parallelism, JoinHandle};

use lazy_static::lazy_static;
use spmc::{channel as worker_channel, Sender as WorkSender};

use thermite_core::board::Board;
use thermite_core::score::PositionEvaluation;
use thermite_search::{NodeCount, Search};
use thermite_search::halt_flag::HaltFlag;
use thermite_search::search_constraints::SearchConstraints;
use thermite_search::search_error::RootSearchError;
use thermite_search::search_inputs::SearchInputs;
use thermite_search::search_results::{MAX_MULTI_PV_LINES, SearchResult};
use uci_driver::{UciCommandParseError, UciGuiCommand, UciNumber, UciOption, UciOptionConfiguration, UciOptionTypeConfiguration, UciOptionValue, UciReader, UciSearchOptions, UciWriter};

const NAME: &str = "Thermite";
const AUTHORS: &str = "Jeffrey Meyer";
const LICENSE: &str = "GPLv3";
const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref STARTUP_TEXT: String = format!("{NAME} v{VERSION} ({LICENSE}) by {AUTHORS}");
}

pub fn start<R: IntoIterator<Item=Result<UciGuiCommand, UciCommandParseError>>, W: Write>(reader: R, mut writer: UciWriter<W>) {
    let mut search_options = ThermiteOptions::default();
    let mut search_driver = SearchDriver::new();

    for command_result in reader.into_iter() {
        match command_result {
            Ok(command) => match command {
                UciGuiCommand::SetOption(uci_option) => {
                    let _ = search_driver.try_stop();
                    if let Ok(new_search_options) = search_options.update(uci_option) {
                        search_options = new_search_options;
                        let _ = search_driver.update_options(search_options);
                    }
                },
                UciGuiCommand::Uci => {
                    let _ = writer.write_id(NAME, AUTHORS);
                    let _ = writer.write_options(ThermiteOptions::get_uci_options_config());
                    let _ = writer.write_uci_ok();
                },
                UciGuiCommand::IsReady => {
                    if let Err(err) = search_driver.try_initialize() {
                        match err {
                            InitializationError::AlreadyInitialized => {}
                        }
                    }
                    let _ = writer.write_ready_ok();
                },
                UciGuiCommand::Debug(debug_enabled) => todo!("debugging {debug_enabled}"),
                UciGuiCommand::Position(box board) => {
                    let _ = search_driver.set_position(board);
                },
                UciGuiCommand::UciNewGame => search_driver.new_game(),
                UciGuiCommand::Go(uci_search_options) => {
                    let _ = search_driver.try_initialize();
                    let _ = search_driver.try_start(uci_search_options);
                    // TODO: Log error if we fail to start
                    // TODO: Figure out how we can print "bestmove {} {}" to uci writer without blocking here
                },
                UciGuiCommand::Stop => {
                    if let Ok(result) = search_driver.try_stop() {
                        let best_move = result.principle_variation.first().copied().flatten();
                        if let Some(best_move) = best_move {
                            let refutation = result.principle_variation.get(1).copied().flatten();
                            let _ = writer.write_best_move(best_move, refutation);
                        }
                    }
                },
                UciGuiCommand::PonderHit => {
                    let _ = search_driver.try_ponderhit();
                },
                UciGuiCommand::Quit => break,
            },
            Err(uci_command_parse_error) => eprintln!("{uci_command_parse_error}"),
        }
    };
}

pub struct SearchDriver {
    state: SearchDriverState,
}

pub type ThreadCount = NonZeroU16;

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ThermiteOptions {
    multipv_count: usize,
    /// TODO
    num_threads: ThreadCount,
    /// TODO
    tt_capacity: NodeCount,
}

/// The default number of max threads for maintaining thread efficiency for a majority of machines
const DEFAULT_MAX_THREADS: ThreadCount = ThreadCount::new(8).unwrap();
lazy_static! {
    /// The maximum number of threads we should allow, takes into account available parallelism before falling back to [`DEFAULT_MAX_THREADS`]
    static ref MAX_THREADS: ThreadCount = get_maximum_thread_count();

    static ref UCI_OPTIONS_CONFIG: Vec<UciOptionConfiguration> = vec![
        UciOptionConfiguration {
            name: THREAD_COUNT_UCI_OPTION_NAME,
            config: UciOptionTypeConfiguration::Spin { min: Some(1), max: Some(MAX_THREADS.get() as UciNumber), default: Some(MAX_THREADS.get() as UciNumber) }
        },
        UciOptionConfiguration {
            name: TT_CAPACITY_UCI_OPTION_NAME,
            // TODO: Convert maxes to mebibytes and reconvert when creating TTg
            config: UciOptionTypeConfiguration::Spin { min: Some(MIN_TT_CAPACITY as UciNumber), max: Some(MAX_TT_CAPACITY as UciNumber), default: Some(MAX_TT_CAPACITY as UciNumber) }
        }
    ];
}

fn get_maximum_thread_count() -> ThreadCount {
    available_parallelism()
        .ok()
        .and_then(|max_threads| ThreadCount::try_from(max_threads).ok())
        .unwrap_or(DEFAULT_MAX_THREADS)
}

/// The capacity in terms of number of entries for the [`thermite_search::transposition_table::TranspositionTable`] to support
const MAX_TT_CAPACITY: NodeCount = 256 * 1024 * 1024;// 256 MiB
const MIN_TT_CAPACITY: NodeCount = 1024 * 1024;// 1 MiB

impl Default for ThermiteOptions {
    fn default() -> Self {
        Self {
            multipv_count: 1,
            num_threads: ThreadCount::new(1).unwrap(),
            tt_capacity: MAX_TT_CAPACITY,
        }
    }
}

pub enum IllegalSetOption {
    AttemptToAssignToWrongType {
        name: &'static str,
    },
    UnrecognizedName(String),
    IllegalValue {
        name: &'static str,
        value: UciOptionValue,
    },
}

fn read_number_value(name: &'static str, value: &UciOptionValue) -> Result<UciNumber, IllegalSetOption> {
    match value {
        UciOptionValue::Spin(number) => Ok(*number),
        _ => Err(IllegalSetOption::AttemptToAssignToWrongType { name })
    }
}

const TT_CAPACITY_UCI_OPTION_NAME: &str = "Hash";
const THREAD_COUNT_UCI_OPTION_NAME: &str = "Threads";

impl ThermiteOptions {
    /// TODO
    pub fn get_uci_options_config() -> impl Iterator<Item=UciOptionConfiguration> {
        UCI_OPTIONS_CONFIG.iter().cloned()
    }

    /// TODO
    pub fn with_num_threads(mut self, num_threads: ThreadCount) -> Self {
        self.num_threads = num_threads.min(*MAX_THREADS);

        self
    }

    /// TODO
    pub fn with_multipv(mut self, num_lines: usize) -> Self {
        self.multipv_count = num_lines.max(1).min(MAX_MULTI_PV_LINES);

        self
    }

    /// TODO
    pub fn with_tt_capacity(mut self, tt_capacity: NodeCount) -> Self {
        self.tt_capacity = tt_capacity.min(MAX_TT_CAPACITY).next_power_of_two();

        self
    }

    pub fn update(self, uci_option: UciOption) -> Result<Self, IllegalSetOption> {
        let UciOption { name, value } = uci_option;
        match name.as_str() {
            THREAD_COUNT_UCI_OPTION_NAME => {
                let number_value = read_number_value(THREAD_COUNT_UCI_OPTION_NAME, &value)?;
                let number_value = u16::try_from(number_value).or(Err(IllegalSetOption::IllegalValue { name: THREAD_COUNT_UCI_OPTION_NAME, value: value.clone() }))?;
                let num_threads = ThreadCount::new(number_value).ok_or(IllegalSetOption::IllegalValue { name: THREAD_COUNT_UCI_OPTION_NAME, value })?;

                Ok(self.with_num_threads(num_threads))
            },
            TT_CAPACITY_UCI_OPTION_NAME => {
                let number_value = read_number_value(TT_CAPACITY_UCI_OPTION_NAME, &value)?;
                let tt_capacity = NodeCount::try_from(number_value).or(Err(IllegalSetOption::IllegalValue { name: TT_CAPACITY_UCI_OPTION_NAME, value }))?;

                Ok(self.with_tt_capacity(tt_capacity))
            },
            _ => Err(IllegalSetOption::UnrecognizedName(name)),
        }
    }
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StartSearchError {
    AlreadySearching,
    NotReady,
    UnableToSendToWorkerThreads,
    NotPondering,
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StopSearchError {
    NotSearching,
    UnableToReceiveResults,
    NoResultsReturned,
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InitializationError {
    AlreadyInitialized,
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct SearchOptions {
    constraints: SearchConstraints,
}

impl From<UciSearchOptions> for SearchOptions {
    fn from(value: UciSearchOptions) -> Self {
        Self {
            constraints: value.constraints,
        }
    }
}

struct ReadySearchState {
    halt_flag: Arc<HaltFlag>,
    result_receiver: Receiver<Result<SearchResult, RootSearchError>>,
    board: Board,
    inputs: SearchInputs,
    search_sender: WorkSender<(Search, SearchOptions)>,
    thread_pool: Vec<JoinHandle<()>>,
    options: ThermiteOptions,
}

enum SearchDriverState {
    /// The invalid state when switching between other states
    Intermediate,
    /// The state before the `isready` after setting the position or options
    Idle {
        board: Option<Box<Board>>,
        options: ThermiteOptions,
    },
    /// The GUI has asked us to initialize the engine and has waited for us to be ready
    ReadyToSearch {
        state: Box<ReadySearchState>,
    },
    /// We are currently searching
    Searching {
        is_pondering: bool,
        ready_state: Box<ReadySearchState>,
    },
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SetPositionError {
    FailedToStopSearch(StopSearchError),
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UpdateOptionsError {
    FailedToStopSearch(StopSearchError),
}

impl SearchDriver {
    pub fn new() -> Self {
        Self {
            state: SearchDriverState::Idle {
                board: None,
                options: Default::default(),
            },
        }
    }

    pub fn update_options(&mut self, options: ThermiteOptions) -> Result<(), UpdateOptionsError> {
        // If we're searching stop and then try to update the options
        if let SearchDriverState::Searching { .. } = self.state {
            // TODO: Dont expect this
            self.try_stop().map_err(UpdateOptionsError::FailedToStopSearch)?;
            return self.update_options(options);
        }

        // Get an owned copy of the current state
        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        let board = match prior_state {
            SearchDriverState::Idle { board, .. } => board,
            SearchDriverState::ReadyToSearch { state: box ReadySearchState { board, .. } } => Some(Box::new(board)),
            SearchDriverState::Intermediate => panic!("attempting to read board to update options from intermediate search driver state"),
            SearchDriverState::Searching { .. } => unreachable!(),
        };

        // Update the state to idle with the new options
        self.state = SearchDriverState::Idle {
            board,
            options,
        };

        Ok(())
    }

    pub fn set_position(&mut self, board: Board) -> Result<(), SetPositionError> {
        // If we're searching stop and then try to update the options
        if let SearchDriverState::Searching { .. } = self.state {
            self.try_stop().map_err(SetPositionError::FailedToStopSearch)?;
            return self.set_position(board);
        }

        // Get an owned copy of the current state
        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        let options = match prior_state {
            SearchDriverState::Idle { options, .. } => options,
            SearchDriverState::ReadyToSearch { state: box ReadySearchState { options, .. } } => options,
            SearchDriverState::Intermediate => panic!("attempting to read board to update position from intermediate search driver state"),
            SearchDriverState::Searching { .. } => unreachable!(),
        };

        // Update the state to idle with the new options
        self.state = SearchDriverState::Idle {
            board: Some(Box::new(board)),
            options,
        };

        Ok(())
    }

    pub fn new_game(&mut self) {
        todo!("Reset internal state and start position")
    }

    pub fn try_initialize(&mut self) -> Result<(), InitializationError> {
        if matches!(&self.state, SearchDriverState::ReadyToSearch { .. } | SearchDriverState::Searching { .. }) {
            return Err(InitializationError::AlreadyInitialized);
        }

        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        // Unnecessary match, but we want to destructure our value
        let (board, options) = match prior_state {
            SearchDriverState::Idle { board, options } => (board.map(|boxed_board| *boxed_board).unwrap_or(Board::starting_position()), options),
            SearchDriverState::Intermediate => panic!("attempting to initialize from intermediate search driver state"),
            SearchDriverState::ReadyToSearch { .. } | SearchDriverState::Searching { .. } => unreachable!(),
        };

        let num_threads = options.num_threads.get() as usize;
        let (results_sender, result_receiver) = channel();
        let (search_sender, search_receiver) = worker_channel();
        let mut thread_pool = Vec::with_capacity(num_threads);
        println!("initializing thread pool of {num_threads} threads");
        for worker_thread_index in 0..num_threads {
            let thread_receiver = search_receiver.clone();
            let thread_sender = results_sender.clone();
            thread_pool.push(std::thread::spawn(move || while let Ok((search, search_options)) = thread_receiver.recv() {
                println!("thread #{worker_thread_index}: received search {search_options:#?}");
                let search: Search = search;
                let search_options: SearchOptions = search_options;
                let search_result = search.start(search_options.constraints)
                    .and_then(|results| results.first().ok_or(RootSearchError::NoResultsReturned).cloned());
                println!("thread #{worker_thread_index}: finished search {search_result:#?}");
                // Send the result
                let send_result = thread_sender.send(search_result);

                // If we failed to send the result then we don't have any future work
                if send_result.is_err() {
                    println!("thread #{worker_thread_index}: killing thread after failing to send search results");
                    break;
                }
            }));
        }

        self.state = SearchDriverState::ReadyToSearch {
            state: Box::new(ReadySearchState {
                halt_flag: Arc::new(Default::default()),
                result_receiver,
                board,
                inputs: Default::default(),
                search_sender,
                thread_pool,
                options: Default::default(),
            })
        };

        Ok(())
    }

    pub fn try_start(&mut self, uci_search_options: UciSearchOptions) -> Result<(), StartSearchError> {
        if matches!(&self.state, SearchDriverState::Idle { .. }) {
            return Err(StartSearchError::NotReady);
        }

        if matches!(&self.state, SearchDriverState::Searching { .. }) {
            return Err(StartSearchError::AlreadySearching);
        }

        let is_pondering = uci_search_options.is_pondering;
        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        self.state = match prior_state {
            SearchDriverState::ReadyToSearch { state: mut ready_state } => {
                let ReadySearchState { board, inputs, search_sender, thread_pool, .. } = ready_state.as_mut();
                // Create the search
                let search = Search::create(*board, inputs.clone(), uci_search_options.multi_pv_count.unwrap_or(1));
                let search_options = uci_search_options.into();
                let num_threads = thread_pool.len();

                // Send search to threads
                println!("sending search to threads");
                for _ in 0..num_threads {
                    // TODO: Maybe modify the original_options sent to each thread so we somewhat cover more positions
                    search_sender.send((search.clone(), search_options)).or(Err(StartSearchError::UnableToSendToWorkerThreads))?;
                    // TODO: Try Instead of a sender use a RwLock buffer and each thread modifies its thread_id index
                }

                SearchDriverState::Searching {
                    is_pondering,
                    ready_state,
                }
            },
            SearchDriverState::Intermediate => panic!("attempting to start search from intermediate search driver state"),
            SearchDriverState::Searching { .. } | SearchDriverState::Idle { .. } => unreachable!(),
        };

        Ok(())
    }

    pub fn try_stop(&mut self) -> Result<SearchResult, StopSearchError> {
        if matches!(self.state, SearchDriverState::Idle { .. }) {
            return Err(StopSearchError::NotSearching);
        }

        if matches!(self.state, SearchDriverState::ReadyToSearch { .. }) {
            return Err(StopSearchError::NotSearching);
        }

        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        let (new_state, search_results) = match prior_state {
            SearchDriverState::Searching { ready_state: state, .. } => {
                let search_results = {
                    let ReadySearchState { result_receiver, thread_pool, halt_flag, .. } = &*state;
                    let num_threads = thread_pool.len();
                    println!("stopping active search");
                    // Tell the search threads to stop
                    halt_flag.halt();

                    // Collect all of the search results
                    let mut search_results = Vec::with_capacity(num_threads);
                    for _ in 0..num_threads {
                        let thread_search_result = result_receiver.recv().or(Err(StopSearchError::UnableToReceiveResults))?;
                        search_results.push(thread_search_result);
                    }

                    // Clear the halt flag since all of the searches have returned their results
                    halt_flag.reset();

                    // Find the best result by evaluation
                    search_results.into_iter()
                        .filter_map(|r| r.ok())
                        .max_by_key(|r| r.evaluation.unwrap_or(PositionEvaluation::MIN))
                        .ok_or(StopSearchError::NoResultsReturned)?
                };
                let new_state = SearchDriverState::ReadyToSearch { state };

                (new_state, search_results)
            },
            SearchDriverState::Intermediate => panic!("attempting to stop search from intermediate search driver state"),
            SearchDriverState::Idle { .. } | SearchDriverState::ReadyToSearch { .. } => unreachable!(),
        };
        self.state = new_state;

        Ok(search_results)
    }

    pub fn try_ponderhit(&mut self) -> Result<(), StartSearchError> {
        if matches!(self.state, SearchDriverState::Idle { .. } | SearchDriverState::ReadyToSearch { .. }) || matches!(self.state, SearchDriverState::Searching { is_pondering, .. } if is_pondering) {
            return Err(StartSearchError::NotPondering);
        }

        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Intermediate);
        self.state = match prior_state {
            SearchDriverState::Searching { ready_state, .. } => {
                // TODO: Stop the search?
                // TODO: make the move we were pondering
                // TODO: Start the search again
                SearchDriverState::Searching {
                    is_pondering: false,
                    ready_state,
                }
            },
            SearchDriverState::Intermediate => panic!("attempting to ponderhit search from intermediate search driver state"),
            SearchDriverState::Idle { .. } | SearchDriverState::ReadyToSearch { .. } => unreachable!(),
        };

        Ok(())
    }
}

impl Default for SearchDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SearchDriver {
    fn drop(&mut self) {
        // Nothing to cleanup if we're idle
        if let SearchDriverState::Idle { .. } = &self.state {
            return;
        }

        // If we're searching stop the search, then try to cleanup
        if let SearchDriverState::Searching { .. } = &self.state {
            let _ = self.try_stop();
        }

        let prior_state = std::mem::replace(&mut self.state, SearchDriverState::Idle { board: None, options: Default::default() });
        // Unnecessary match we're not idle or searching, but we need to destructure
        if let SearchDriverState::ReadyToSearch { state: box ReadySearchState { thread_pool, search_sender, ..} } = prior_state {
            // Drop the sender so the worker threads stop waiting for work
            drop(search_sender);
            println!("dropped sender, waiting for {} threads to join", thread_pool.len());
            // Join all of the worker threads
            let _ = thread_pool.into_iter().map(|t| t.join());
            println!("all threads joined successfully");
        }
    }
}

/// Run thermite search using a UCI engine driver based on stdin/stdout/stderr
fn main() {
    println!("{}", *STARTUP_TEXT);
    let uci_reader = UciReader::create(BufReader::new(std::io::stdin()));
    let uci_writer = UciWriter::create(std::io::stdout());
    start(uci_reader, uci_writer);
}

