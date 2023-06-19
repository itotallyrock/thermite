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
use crate::chess_move::quiet::QuietMove;
use crate::chess_move::ChessMove;
use crate::pieces::{NonKingPieceType, Piece, PieceType};
use crate::position::{LegalPosition, LegalPositionState};

impl LegalPosition {
    /// Undo a [`QuietMove`]
    /// - Move the piece back to its starting [`Square`](square::Square)
    fn unmake_quiet(&mut self, quiet: QuietMove) {
        self.move_piece(quiet.reverse());
    }

    /// Undo an [`DoublePawnPush`]
    /// - Move the [`Pawn`](crate::pieces::PieceType::Pawn) back to its starting [`Square`](square::Square)
    /// - Add the captured [`Pawn`](crate::pieces::PieceType::Pawn) back
    fn unmake_double_pawn_push(&mut self, pawn_push: DoublePawnPush) {
        let quiet: QuietMove = pawn_push.into();
        self.unmake_quiet(quiet);
    }

    /// Undo a capture move
    /// - Move the piece doing the capturing back to its starting [`Square`](square::Square)
    fn unmake_capture(&mut self, capture: Capture) {
        let quiet: QuietMove = capture.into();
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
        let quiet: QuietMove = capture.into();
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

    /// Undo a [chess move](ChessMove) on a board given the previous [`LegalPositionState`]
    /// from calling [`make_move`](LegalPosition::make_move)
    pub fn unmake_move(&mut self, chess_move: ChessMove, previous_state: LegalPositionState) {
        // Switch to side that did the move
        self.switch_player_to_move();

        match chess_move {
            ChessMove::Quiet(quiet) => self.unmake_quiet(quiet),
            ChessMove::DoublePawnPush(pawn_push) => self.unmake_double_pawn_push(pawn_push),
            ChessMove::Capture(capture) => self.unmake_capture(capture),
            ChessMove::EnPassantCapture(capture) => self.unmake_en_passant_capture(capture),
            ChessMove::Castle(castle) => self.unmake_castle(castle),
            ChessMove::Promotion(promotion) => self.unmake_promotion(promotion),
            ChessMove::PromotingCapture(promoting_capture) => {
                self.unmake_promoting_capture(promoting_capture);
            }
        }

        self.decrement_halfmove_clock();
        // Reset state from previous move
        self.state = previous_state;
    }
}
