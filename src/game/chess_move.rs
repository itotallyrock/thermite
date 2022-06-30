use crate::game::piece_type::PromotionPieceType;
use crate::game::square::SquareOffset;
use std::fmt::{Display, Formatter};

/// Basic stateless chess move. Only contains the minimum amount of information to relay a move.
/// Has a starting square, a target square, and optionally if promoting a pawn, which piece.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SimpleChessMove {
    /// The square the piece being moved is starting on
    pub from: SquareOffset,
    /// The square the piece is moved to
    pub to: SquareOffset,
    /// If the piece was a promoting pawn, then the promotion piece type.
    pub promotion: Option<PromotionPieceType>,
}

impl SimpleChessMove {
    /// Create a new basic chess move, for a given from-square and to-square.
    /// This move isn't a promotion, and can be an illegal move.
    ///
    /// * `from` - The origin square for the piece moving
    /// * `to` - The destination square for the piece moving
    pub fn new(from: SquareOffset, to: SquareOffset) -> Self {
        Self {
            from,
            to,
            promotion: None,
        }
    }

    /// Create a new basic promoting chess move, for a given from-square and to-square promoting to
    /// a specific piece. This move is a promotion, but can be an illegal move.
    ///
    /// * `from` - The origin square for the piece moving
    /// * `to` - The destination square for the piece moving
    /// * `promotion` - The piece this pawn should promote into
    pub fn new_promotion(
        from: SquareOffset,
        to: SquareOffset,
        promotion: PromotionPieceType,
    ) -> Self {
        Self {
            from,
            to,
            promotion: Some(promotion),
        }
    }
}

impl Display for SimpleChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.promotion {
            None => write!(f, "{}{}", self.from, self.to),
            Some(promotion) => write!(f, "{}{}{}", self.from, self.to, promotion),
        }
    }
}
