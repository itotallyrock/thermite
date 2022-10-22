use crate::move_type::MoveType;
use crate::square::Square;

/// The move type and from/to squares for a chess move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ChessMove {
    /// Move metadata for each type of chess move
    pub move_type: MoveType,
    /// The starting square for the piece being moved
    pub from: Square,
    /// The ending square for the piece being moved
    pub to: Square,
}
