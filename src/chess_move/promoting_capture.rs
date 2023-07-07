use crate::chess_move::promotion::Promotion;
use crate::pieces::NonKingPieceType;
use crate::square::Square;

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
    ///
    /// # Panics
    /// - In debug mode when trying to create a new promoting capture with a pawn that doesn't attack diagonally
    pub(crate) fn new(promotion: Promotion, captured_piece: NonKingPieceType) -> Self {
        debug_assert_ne!(
            Square::from(promotion.from()).file(),
            Square::from(promotion.to()).file(),
            "attempting to create `new` `PromotingCapture` with a non-capturing `Promotion`"
        );
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
