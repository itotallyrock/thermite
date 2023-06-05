use crate::direction::PawnCaptureDirection;
use crate::player_color::PlayerColor;
use crate::square::{DoublePawnToSquare, EnPassantSquare, Square};

/// A valid capture of a pawn on its skipped square for a pawn that *just* [double jumped](DoublePawnPush)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EnPassantCapture {
    /// The starting [`square`](Square) the piece is moving `from`
    from: DoublePawnToSquare,
    /// The ending [`en-passant square`](EnPassantSquare) the piece moving `to`
    to: EnPassantSquare,
    /// The square of the pawn that double-jumped
    captured_pawn_square: DoublePawnToSquare,
}

impl EnPassantCapture {
    /// Create a new valid [`EnPassantCapture`]
    /// Returns [`None`] if direction would shift off of the board.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(
        from: DoublePawnToSquare,
        direction: PawnCaptureDirection,
        player: PlayerColor,
    ) -> Option<Self> {
        let to = Square::from(from).shift(direction.to_sided_direction(player))?;
        let captured_pawn_direction = PawnCaptureDirection::get_pawn_push_for(player.switch());
        let captured_pawn_square = to.shift(captured_pawn_direction).unwrap();
        let to = EnPassantSquare::try_from(to).unwrap();
        let captured_pawn_square = DoublePawnToSquare::try_from(captured_pawn_square).unwrap();

        Some(Self {
            from,
            to,
            captured_pawn_square,
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
}
