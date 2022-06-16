use std::fmt::{Display, Formatter};
use std::time::Duration;

use crate::engine_types::{NodeCount, PvCount, SearchDepth, SimpleMoveList, TableBaseHits};
use crate::game::SimpleChessMove;
use crate::uci::UciScore;

/// An informational message that can be sent to a UCI compatible GUI and processed to show the
/// engine's current state.  State like: how many moves have been search, for how long, what is the
/// best move so far, the evaluation for the current position, and more.
///
/// The [UCI spec](https://backscattering.de/chess/uci/#engine-info) outlines available info types.
#[derive(Default, Debug, Clone)]
pub struct UciInfoResponse {
    /// Raw message
    string: Option<String>,
    /// How many nodes per second the engine is covering
    nodes_per_second: Option<NodeCount>,
    /// The root move currently being explored
    current_move: Option<SimpleChessMove>,
    /// The current root move index, starting at 1
    current_move_number: Option<usize>,
    /// How many moves ahead the search has looked
    search_depth: Option<SearchDepth>,
    /// If searching selectively deeper, to what depth
    selective_search_depth: Option<SearchDepth>,
    /// How much time has been spent searching
    time_searched: Option<Duration>,
    /// The number of chess positions (nodes) searched
    nodes_searched: Option<NodeCount>,
    /// The current best line
    principle_variation: Option<SimpleMoveList>,
    /// Multi-PV Nth best variation index
    multi_pv_index: Option<PvCount>,
    /// Evaluation for the current move
    evaluation: Option<UciScore>,
    /// Hash table usage
    hash_table_usage: Option<f64>,
    /// The current CPU's average usage across all cores
    cpu_usage: Option<f64>,
    /// End-game table-base hits
    endgame_table_base_hits: Option<TableBaseHits>,
}

impl UciInfoResponse {
    /// Create a new UciInfo message without any information included
    pub fn new() -> Self {
        Self::default()
    }

    /// Print a custom string message
    ///
    /// * `message` - The custom info message to send
    pub fn with_message(mut self, message: String) -> Self {
        self.string = Some(message);
        self
    }

    /// Inform the GUI how many positions this engine is covering in one second.
    ///
    /// * `nodes_per_second` - How many positions (nodes) this engine is searching per second (rounded)
    pub fn with_nodes_per_second(mut self, nodes_per_second: NodeCount) -> Self {
        self.nodes_per_second = Some(nodes_per_second);
        self
    }

    /// Express how many positions have been checked so far
    ///
    /// * `nodes_searched` - How many positions (nodes) have been searched in this search so far
    pub fn with_nodes_searched(mut self, nodes_searched: NodeCount) -> Self {
        self.nodes_searched = Some(nodes_searched);
        self
    }

    /// Add information for the evaluation of the current position in the search so far.
    ///
    /// * `evaluation` - The evaluation of the current position
    pub fn with_evaluation(mut self, evaluation: UciScore) -> Self {
        self.evaluation = Some(evaluation);
        self
    }

    /// Include information about what move is currently being searched.
    /// A root move to inform the GUI which moves are being checked.
    ///
    /// * `current_move` - The current move being searched
    /// * `move_index` - The current move's search index (zero indexed)
    pub fn with_current_move(mut self, current_move: SimpleChessMove, move_index: usize) -> Self {
        self.current_move = Some(current_move);
        self.current_move_number = Some(move_index + 1);
        self
    }

    /// Include information on how many endgame positions were avoided because of a table database.
    ///
    /// * `endgame_table_base_hits` - How many positions searched were in the endgame table database
    pub fn with_table_base_hits(mut self, endgame_table_base_hits: TableBaseHits) -> Self {
        self.endgame_table_base_hits = Some(endgame_table_base_hits);
        self
    }

    /// Tell the GUI how full the engine's hashtable is.
    ///
    /// * `hash_table_usage` - The engine's filled percentage of its entire hash table
    pub fn with_hash_table_usage(mut self, hash_table_usage: f64) -> Self {
        self.hash_table_usage = Some(hash_table_usage);
        self
    }

    /// Tell the GUI what percentage of the CPU it's utilizing.
    ///
    /// * `hash_table_usage` - The engine's filled percentage of its entire hash table
    pub fn with_cpu_usage(mut self, cpu_usage: f64) -> Self {
        self.cpu_usage = Some(cpu_usage);
        self
    }

