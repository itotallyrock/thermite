//! Contains specialized undo methods (called via [`unmake_move`](LegalPosition::unmake_move))
//! We can do less work unmaking than making a move because the original state is passed to
//! [`unmake_move`](LegalPosition::unmake_move) allowing us to skip any steps undoing the state.

use crate::chess_move::capture::Capture;
use crate::chess_move::castle::Castle;
use crate::chess_move::castle::CastleQuietMoves;
use crate::chess_move::double_pawn_push::DoublePawnPush;
use crate::chess_move::en_passant_capture::EnPassantCapture;
use crate::chess_move::promoting_capture::PromotingCapture;
use crate::chess_move::promotion::Promotion;
use crate::chess_move::quiet::Quiet;
use crate::chess_move::ChessMove;
use crate::pieces::{NonKingPieceType, Piece, PieceType};
use crate::position::legal_position::State;
use crate::position::{LegalPosition, LegalPositionState};
use crate::square::EnPassantSquare;

impl LegalPosition {
    /// Undo a [`Quiet`]
    /// - Move the piece back to its starting [`Square`](square::Square)
    fn unmake_quiet(&mut self, quiet: Quiet) {
        self.move_piece(quiet.reverse());
    }

    /// Undo an [`DoublePawnPush`]
    /// - Move the [`Pawn`](crate::pieces::PieceType::Pawn) back to its starting [`Square`](square::Square)
    /// - Add the captured [`Pawn`](crate::pieces::PieceType::Pawn) back
    fn unmake_double_pawn_push(
        &mut self,
        pawn_push: DoublePawnPush,
        previous_en_passant: Option<EnPassantSquare>,
    ) {
        let quiet: Quiet = pawn_push.into();
        self.unmake_quiet(quiet);
        if let Some(previous_en_passant) = previous_en_passant {
            self.set_en_passant(previous_en_passant);
        } else {
            self.clear_en_passant();
        }
    }

    /// Undo a capture move
    /// - Move the piece doing the capturing back to its starting [`Square`](square::Square)
    fn unmake_capture(&mut self, capture: Capture) {
        let quiet: Quiet = capture.into();
        let opposite_player = quiet.piece().player.switch();
        let captured_piece = PieceType::from(capture.captured_piece())
            .owned_by(opposite_player)
            .placed_on(quiet.to());
        self.unmake_quiet(quiet);
        self.add_piece(captured_piece);
    }

    /// Undo an [`EnPassantCapture`]
    /// - Move the [`Pawn`](crate::pieces::PieceType::Pawn) back to its starting [`Square`](square::Square)
    /// - Add the captured [`Pawn`](crate::pieces::PieceType::Pawn) back
    fn unmake_en_passant_capture(&mut self, capture: EnPassantCapture) {
        let quiet: Quiet = capture.into();
        let opposite_player = quiet.piece().player.switch();
        let captured_pawn = PieceType::Pawn
            .owned_by(opposite_player)
            .placed_on(capture.captured_square().into());
        self.unmake_quiet(quiet);
        self.add_piece(captured_pawn);
    }

    /// Undo a [`Castle`]
    /// - Move the [King](PieceType::King) back to its original [`Square`](square::Square)
    /// - Move the [Rook](PieceType::Rook) back to its original [`Square`](square::Square)
    fn unmake_castle(&mut self, castle: Castle) {
        let CastleQuietMoves {
            rook_quiet,
            king_quiet,
        } = CastleQuietMoves::new(castle);
        self.unmake_quiet(king_quiet);
        self.unmake_quiet(rook_quiet);
    }

    /// Undo a [`Promotion`]
    /// - Remove the [promoted piece](PromotablePieceType)
    /// - Add the [pawn](PieceType::Pawn) back
    fn unmake_promotion(&mut self, promotion: Promotion) {
        let player = promotion.player();
        let promoted_piece = NonKingPieceType::try_from(PieceType::from(promotion.piece))
            .expect("all promotion pieces are non-king")
            .owned_by(player)
            .placed_on(promotion.to().into());
        let pawn = PieceType::Pawn
            .owned_by(player)
            .placed_on(promotion.from().into());
        self.remove_piece(promoted_piece);
        self.add_piece(pawn);
    }

