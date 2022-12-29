
/// How many directions the king can castle in
pub const NUM_CASTLE_DIRECTIONS: usize = 2;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CastleDirection {
    /// Castle with the rook on the same side as the king
    KingSide,
    /// Castle with the rook on the same side as the queen
    QueenSide,
}