use std::fmt::{Display, Formatter};
use crate::game::piece_type::{PromotionPieceType};
use crate::game::square::SquareOffset;

/// Basic stateless chess move. Only contains the minimum amount of information to relay a move.
/// Has a starting square, a target square, and optionally if promoting a pawn, which piece.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SimpleChessMove {
    /// The square the piece being moved is starting on
    from: SquareOffset,
    /// The square the piece is moved to
    to: SquareOffset,
    /// If the piece was a promoting pawn, then the promotion piece type.
    promotion: Option<PromotionPieceType>,
}

impl Display for SimpleChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.promotion {
            None => write!(f, "{}{}", self.from, self.to),
            Some(promotion) => write!(f, "{}{}{}", self.from, self.to, promotion),
        }
    }
}