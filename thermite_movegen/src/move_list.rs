use arrayvec::ArrayVec;
use thermite_core::chess_move::ChessMove;
use crate::pseudo_legal_move::PseudoLegalChessMove;

const MOVE_LIST_CAPACITY: usize = 250;

/// TODO
pub type PseudoLegalMoveContainer = ArrayVec<PseudoLegalChessMove, MOVE_LIST_CAPACITY>;
/// TODO
pub type LegalMoveContainer = ArrayVec<ChessMove, MOVE_LIST_CAPACITY>;