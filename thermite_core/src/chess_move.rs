use std::fmt::{Display, Formatter};
use crate::move_type::MoveType;
use crate::piece_type::PieceType;
use crate::square::Square;

/// The move type and from/to squares for a chess move
#[cfg_attr(feature = "hashable_chess_move", derive(Hash))]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ChessMove {
    /// Move metadata for each type of chess move
    pub move_type: MoveType,
    /// The starting square for the piece being moved
    pub from: Square,
    /// The ending square for the piece being moved
    pub to: Square,
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let &Self { from, to, move_type } = self;
        match move_type {
            MoveType::Quiet { .. } | MoveType::DoublePawnPush { .. } | MoveType::Capture { .. } | MoveType::EnPassantCapture { .. } | MoveType::Castle { .. } => {
                write!(f, "{from}{to}")
            },
            MoveType::PromotingCapture { promotion, .. } | MoveType::Promotion { promotion } => {
                let promotion = PieceType::from(promotion).get_lower_char();
                write!(f, "{from}{to}{promotion}")
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_of_chess_move_is_consistent() {
        assert_eq!(std::mem::size_of::<ChessMove>(), 5);
    }
}