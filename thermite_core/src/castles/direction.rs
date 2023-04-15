
/// How many directions the king can castle in
pub const NUM_CASTLE_DIRECTIONS: usize = 2;

/// The direction to castle in for either side
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CastleDirection {
    /// Castle with the rook on the same side as the king
    KingSide,
    /// Castle with the rook on the same side as the queen
    QueenSide,
}