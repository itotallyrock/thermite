use crate::chess_move::quiet::Quiet;
use crate::direction::PawnCaptureDirection;
use crate::pieces::{Piece, PieceType};
use crate::player_color::PlayerColor;
use crate::square::{DoublePawnToSquare, EnPassantSquare, Square};

/// A valid capture of a pawn on its skipped square for a pawn that *just* [double jumped](crate::chess_move::DoublePawnPush)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EnPassantCapture {
    /// The starting [`square`](Square) the piece is moving `from`
    from: DoublePawnToSquare,
    /// The ending [`en-passant square`](EnPassantSquare) the piece moving `to`
    to: EnPassantSquare,
    /// The square of the pawn that double-jumped
    captured_pawn_square: DoublePawnToSquare,
    /// The player doing the en-passant-capture
    player: PlayerColor,
}

impl EnPassantCapture {
    /// Create a new valid [`EnPassantCapture`] given the capturing [pawn](PieceType::Pawn)'s starting [square](Square)
    /// Returns [`None`] if direction would shift off of the board.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    #[cfg(test)]
    pub(crate) fn new_from(
        from: DoublePawnToSquare,
        direction: PawnCaptureDirection,
        player: PlayerColor,
    ) -> Option<Self> {
        let to = Square::from(from).shift(direction.to_sided_direction(player))?;
        let captured_pawn_direction = PawnCaptureDirection::get_pawn_push_for(player).opposite();
        let captured_pawn_square = to.shift(captured_pawn_direction).unwrap();
        let to = EnPassantSquare::try_from(to).unwrap();
        let captured_pawn_square = DoublePawnToSquare::try_from(captured_pawn_square).unwrap();

        Some(Self {
            from,
            to,
            captured_pawn_square,
            player,
        })
    }

    /// Create a new valid [`EnPassantCapture`] given the target [en-passant square](EnPassantSquare) (or the capturing [pawn](PieceType::Pawn)'s destination [square](Square))
    /// Returns [`None`] if direction would shift off of the board.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub(crate) fn new_en_passant_square(
        to: EnPassantSquare,
        direction: PawnCaptureDirection,
        player: PlayerColor,
    ) -> Option<Self> {
        let from = Square::from(to).shift(direction.to_sided_direction(player).opposite())?;
        let from = DoublePawnToSquare::try_from(from).unwrap();
        let captured_pawn_direction = PawnCaptureDirection::get_pawn_push_for(player).opposite();
        let captured_pawn_square = Square::from(to).shift(captured_pawn_direction).unwrap();
        let captured_pawn_square = DoublePawnToSquare::try_from(captured_pawn_square).unwrap();

        Some(Self {
            from,
            to,
            captured_pawn_square,
            player,
        })
    }

    /// Get the starting pawn [square](DoublePawnToSquare)
    #[must_use]
    pub const fn from(&self) -> DoublePawnToSquare {
        self.from
    }

    /// Get the ending pawn [square](EnPassantSquare)
    #[must_use]
    pub const fn to(&self) -> EnPassantSquare {
        self.to
    }

    /// Get the player doing the en-passant-capture
    #[must_use]
    pub const fn player(&self) -> PlayerColor {
        self.player
    }

    /// Get the captured pawn's square (the en-passant-square)
    #[must_use]
    pub const fn captured_square(&self) -> DoublePawnToSquare {
        self.captured_pawn_square
    }
}

impl From<EnPassantCapture> for Quiet {
    fn from(value: EnPassantCapture) -> Self {
        Self::new(
            value.from().into(),
            value.to().into(),
            PieceType::Pawn.owned_by(value.player()),
        )
        .expect("EnPassantCapture shouldn't have the same from and to squares")
    }
}
