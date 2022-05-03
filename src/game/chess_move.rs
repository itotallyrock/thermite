use crate::game::piece_type::PieceType;
use crate::game::square::SquareOffset;

pub struct SimpleChessMove {
    from: SquareOffset,
    to: SquareOffset,
    promotion: Option<PieceType>,
}