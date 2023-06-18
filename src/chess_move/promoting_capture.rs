use crate::chess_move::promotion::Promotion;
use crate::pieces::NonKingPieceType;

/// A valid double-pawn push, or a special starting rank unobstructed two square pawn push
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PromotingCapture {
    /// The inner promotion
    promotion: Promotion,
    /// The piece being captured
    captured_piece: NonKingPieceType,
}

impl PromotingCapture {
    /// Create a new promoting capture
    #[cfg(test)]
    pub(crate) fn new(promotion: Promotion, captured_piece: NonKingPieceType) -> Self {
        Self {
            promotion,
            captured_piece,
        }
    }

    /// Get the [piece](NonKingPieceType) that was captured
    #[must_use]
    pub const fn captured_piece(&self) -> NonKingPieceType {
        self.captured_piece
    }

    /// Get the [promotion](Promotion) details
    pub const fn promotion(&self) -> Promotion {
        self.promotion
    }
}
