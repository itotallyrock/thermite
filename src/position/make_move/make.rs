use crate::bitboard::BoardMask;
use crate::castles::{CastleRights, KING_FROM_SQUARES};
use crate::chess_move::capture::Capture;
use crate::chess_move::castle::Castle;
use crate::chess_move::castle::CastleQuietMoves;
use crate::chess_move::double_pawn_push::DoublePawnPush;
use crate::chess_move::en_passant_capture::EnPassantCapture;
use crate::chess_move::promoting_capture::PromotingCapture;
use crate::chess_move::promotion::Promotion;
use crate::chess_move::quiet::Quiet;
use crate::chess_move::ChessMove;
use crate::pieces::{
    NonKingPieceType, NonPawnPieceType, Piece, PieceType, PlacedPiece, SlidingPieceType,
};
use crate::player_color::PlayerColor;
use crate::position::{LegalPosition, LegalPositionState};

impl LegalPosition {
    /// Update the [mask](BoardMask) of [pieces](NonKingPieceType) giving check
    fn update_checkers(&mut self) {
        let attackers = self.player_to_move_mask();
        let defending_king_square = self.king_squares[self.player_to_move.switch()];
        self.state.checkers =
            self.attackers_to(defending_king_square, self.occupied_mask()) & attackers;
    }

    /// Update the blockers and pinned piece [masks](BoardMask) for a given [player](PlayerColor)
    fn update_blockers_pinners_for_side(&mut self, player: PlayerColor) {
        let attackers = self.opposite_player_mask();
        let king_square = self.king_squares[player];
        let mut blockers = BoardMask::EMPTY;
        let mut pinners = BoardMask::EMPTY;

        // Get all snipers aligned with the king
        let queens = self.piece_mask(NonKingPieceType::Queen);
        let cardinal_snipers = (self.piece_mask(NonKingPieceType::Rook) | queens)
            & BoardMask::pseudo_attacks_for(NonPawnPieceType::Rook, king_square);
        let ordinal_snipers = (self.piece_mask(NonKingPieceType::Bishop) | queens)
            & BoardMask::pseudo_attacks_for(NonPawnPieceType::Bishop, king_square);
        let all_snipers = cardinal_snipers | ordinal_snipers;
        let attacking_snipers = attackers & all_snipers;
        // All pieces, for both sides, that aren't attacking snipers
        let non_attacking_sniper_mask = self.occupied_mask() ^ attacking_snipers;

        for sniper_square in attacking_snipers {
            let blockers_mask =
                BoardMask::line_between(king_square, sniper_square) & non_attacking_sniper_mask;
            let has_single_blocker = blockers_mask.num_squares() == 1;

            if has_single_blocker {
                blockers |= blockers_mask;
                // If the blocker is also the same side as the attacking sniper, save the sniper as a pinner
                if (blockers_mask & attackers).is_empty() {
                    pinners |= sniper_square.to_mask();
                }
            }
        }

        self.state.pinners_for[player] = pinners;
        self.state.blockers_for[player] = blockers;
    }

    /// Update the check square [masks](BoardMask) for keeping track of which [squares](Square) a [piece](PieceType) can move to, to give check
    fn update_check_squares(&mut self) {
        let defending_player = self.player_to_move.switch();
        let defending_king_square = self.king_squares[defending_player];
        let defending_king_mask = self.king_squares[defending_player].to_mask();

        // Update non-sliders
        self.state.check_squares[NonKingPieceType::Pawn] =
            BoardMask::pawn_attacks(defending_king_mask, defending_player);
        self.state.check_squares[NonKingPieceType::Knight] =
            BoardMask::pseudo_attacks_for(NonPawnPieceType::Knight, defending_king_square);

        // Update sliders
        let occupied_mask = self.occupied_mask();
        let ordinal_attacks = BoardMask::sliding_attacks_for(
            SlidingPieceType::Bishop,
            defending_king_square,
            occupied_mask,
        );
        let cardinal_attacks = BoardMask::sliding_attacks_for(
            SlidingPieceType::Rook,
            defending_king_square,
            occupied_mask,
        );
        self.state.check_squares[NonKingPieceType::Bishop] = ordinal_attacks;
        self.state.check_squares[NonKingPieceType::Rook] = cardinal_attacks;
        self.state.check_squares[NonKingPieceType::Queen] = ordinal_attacks | cardinal_attacks;
    }

