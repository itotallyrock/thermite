use crate::board::Board;
use crate::board::state::State;
use crate::castles::CastleDirection;
use crate::chess_move::ChessMove;
use crate::move_type::MoveType;
use crate::piece_type::PieceType;
use crate::player::Player;
use crate::promotion_piece_type::PromotionPieceType;
use crate::sided_piece::SidedPiece;
use crate::square::Square;

impl Board {

    /// Undo a [chess move](ChessMove) on a board given the previous [`State`]
    pub fn unmake_move(&mut self, chess_move: ChessMove, previous_state: State) {
        // Switch to side that did the move
        self.switch_sides();

        let ChessMove { move_type, from, to } = chess_move;
        match move_type {
            MoveType::Quiet { piece_type } => self.move_piece(to, from, SidedPiece::new(piece_type, self.side_to_move)),
            MoveType::DoublePawnPush { .. } => self.move_piece(to, from, SidedPiece::new(PieceType::Pawn, self.side_to_move)),
            MoveType::Capture { piece_type, captured_piece } => self.undo_capture(from, to, SidedPiece::new(piece_type, self.side_to_move), captured_piece),
            MoveType::EnPassantCapture { captured_pawn_square } => self.undo_en_passant_capture(from, to, captured_pawn_square, self.side_to_move),
            MoveType::Castle { castle_direction } => self.undo_castle(from, to, castle_direction, self.side_to_move),
            MoveType::Promotion { promotion } => self.undo_promotion(from, to, promotion, self.side_to_move),
            MoveType::PromotingCapture { promotion, captured_piece } => self.undo_promoting_capture(from, to, promotion, captured_piece, self.side_to_move),
        }

        self.halfmove_count -= 1;
        // Reset state from previous move
        self.state = previous_state;
    }

    /// Undo a capture
    fn undo_capture(&mut self, from: Square, to: Square, piece: SidedPiece, captured_piece: PieceType) {
        self.move_piece(to, from, piece);
        self.add_piece(to, SidedPiece::new(captured_piece, piece.player.switch()));
    }

    /// Undo an en-passant-capture
    fn undo_en_passant_capture(&mut self, from: Square, to: Square, captured_pawn_square: Square, player: Player) {
        self.move_piece(to, from, SidedPiece::new(PieceType::Pawn, player));
        self.add_piece(captured_pawn_square, SidedPiece::new(PieceType::Pawn, player.switch()));
    }

    /// Undo a castle
    fn undo_castle(&mut self, king_from: Square, king_to: Square, castle_direction: CastleDirection, player: Player) {
        let rook_from_square = self.as_ref().castles().rook_from_square(player, castle_direction);
        let rook_to_square = self.as_ref().castles().rook_to_square(player, castle_direction);
        self.move_piece(rook_to_square, rook_from_square, SidedPiece::new(PieceType::Rook, player));
        self.move_piece(king_to, king_from, SidedPiece::new(PieceType::King, player));
    }

    /// Undo a pawn promotion
    fn undo_promotion(&mut self, from: Square, to: Square, promotion: PromotionPieceType, player: Player) {
        self.remove_piece(to, SidedPiece::new(promotion.into(), player));
        self.add_piece(from, SidedPiece::new(PieceType::Pawn, player));
    }

    /// Undo a pawn promotion-capture
    fn undo_promoting_capture(&mut self, from: Square, to: Square, promotion: PromotionPieceType, captured_piece: PieceType, player: Player) {
        self.remove_piece(to, SidedPiece::new(promotion.into(), player));
        self.add_piece(from, SidedPiece::new(PieceType::Pawn, player));
        self.add_piece(to, SidedPiece::new(captured_piece, player.switch()));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::Square::{E2, E4, E3, C3, B1, F3, G1, G3, G4, H4, H5, G7, H6, F2};
    use crate::piece_type::PieceType::{Pawn, Knight, King, Queen};
    use crate::chess_move::ChessMove;
    use test_case::test_case;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const POS_1_MATE_IN_THREE_W: &str = "r7/6Q1/2pp4/p6k/1qP1P3/6P1/P4PK1/8 w - - 0 1";
    const POS_1_MATE_IN_THREE_B: &str = "r7/6Q1/2pp4/p6k/1qP1P1P1/8/P4PK1/8 b - - 0 1";
    const POS_1_MATE_IN_TWO_W: &str = "r7/6Q1/2pp4/p7/1qP1P1Pk/8/P4PK1/8 w - - 1 2";
    const POS_1_MATE_IN_TWO_B: &str = "r7/8/2pp3Q/p7/1qP1P1Pk/8/P4PK1/8 b - - 2 2";
    const POS_1_MATE_IN_ONE_W: &str = "r7/8/2pp3Q/p7/1qP1P1k1/8/P4PK1/8 w - - 0 3";

    #[test_case(STARTPOS, ChessMove { to: E4, from: E2, move_type: MoveType::DoublePawnPush { en_passant_square: E3 } })]
    #[test_case(STARTPOS, ChessMove { to: E3, from: E2, move_type: MoveType::Quiet { piece_type: Pawn } })]
    #[test_case(STARTPOS, ChessMove { to: C3, from: B1, move_type: MoveType::Quiet { piece_type: Knight } })]
    #[test_case(STARTPOS, ChessMove { to: F3, from: G1, move_type: MoveType::Quiet { piece_type: Knight } })]
    #[test_case(POS_1_MATE_IN_THREE_W, ChessMove { to: G4, from: G3, move_type: MoveType::Quiet { piece_type: Pawn } })]
    #[test_case(POS_1_MATE_IN_THREE_B, ChessMove { to: H4, from: H5, move_type: MoveType::Quiet { piece_type: King } })]
    #[test_case(POS_1_MATE_IN_TWO_W, ChessMove { to: H6, from: G7, move_type: MoveType::Quiet { piece_type: Queen } })]
    #[test_case(POS_1_MATE_IN_TWO_B, ChessMove { to: G4, from: H4, move_type: MoveType::Capture { piece_type: King, captured_piece: Pawn } })]
    #[test_case(POS_1_MATE_IN_ONE_W, ChessMove { to: F3, from: F2, move_type: MoveType::Quiet { piece_type: Pawn } })]
    fn unmake_move_gives_previous_board(starting_fen: &str, chess_move: ChessMove) {
        let mut board = Board::from_fen(starting_fen).expect("illegal FEN");
        let expected = Board::from_fen(starting_fen).expect("illegal FEN");
        let state = board.make_move(chess_move);
        board.unmake_move(chess_move, state);
        assert_eq!(board, expected, "{board:#?} != {expected:#?}");
    }
}