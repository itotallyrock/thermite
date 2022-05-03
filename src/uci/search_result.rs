use crate::game::SimpleChessMove;

pub struct SearchResult {
    best_move: SimpleChessMove,
    ponder_move: Option<SimpleChessMove>,
}