    /// Update the masks required for move generation
    /// - Blockers and pinners for both players
    /// - Squares that give check
    fn update_move_gen_masks(&mut self) {
        self.update_blockers_pinners_for_side(PlayerColor::White);
        self.update_blockers_pinners_for_side(PlayerColor::Black);
        self.update_check_squares();
    }

    /// Switch the player to move and update state [masks](crate::bitboard::BoardMask) (ie. `checkers`, `blockers`, `pinners`, `check_masks`)
    fn switch_perspectives(&mut self) {
        // Update the `checkers`
        self.update_checkers();
        // Update the `player_to_move`
        self.switch_player_to_move();
        // Update `blockers`, `pinners`, and `check_masks`
        self.update_move_gen_masks();
    }

    /// Force re-updating all state masks from the current board arrangement
    /// Used when importing a `PositionBuilder` after pieces have been added, before the legality check.
    pub(in crate::position) fn update_masks(&mut self) {
        // All of the logic we want is inside switch_perspectives, but we don't want the added
        // side-effect of changing the player to move so we do that first.
        self.switch_player_to_move();
        self.switch_perspectives();
    }

    /// Remove castle rights from the state if they're granted
    fn try_remove_castle_rights(&mut self, rights: CastleRights) {
        if self.state.castles.has_rights(rights) {
            self.state.castles.remove_rights(rights);
            // Remove any rights from the hash
            Castle::all()
                .filter(|castle| rights.has_rights(castle.required_rights()))
                .for_each(|castle| self.state.hash.toggle_castle_ability(castle));
        }
    }

    /// Remove a [`captured_piece`](NonKingPieceType) as part of a capture ([`make_capture`](Self::make_capture) and [`make_promoting_capture`](Self::make_promoting_capture))
    /// Also invalidates castle rights if the captured piece was a [rook](PieceType::Rook).
    fn capture_piece(&mut self, captured_piece: PlacedPiece<NonKingPieceType>) {
        self.remove_piece(captured_piece);

        // Check if piece that was captured was a castle rook to invalidate rights
        if captured_piece.owned_piece.piece == NonKingPieceType::Rook {
            let invalidated_castle = Castle::all_for_player(captured_piece.owned_piece.player)
                .find(|castle| captured_piece.square == castle.rook_from());
            if let Some(invalidated_castle) = invalidated_castle {
                let invalidated_rights = invalidated_castle.required_rights();
                self.try_remove_castle_rights(invalidated_rights);
            }
        }
    }

    /// Make a normal from-to move, that doesn't capture or promote or have any special state handling (ie. double-pawn-push)
    /// Also invalidates castle rights if the piece moving is a [king](PieceType::King) or [rook](PieceType::Rook)
    fn make_quiet(&mut self, quiet: Quiet) {
        // Move the piece
        self.move_piece(quiet);

        let owned_piece = quiet.piece();
        let player = owned_piece.player;
        match owned_piece.piece {
            PieceType::King => {
                if quiet.from() == KING_FROM_SQUARES[player] {
                    let invalidated_rights = CastleRights::for_player(player);
                    self.try_remove_castle_rights(invalidated_rights);
                }
            }
            PieceType::Rook => {
                let invalidated_castle = Castle::all_for_player(player)
                    .find(|castle| quiet.from() == castle.rook_from());
                if let Some(invalidated_castle) = invalidated_castle {
                    let invalidated_rights = invalidated_castle.required_rights();
                    self.try_remove_castle_rights(invalidated_rights);
                }
            }
            PieceType::Pawn => {
                self.reset_halfmove_clock();
            }
            _ => {}
        }
    }

    /// [`Capture`] a [piece](NonKingPieceType)
    ///
    /// # Panics
    /// Will panic during debug mode when the original square doesn't have the expected piece or the target square doesn't have the expected captured-piece for the opposite side
    fn make_capture(&mut self, capture: Capture) {
        let quiet: Quiet = capture.into();
        let captured_piece = capture
            .captured_piece()
            .owned_by(quiet.piece().player.switch())
            .placed_on(quiet.to());

        // Move the pieces
        self.capture_piece(captured_piece);
        self.make_quiet(quiet);

        // Reset the clock as this move is irreversible
        self.reset_halfmove_clock();
    }

