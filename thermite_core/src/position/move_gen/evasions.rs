use crate::bitboard::BoardMask;
use crate::chess_move::ChessMove;
use crate::position::LegalPosition;

impl LegalPosition {
    /// Generate any legal moves that will get the king out of check
    pub(super) fn generate_evasion_moves(&self) -> impl Iterator<Item = ChessMove> + '_ {
        self.generate_king_evasions()
            .chain(self.get_block_evasion_moves())
    }

    /// Generate legal non-castling king moves to any other square to attempt get out of the check
    fn generate_king_evasions(&self) -> impl Iterator<Item = ChessMove> + '_ {
        self.generate_non_castling_king_moves(self.attackable_mask())
    }

    /// Generate legal non-king evasion moves which can block or capture a single checking piece
    fn get_block_evasion_moves(&self) -> impl Iterator<Item = ChessMove> + '_ {
        let king_square = self.king_squares[self.player_to_move];
        let blockable_checker_square = (self.state.checkers.num_squares() == 1).then(|| {
            self.state
                .checkers
                .clone()
                .pop_square()
                .expect("one bit is set based on condition")
        });
        blockable_checker_square
            .into_iter()
            .map(move |checker_square| BoardMask::line_between(king_square, checker_square))
            .flat_map(|blockable_squares| self.generate_non_king_moves(blockable_squares))
    }
}
