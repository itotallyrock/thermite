use crate::pieces::OwnedPiece;
use crate::square::Square;

/// Plain chess move, take a piece from a square and move it to another square
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct QuietMove {
    /// The starting [`square`](Square) the piece is moving `from`
    from: Square,
    /// The ending [`square`](Square) the piece moving `to`
    to: Square,
    /// The [`OwnedPiece`] moving
    owned_piece: OwnedPiece,
}

impl QuietMove {
    /// Create a new valid [`QuietMove`]
    /// Returns [`None`] if `from` and `to` are the same
    #[must_use]
    pub fn new(from: Square, to: Square, owned_piece: OwnedPiece) -> Option<Self> {
        // If moving to the same starting square this invalid
        if from == to {
            return None;
        }

        Some(Self {
            from,
            to,
            owned_piece,
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

    /// Get the [piece](OwnedPiece) being moves
    #[must_use]
    pub const fn piece(&self) -> OwnedPiece {
        self.owned_piece
    }

    /// Get a new [`QuietMove`] with the `from` and `to` [`Square`]s switched
    /// Useful for undoing a move.
    ///
    /// ```
    /// use thermite::chess_move::quiet::QuietMove;
    /// use thermite::pieces::{Piece, PieceType};
    /// use thermite::player_color::PlayerColor;
    /// use thermite::square::Square::*;
    ///
    /// let piece = PieceType::Pawn.owned_by(PlayerColor::White);
    /// assert_eq!(QuietMove::new(A4, A6, piece).unwrap().reverse(), QuietMove::new(A6, A4, piece).unwrap());
    /// ```
    #[must_use]
    pub const fn reverse(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            owned_piece: self.owned_piece,
        }
    }
}