    /// Undo a promotion capture
    /// - Undo the [`Promotion`]
    /// - Add the [captured piece](NonKingPieceType) back
    fn unmake_promoting_capture(&mut self, promoting_capture: PromotingCapture) {
        let promotion = promoting_capture.promotion();
        let opposite_player = promotion.player().switch();
        let captured_piece = PieceType::from(promoting_capture.captured_piece())
            .owned_by(opposite_player)
            .placed_on(promotion.to().into());
        self.unmake_promotion(promotion);
        self.add_piece(captured_piece);
    }

    /// Reset state from a previous move
    fn restore_state(&mut self, previous_state: State) {
        self.state = previous_state;
    }

    /// Undo a [chess move](ChessMove) on a board given the previous [`LegalPositionState`]
    /// from calling [`make_move`](LegalPosition::make_move)
    pub fn unmake_move(&mut self, chess_move: ChessMove, previous_state: LegalPositionState) {
        // Switch to side that did the move
        self.switch_player_to_move();

        match chess_move {
            ChessMove::Quiet(quiet) => self.unmake_quiet(quiet),
            ChessMove::DoublePawnPush(pawn_push) => {
                self.unmake_double_pawn_push(pawn_push, previous_state.en_passant_square);
            }
            ChessMove::Capture(capture) => self.unmake_capture(capture),
            ChessMove::EnPassantCapture(capture) => self.unmake_en_passant_capture(capture),
            ChessMove::Castle(castle) => self.unmake_castle(castle),
            ChessMove::Promotion(promotion) => self.unmake_promotion(promotion),
            ChessMove::PromotingCapture(promoting_capture) => {
                self.unmake_promoting_capture(promoting_capture);
            }
        }

        self.decrement_halfmove_clock();
        self.restore_state(previous_state);
    }
}

#[cfg(test)]
mod test {
    use crate::castles::CastleDirection::{KingSide, QueenSide};
    use crate::chess_move::{
        capture::Capture, castle::Castle, double_pawn_push::DoublePawnPush,
        en_passant_capture::EnPassantCapture, promoting_capture::PromotingCapture,
        promotion::Promotion, quiet::Quiet, ChessMove,
    };
    use crate::direction::PawnCaptureDirection::{East, West};
    use crate::fen;
    use crate::pieces::{
        NonKingPieceType, Piece,
        PieceType::{King, Knight, Pawn, Queen},
        PromotablePieceType,
    };
    use crate::player_color::PlayerColor::{Black, White};
    use crate::square::{
        DoublePawnToSquare, EastShiftableFile, File,
        Square::{B1, C3, E2, E3, F2, F3, F5, G1, G3, G4, G7, H4, H5, H6},
        WestShiftableFile,
    };
    use test_case::test_case;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const POS_1_MATE_IN_THREE_W: &str = "r7/6Q1/2pp4/p6k/1qP1P3/6P1/P4PK1/8 w - - 0 1";
    const POS_1_MATE_IN_THREE_B: &str = "r7/6Q1/2pp4/p6k/1qP1P1P1/8/P4PK1/8 b - - 0 1";
    const POS_1_MATE_IN_TWO_W: &str = "r7/6Q1/2pp4/p7/1qP1P1Pk/8/P4PK1/8 w - - 1 2";
    const POS_1_MATE_IN_TWO_B: &str = "r7/8/2pp3Q/p7/1qP1P1Pk/8/P4PK1/8 b - - 2 2";
    const POS_1_MATE_IN_ONE_W: &str = "r7/8/2pp3Q/p7/1qP1P1k1/8/P4PK1/8 w - - 0 3";
    const POS_2_CASTLES_W: &str =
        "r3k2r/2nb1ppp/2ppqn2/1pP3b1/p2PP3/2N1NPP1/PPBBQ2P/R3K2R w KQkq - 0 1";
    const POS_2_CASTLES_B: &str =
        "r3k2r/2nb1ppp/2ppqn2/1pP3b1/p2PP3/2N1NPP1/PPBBQ2P/R3K2R b KQkq - 0 1";
    const POS_3_PINNED_W: &str = "8/2q3kp/6p1/3BpP2/8/Q3B1K1/1r5P/8 w - e6 0 1";
    const POS_4_PROMO_W: &str = "4R3/P5kp/2q2pp1/3BpP2/3nP3/Q3B1K1/1r5P/8 w - e6 0 1";

