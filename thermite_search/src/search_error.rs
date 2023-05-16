
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RootSearchError {
    IllegalPosition,
    NoResultsReturned,
    FailedToReadTT,
    FailedToWriteTT,
    #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "static_exchange_eval", feature = "countermove_heuristic"))]
    FailedToReadMoveOrdering,
    #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "static_exchange_eval", feature = "countermove_heuristic"))]
    FailedToWriteMoveOrdering,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SearchError {
    Halted,
    #[cfg(feature = "transposition_table")]
    FailedToReadTT,
    #[cfg(feature = "transposition_table")]
    FailedToWriteTT,
    #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "static_exchange_eval", feature = "countermove_heuristic"))]
    FailedToReadMoveOrdering,
    #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "static_exchange_eval", feature = "countermove_heuristic"))]
    FailedToWriteMoveOrdering,
}

