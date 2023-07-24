use crate::chess_move::quiet::Quiet;
use crate::pieces::NonKingPieceType;
use crate::square::Square;

/// A valid capture move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct Capture {
    /// The inner quiet move
    quiet: Quiet,
    /// The piece being captured
    captured_piece: NonKingPieceType,
}

impl Capture {
    /// Create new capturing move given a quiet move and captured piece
    pub(crate) const fn new(quiet: Quiet, captured_piece: NonKingPieceType) -> Self {
        Self {
            quiet,
            captured_piece,
        }
    }
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

impl From<Capture> for Quiet {
    fn from(capture: Capture) -> Self {
        capture.quiet
    }
}