    /// [Double pawn push](DoublePawnPush) (jump the first square) for [pawns](PieceType::Pawn) on their side's starting [rank](crate::square::Rank) with both forward squares unobstructed.
    ///
    /// # Panics
    /// Will panic during debug mode when the to and en-passant squares are occupied, the from square doesn't have the pawn.
    fn make_double_pawn_push(&mut self, push: DoublePawnPush) {
        debug_assert_eq!(
            self.owned_piece_on(push.en_passant_square().into()),
            None,
            "attempting to `make_double_pawn_push` over occupied `Square`"
        );

        // Move the pawn
        let quiet = From::from(push);
        self.move_piece(quiet);
        // Update en-passant square in state
        self.set_en_passant(push.en_passant_square());
        // Reset the clock as this move is irreversible
        self.reset_halfmove_clock();
    }

    /// [`Pawn`](PieceType::Pawn) [en-passant capture](EnPassantCapture) opposing [player](player_color::PlayerColor)'s previously [double pushed pawn](DoublePawnPush).
    fn make_en_passant_capture(&mut self, en_passant_capture: EnPassantCapture) {
        // Move the pawn
        let quiet = From::from(en_passant_capture);
        self.move_piece(quiet);
        // Remove the captured pawn
        let captured_pawn = NonKingPieceType::Pawn
            .owned_by(en_passant_capture.player().switch())
            .placed_on(en_passant_capture.captured_square().into());
        self.remove_piece(captured_pawn);
        // Reset the clock as this move is irreversible
        self.reset_halfmove_clock();
    }

    /// [`Castle`] (king and rook back-rank file-swapping)
    ///
    /// # Panics
    /// Will panic during debug mode when castling without [rights](CastleRights)
    /// Will panic during debug mode when the path is attacked
    fn make_castle(&mut self, castle: Castle) {
        debug_assert!(
            self.state.castles.can_castle(castle),
            "attempting to `make_castle` without adequate `CastleRights`"
        );
        debug_assert!(
            castle
                .unattacked_mask()
                .into_iter()
                .all(|sq| (self.attackers_to(sq, self.occupied_mask())
                    & self.opposite_player_mask())
                .is_empty()),
            "attempting to `make_castle` through attacked squares"
        );

        // Move the pieces
        let CastleQuietMoves {
            rook_quiet,
            king_quiet,
        } = CastleQuietMoves::new(castle);
        self.move_piece(king_quiet);
        self.move_piece(rook_quiet);

        // Remove both queen/king castle rights once a player has castled
        let both_castle_rights = CastleRights::for_player(castle.player());
        self.try_remove_castle_rights(both_castle_rights);
    }

    /// Perform a legal [`Promotion`]
    fn make_promotion(&mut self, promotion: Promotion) {
        // Remove the old pawn
        let original_pawn = NonKingPieceType::Pawn
            .owned_by(promotion.player())
            .placed_on(promotion.from().into());
        self.remove_piece(original_pawn);
        // Add the promoted piece
        self.add_piece(
            PieceType::from(promotion.piece)
                .owned_by(promotion.player())
                .placed_on(promotion.to().into()),
        );
        // Reset the clock as this move is irreversible
        self.reset_halfmove_clock();
    }

    /// Perform a legal [`PromotingCapture`]
    fn make_promoting_capture(&mut self, promoting_capture: PromotingCapture) {
        let promotion = promoting_capture.promotion();
        let captured_piece = promoting_capture
            .captured_piece()
            .owned_by(promotion.player().switch())
            .placed_on(promotion.to().into());
        // Remove the captured piece
        self.capture_piece(captured_piece);
        // Do the promotion
        self.make_promotion(promotion);
    }

    /// Perform a legal [move](ChessMove) on the [board](LegalPosition) returning a copy of the [`LegalPositionState`] from before the move was made in order to [undo the move](LegalPosition::unmake_move).
    pub fn make_move(&mut self, chess_move: ChessMove) -> LegalPositionState {
        let current_state = self.state;

        // Clear any single ply state
        self.clear_en_passant();

        // Increment our move counter
        self.increment_halfmove_clock();

        match chess_move {
            ChessMove::Quiet(quiet) => self.make_quiet(quiet),
            ChessMove::DoublePawnPush(pawn_push) => self.make_double_pawn_push(pawn_push),
            ChessMove::Capture(capture) => self.make_capture(capture),
            ChessMove::EnPassantCapture(capture) => self.make_en_passant_capture(capture),
            ChessMove::Castle(castle) => self.make_castle(castle),
            ChessMove::Promotion(promotion) => self.make_promotion(promotion),
            ChessMove::PromotingCapture(promoting_capture) => {
                self.make_promoting_capture(promoting_capture);
            }
        }

        self.switch_perspectives();

        current_state
    }
}