    /// Tell the GUI what the current search depth is
    ///
    /// * `search_depth` - How many moves deep have been checked for the current search
    pub fn with_search_depth(mut self, search_depth: SearchDepth) -> Self {
        self.search_depth = Some(search_depth);
        self
    }

    /// Tell the GUI what the current search depth a selective depth for the current move.
    ///
    /// * `selective_search_depth` - How many moves deep have been checked for the current move
    /// * `search_depth` - How many moves deep have been checked for the current search
    pub fn with_selective_search_depth(
        mut self,
        selective_search_depth: SearchDepth,
        search_depth: SearchDepth,
    ) -> Self {
        self.selective_search_depth = Some(selective_search_depth);
        self.search_depth = Some(search_depth);
        self
    }

    /// Give the best line to the GUI.
    ///
    /// * `search_duration` - How long it took to arrive at this principle variation
    /// * `principle_variation` - The line of moves that the engine thinks will end with the highest evaluation for the side to move.
    pub fn with_principle_variation(
        mut self,
        search_duration: Duration,
        principle_variation: SimpleMoveList,
    ) -> Self {
        self.time_searched = Some(search_duration);
        self.principle_variation = Some(principle_variation);
        self
    }

    /// Give the Nth best move line to the GUI.
    ///
    /// * `search_duration` - How long it took to arrive at this principle variation
    /// * `variation` - The line of moves that the engine thinks will end with the Nth highest evaluation for the side to move.
    /// * `multi_pv_index` - The index for this variation
    pub fn with_multi_principle_variation(
        mut self,
        search_duration: Duration,
        variation: SimpleMoveList,
        multi_pv_index: PvCount,
    ) -> Self {
        self.time_searched = Some(search_duration);
        self.principle_variation = Some(variation);
        self.multi_pv_index = Some(multi_pv_index);
        self
    }
}

impl Display for UciInfoResponse {
    /// Create the printable info response
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            string,
            nodes_per_second,
            current_move,
            current_move_number,
            search_depth,
            selective_search_depth,
            time_searched,
            nodes_searched,
            principle_variation,
            multi_pv_index,
            evaluation,
            hash_table_usage,
            cpu_usage,
            endgame_table_base_hits,
        } = self;

        if let Some(current_move) = current_move {
            write!(f, " currmove {}", current_move)?;
        }

        if let Some(current_move_number) = current_move_number {
            write!(f, " currmovenumber {}", current_move_number)?;
        }

        if let Some(search_depth) = search_depth {
            write!(f, " depth {}", search_depth)?;
        }

        if let Some(selective_search_depth) = selective_search_depth {
            write!(f, " seldepth {}", selective_search_depth)?;
        }

        if let Some(time_searched) = time_searched {
            write!(f, " time {}", time_searched.as_millis())?;
        }

        if let Some(nodes_searched) = nodes_searched {
            write!(f, " nodes {}", nodes_searched)?;
        }

        if let Some(principle_variation) = principle_variation {
            write!(f, " pv")?;
            for pv_move in principle_variation {
                write!(f, " {}", pv_move)?;
            }
        }

        if let Some(multi_pv_index) = multi_pv_index {
            debug_assert!(
                principle_variation.is_some(),
                "Cannot send multipv without a pv"
            );
            write!(f, " multipv {}", *multi_pv_index as u8)?;
        }

        if let Some(evaluation) = evaluation {
            write!(f, " score {}", evaluation)?;
        }

        if let Some(hash_table_usage) = hash_table_usage {
            debug_assert!(
                *hash_table_usage <= 1.0 && *hash_table_usage >= 0.0,
                "Hash table usage must be between 0 and 1"
            );
            write!(f, " hashfull {}", (hash_table_usage * 1000.0) as u32)?;
        }

        if let Some(cpu_usage) = cpu_usage {
            debug_assert!(
                *cpu_usage <= 1.0 && *cpu_usage >= 0.0,
                "CPU usage must be between 0 and 1"
            );
            write!(f, " cpuload {}", (cpu_usage * 1000.0) as u32)?;
        }

        if let Some(endgame_table_base_hits) = endgame_table_base_hits {
            write!(f, " tbhits {}", endgame_table_base_hits)?;
        }

        if let Some(nodes_per_second) = nodes_per_second {
            write!(f, " nps {}", nodes_per_second)?;
        }

        // String must be last
        if let Some(string) = string {
            write!(f, " string {}", string)?;
        }

        Ok(())
    }
}
