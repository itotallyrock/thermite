use crate::pieces::OwnedPiece;
use crate::square::Square;

/// Plain chess move, take a piece from a square and move it to another square
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Quiet {
    /// The starting [`square`](Square) the piece is moving `from`
    from: Square,
    /// The ending [`square`](Square) the piece moving `to`
    to: Square,
    /// The [`OwnedPiece`] moving
    owned_piece: OwnedPiece,
}

impl Quiet {
    /// Create a new valid [`Quiet`]
    /// Returns [`None`] if `from` and `to` are the same
    #[must_use]
    pub(crate) fn new(from: Square, to: Square, owned_piece: OwnedPiece) -> Option<Self> {
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

    /// Get a new [`Quiet`] with the `from` and `to` [`Square`]s switched
    /// Useful for undoing a move.
    #[must_use]
    pub const fn reverse(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            owned_piece: self.owned_piece,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chess_move::quiet::Quiet;
    use crate::pieces::{OwnedPiece, Piece, PieceType::*};
    use crate::player_color::PlayerColor::{Black, White};
    use crate::square::{Square, Square::*};
    use test_case::test_case;

    #[test_case(Pawn.owned_by(White), A4, A6)]
    #[test_case(Pawn.owned_by(Black), B7, H1)]
    fn reverse_works(piece: OwnedPiece, from: Square, to: Square) {
        assert_eq!(
            Quiet::new(from, to, piece).unwrap().reverse(),
            Quiet::new(to, from, piece).unwrap()
        );
    }
}
