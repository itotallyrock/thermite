use std::fmt::{Display, Formatter};
use std::num::{NonZeroU8, NonZeroUsize};
use std::time::Duration;

use crate::engine_types::{NodeCount, SearchDepth, SimpleMoveList, TableBaseHits};
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
    current_move_number: Option<NonZeroUsize>,
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
    multi_pv_index: Option<NonZeroU8>,
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
        debug_assert!(
            move_index < usize::MAX,
            "currmovenumber would saturate at usize::MAX"
        );
        self.current_move = Some(current_move);
        self.current_move_number = Some(NonZeroUsize::new(move_index.saturating_add(1)).unwrap());
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
        multi_pv_index: u8,
    ) -> Self {
        debug_assert!(!variation.is_empty(), "Cannot print an empty PV");
        debug_assert!(
            multi_pv_index < u8::MAX,
            "MultiPV would saturate at u8::MAX"
        );
        self.time_searched = Some(search_duration);
        self.principle_variation = Some(variation);
        self.multi_pv_index = Some(NonZeroU8::new(multi_pv_index.saturating_add(1)).unwrap());
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

        if let Some(nodes_per_second) = nodes_per_second {
            write!(f, " nps {}", nodes_per_second)?;
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
            write!(f, " multipv {}", multi_pv_index)?;
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

        // String must be last because it can contain "string" which would be recursive
        if let Some(string) = string {
            write!(f, " string {}", string)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::engine_types::Score;
    use crate::game::SquareOffset;
    use crate::uci::ScoreBoundsType;

    use super::*;

    #[test]
    fn string_works() {
        const MESSAGE: &str = "The Ther-Mitey Quinn";
        let uci_info_response = UciInfoResponse::new().with_message(MESSAGE.to_owned());
        assert_eq!(
            uci_info_response.to_string(),
            format!(" string {}", MESSAGE)
        );
    }

    #[test]
    fn is_empty_by_default() {
        let uci_info_response = UciInfoResponse::new();
        assert_eq!(uci_info_response.to_string().as_str(), "");
    }

    #[test]
    fn string_is_always_last() {
        const MESSAGE: &str = "Oxide was a pretty cool engine";
        let uci_info_response = UciInfoResponse::new()
            .with_message(MESSAGE.to_owned())
            .with_nodes_per_second(NodeCount::new(623233).unwrap())
            .with_nodes_searched(NodeCount::new(1404992).unwrap())
            .with_evaluation(UciScore::new(Score::Centipawns(30), ScoreBoundsType::Exact))
            .with_current_move(SimpleChessMove::new(SquareOffset::A1, SquareOffset::A2), 0)
            .with_table_base_hits(312)
            .with_hash_table_usage(0.01)
            .with_cpu_usage(0.282828)
            .with_selective_search_depth(
                SearchDepth::new(42).unwrap(),
                SearchDepth::new(23).unwrap(),
            )
            .with_multi_principle_variation(
                Duration::from_millis(13742),
                [SimpleChessMove::new(SquareOffset::C5, SquareOffset::H5)]
                    .into_iter()
                    .collect(),
                1.try_into().unwrap(),
            );

        let expected = format!(" string {}", MESSAGE);
        assert!(uci_info_response.to_string().ends_with(&expected));
    }

    #[test]
    fn hash_table_usage_is_permil() {
        const PERCENT: f64 = 0.42;
        let uci_info_response = UciInfoResponse::new().with_hash_table_usage(PERCENT);

        let expected = format!(" hashfull {}", (PERCENT * 1000.0) as u32);
        assert_eq!(uci_info_response.to_string(), expected);
    }

    #[test]
    fn cpu_usage_is_permil() {
        const PERCENT: f64 = 0.31415;
        let uci_info_response = UciInfoResponse::new().with_cpu_usage(PERCENT);

        let expected = format!(" cpuload {}", (PERCENT * 1000.0) as u32);
        assert_eq!(uci_info_response.to_string(), expected);
    }

    #[test]
    fn current_move_index_is_one_indexed() {
        const MOVE_INDEX: usize = 0;
        let uci_info_response = UciInfoResponse::new().with_current_move(
            SimpleChessMove::new(SquareOffset::E2, SquareOffset::E4),
            MOVE_INDEX,
        );

        let expected = format!(" currmovenumber {}", MOVE_INDEX + 1);
        assert!(uci_info_response.to_string().contains(&expected));
    }

    #[test]
    fn multi_pv_index_is_one_indexed() {
        const VARIATION_INDEX: u8 = 2;
        let uci_info_response = UciInfoResponse::new().with_multi_principle_variation(
            Duration::from_millis(9001),
            [SimpleChessMove::new(SquareOffset::E2, SquareOffset::E4)]
                .into_iter()
                .collect(),
            VARIATION_INDEX,
        );

        let expected = format!(" multipv {}", VARIATION_INDEX + 1);
        assert!(uci_info_response.to_string().contains(&expected));
    }

    #[test]
    fn selective_search_depth_shadows_search_depth() {
        let shadowed_depth = SearchDepth::new(23).unwrap();
        let depth = SearchDepth::new(25).unwrap();
        let selective_depth = SearchDepth::new(32).unwrap();
        assert_ne!(shadowed_depth, depth);
        let uci_info_response = UciInfoResponse::new()
            .with_search_depth(shadowed_depth)
            .with_selective_search_depth(selective_depth, depth);

        let result = uci_info_response.to_string();
        assert!(!result.contains(&format!(" depth {}", shadowed_depth)));
        assert!(result.contains(&format!(" depth {}", depth)));
        assert!(result.contains(&format!(" seldepth {}", selective_depth)));
    }

    #[test]
    fn multiple_pvs_shadows_pv() {
        const PV_INDEX: u8 = 3;
        const SHADOWED_DURATION: Duration = Duration::from_millis(3913);
        const DURATION: Duration = Duration::from_millis(20123);
        assert_ne!(SHADOWED_DURATION, DURATION);
        let shadowed_variation: SimpleMoveList = [SimpleChessMove::new(SquareOffset::A4, SquareOffset::B6)].into_iter().collect();
        let variation: SimpleMoveList = [SimpleChessMove::new(SquareOffset::C5, SquareOffset::G3)].into_iter().collect();
        assert_ne!(shadowed_variation, variation);

        let uci_info_response = UciInfoResponse::new()
            .with_principle_variation(SHADOWED_DURATION, shadowed_variation.clone())
            .with_multi_principle_variation(DURATION, variation.clone(), PV_INDEX);

        let result = uci_info_response.to_string();
        assert!(!result.contains(&format!(" time {}", SHADOWED_DURATION.as_millis())));
        assert!(result.contains(&format!(" time {}", DURATION.as_millis())));
        assert!(!result.contains(&format!(" pv {}", shadowed_variation.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(" "))));
        assert!(result.contains(&format!(" pv {}", variation.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(" "))));
    }
}
