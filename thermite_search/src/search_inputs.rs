use std::sync::{Arc, RwLock};
use crate::halt_flag::HaltFlag;
use crate::move_ordering::MoveOrderingState;
use crate::transposition_table::TranspositionTable;

#[derive(Clone, Default, Debug)]
pub struct SearchInputs {
    /// The half search flag to periodically check
    pub halt_flag: Arc<HaltFlag>,// TODO: Make these private again and use builder or wrap in some service/factory
    #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "countermove_heuristic", feature = "piece_square_heuristic", feature = "static_exchange_eval"))]
    pub move_ordering_state: Arc<RwLock<MoveOrderingState>>,// TODO: Make these private again and use builder or wrap in some service/factory
    #[cfg(feature = "transposition_table")]
    pub transposition_table: Arc<RwLock<TranspositionTable>>,// TODO: Make these private again and use builder or wrap in some service/factory
    // TODO: Other table references (zobrist, pawn key, material key, etc)
}
