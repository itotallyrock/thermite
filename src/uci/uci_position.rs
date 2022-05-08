use crate::engine_types::SimpleMoveList;

/// The initial position
pub enum InitialPosition {
    /// FIDE standard chess starting position
    StandardStartingPosition,
    /// Custom FEN string starting position
    Fen(String),
}

/// Position set from UCI command
pub struct UciPosition {
    /// The starting position
    initial_position: InitialPosition,
    /// Optional number of moves to apply to the initial position
    moves: Option<SimpleMoveList>,
}