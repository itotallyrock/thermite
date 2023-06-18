use crate::chess_move::quiet::QuietMove;
use crate::pieces::NonKingPieceType;
use crate::square::Square;

/// A valid capture move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct Capture {
    /// The inner quiet move
    pub(crate) quiet: QuietMove,
    /// The piece being captured
    pub(crate) captured_piece: NonKingPieceType,
}

impl Capture {
    /// Get the starting square for the piece doing the capturing
    #[must_use]
    pub const fn from(&self) -> Square {
        self.quiet.from()
    }

    /// Get the destination [`Square`], occupied by the [captured piece](NonKingPieceType)
    #[must_use]
    pub const fn to(&self) -> Square {
        self.quiet.to()
    }

    /// Get the [piece](NonKingPieceType) being captured
    #[must_use]
    pub const fn captured_piece(&self) -> NonKingPieceType {
        self.captured_piece
    }
}

impl From<Capture> for QuietMove {
    fn from(capture: Capture) -> Self {
        capture.quiet
    }
}
