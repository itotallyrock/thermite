use crate::pieces::PieceType;
use crate::square::Square;

/// Plain chess move, take a piece from a square and move it to another square
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QuietMove {
    /// The starting [`square`](Square) the piece is moving `from`
    from: Square,
    /// The ending [`square`](Square) the piece moving `to`
    to: Square,
    /// The piece moving
    piece_type: PieceType,
}

impl QuietMove {
    /// Create a new valid [`QuietMove`]
    /// Returns [`None`] if `from` and `to` are the same
    #[must_use]
    pub fn new(from: Square, to: Square, piece_type: PieceType) -> Option<Self> {
        // If moving to the same starting square this invalid
        if from == to {
            return None;
        }

        Some(Self {
            from,
            to,
            piece_type,
        })
    }

    /// Get the piece's original [square](Square)
    #[must_use]
    pub const fn from(&self) -> Square {
        self.from
    }

    /// Get the piece's ending/destination [square](Square)
    #[must_use]
    pub const fn to(&self) -> Square {
        self.to
    }
}
