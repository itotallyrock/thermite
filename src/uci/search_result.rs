use crate::game::SimpleChessMove;

/// The result from searching a position, contains the engine's perceived best move and optionally
/// what it believes the opponent will play next.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SearchResult {
    /// The current best move
    pub best_move: SimpleChessMove,
    /// Expected opponent's response to the current best move
    pub ponder_move: Option<SimpleChessMove>,
}