    #[test_case(
        STARTPOS,
        ChessMove::DoublePawnPush(DoublePawnPush::new(White, File::E))
    )]
    #[test_case(
        POS_3_PINNED_W,
        ChessMove::DoublePawnPush(DoublePawnPush::new(White, File::H))
    )]
    #[test_case(STARTPOS, ChessMove::Quiet(Quiet::new(E2, E3, Pawn.owned_by(White)).unwrap()))]
    #[test_case(STARTPOS, ChessMove::Quiet(Quiet::new(B1, C3, Knight.owned_by(White)).unwrap()))]
    #[test_case(STARTPOS, ChessMove::Quiet(Quiet::new(G1, F3, Knight.owned_by(White)).unwrap()))]
    #[test_case(POS_1_MATE_IN_THREE_W, ChessMove::Quiet(Quiet::new(G3, G4, Pawn.owned_by(White)).unwrap()))]
    #[test_case(POS_1_MATE_IN_THREE_B, ChessMove::Quiet(Quiet::new(H5, H4, King.owned_by(Black)).unwrap()))]
    #[test_case(POS_1_MATE_IN_TWO_W, ChessMove::Quiet(Quiet::new(G7, H6, Queen.owned_by(White)).unwrap()))]
    #[test_case(POS_1_MATE_IN_TWO_B, ChessMove::Capture(Capture::new(Quiet::new(H4, G4, King.owned_by(Black)).unwrap(), NonKingPieceType::Pawn)))]
    #[test_case(POS_1_MATE_IN_ONE_W, ChessMove::Quiet(Quiet::new(F2, F3, Pawn.owned_by(White)).unwrap()))]
    #[test_case(POS_2_CASTLES_W, ChessMove::Castle(Castle::new(White, KingSide)))]
    #[test_case(POS_2_CASTLES_W, ChessMove::Castle(Castle::new(White, QueenSide)))]
    #[test_case(POS_2_CASTLES_B, ChessMove::Castle(Castle::new(Black, KingSide)))]
    #[test_case(POS_2_CASTLES_B, ChessMove::Castle(Castle::new(Black, QueenSide)))]
    #[test_case(
        POS_3_PINNED_W,
        ChessMove::EnPassantCapture(EnPassantCapture::new(DoublePawnToSquare::F5, West, White).unwrap())
    )]
    #[test_case(
        POS_4_PROMO_W,
        ChessMove::EnPassantCapture(EnPassantCapture::new(DoublePawnToSquare::F5, West, White).unwrap())
    )]
    #[test_case(
        POS_4_PROMO_W,
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Queen, File::A, White))
    )]
    #[test_case(
        POS_4_PROMO_W,
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Knight, File::A, White))
    )]
    #[test_case(
        POS_4_PROMO_W,
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Bishop, File::A, White))
    )]
    #[test_case(
        POS_4_PROMO_W,
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Rook, File::A, White))
    )]
    #[test_case(
        "1n2R3/P5kp/2q2pp1/3BpP2/3nP3/Q3B1K1/1r5P/8 w - e6 0 1",
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_east_capture(PromotablePieceType::Rook, EastShiftableFile::A, White),
            NonKingPieceType::Knight
        ))
    )]
    #[test_case(
        "1n2R3/2P3kp/2q2pp1/3BpP2/3nP3/Q3B1K1/1r5P/8 w - e6 0 1",
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_west_capture(PromotablePieceType::Queen, WestShiftableFile::C, White),
            NonKingPieceType::Knight
        ))
    )]
    // TODO: Test capturing castle rook
    // TODO: Test moving king with castle rights
    fn unmake_move_gives_previous_board(starting_fen: &str, chess_move: ChessMove) {
        let expected = fen!(starting_fen);
        let mut board = expected.clone();
        let state = board.make_move(chess_move);
        board.unmake_move(chess_move, state);
        assert_eq!(board, expected, "{board:#?} != {expected:#?}");
    }
}
