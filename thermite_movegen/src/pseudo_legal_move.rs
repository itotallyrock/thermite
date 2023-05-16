use thermite_core::bitboard::Bitboard;
use thermite_core::board::Board;
use thermite_core::chess_move::ChessMove;
use thermite_core::move_type::MoveType;
use thermite_core::piece_type::PieceType;
use thermite_core::player::Player;
use thermite_core::square::Square;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PseudoLegalChessMove(ChessMove);

impl PseudoLegalChessMove {
    pub const fn new(potentially_illegal_move: ChessMove) -> Self {
        PseudoLegalChessMove(potentially_illegal_move)
    }

    fn is_illegal_pin(from: Square, to: Square, pinned: Bitboard, king_square: Square) -> bool {
        from.to_mask() & pinned != Bitboard::EMPTY && !Bitboard::is_aligned(from, to, king_square)
    }

    fn is_illegal_king_move(board: &Board, from: Square, to: Square, side: Player, king_square: Square) -> bool {
        from == king_square && board.attackers_to(to, board.occupied() ^ from.to_mask()) & board.side_mask(side.switch()) != Bitboard::EMPTY
    }

    fn is_illegal_en_passant_capture(board: &Board, pseudo_legal: ChessMove, king_square: Square, side: Player) -> bool {
        if let MoveType::EnPassantCapture { captured_pawn_square } = pseudo_legal.move_type {
            let opposite_side = side.switch();
            let enemies = board.side_mask(opposite_side);
            let occupied = board.occupied() ^ pseudo_legal.from.to_mask() ^ captured_pawn_square.to_mask() | pseudo_legal.to.to_mask();
            let queens = board.piece_mask(PieceType::Queen);
            let cardinal_sliders = board.piece_mask(PieceType::Rook) | queens;
            let ordinal_sliders = board.piece_mask(PieceType::Bishop) | queens;

            Bitboard::occluded_attacks_mask(PieceType::Rook, king_square, occupied) & cardinal_sliders & enemies != Bitboard::EMPTY
                || Bitboard::occluded_attacks_mask(PieceType::Bishop, king_square, occupied) & ordinal_sliders & enemies != Bitboard::EMPTY
        } else {
            false
        }
    }

    fn is_illegal_castle(board: &Board, pseudo_legal: ChessMove, side: Player) -> bool {
        match pseudo_legal.move_type {
            MoveType::Castle { castle_direction } => {
                // Check that no squares the king passes through are attacked
                let attackers = board.side_mask(side.switch());
                let mut castle_path = board.as_ref().castles().get_unattacked_path(side, castle_direction);
                while let Some(pass_square) = castle_path.pop_square() {
                    if board.attackers_to(pass_square, board.occupied()) & attackers != Bitboard::EMPTY {
                        return true;
                    }
                }

                // TODO: If chess960 check that the rook wasn't a blocker

                false
            },
            _ => false,
        }
    }

    /// TODO
    pub fn into_legal(self, board: &Board) -> Option<ChessMove> {
        let pseudo_legal = self.0;
        debug_assert!(pseudo_legal.from != pseudo_legal.to);
        debug_assert!(board.piece_on(pseudo_legal.from).is_some());
        debug_assert!(board.side_on(pseudo_legal.from).unwrap() == board.side_to_move());

        let side = board.side_to_move();
        let pinned = board.as_ref().blockers_for(side) & board.side_mask(side);
        let king_square = board.king_square(side).expect("missing king in legal position");

        if Self::is_illegal_pin(pseudo_legal.from, pseudo_legal.to, pinned, king_square)
            || Self::is_illegal_king_move(board, pseudo_legal.from, pseudo_legal.to,side, king_square)
            || Self::is_illegal_en_passant_capture(board, pseudo_legal, king_square, side)
            || Self::is_illegal_castle(board, pseudo_legal, side)
        {
            return None;
        }

        Some(pseudo_legal)
    }
}

#[cfg(test)]
mod test {
    use super::*;


    use test_case::test_case;
    use thermite_core::square::Square::*;

    #[test_case("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", ChessMove { from: A5, to: B6, move_type: MoveType::Quiet { piece_type: PieceType::King } }, false)]
    fn into_legal_works(fen: &str, chess_move: ChessMove, is_legal: bool) {
        assert_eq!(PseudoLegalChessMove::new(chess_move).into_legal(&Board::from_fen(fen).expect("illegal FEN")), if is_legal { Some(chess_move) } else { None });
    }
}