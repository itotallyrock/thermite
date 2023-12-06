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
use crate::square::EnPassantSquare;

impl LegalPosition {
    /// Clear the [`HalfMoveClock`](crate::half_move_clock::HalfMoveClock) due to an irreversible [`chess_move::ChessMove`] being played
    fn reset_halfmove_clock(&mut self) {
        self.state.halfmove_clock.reset();
    }

    /// Increment the [`HalfMoveClock`](crate::half_move_clock::HalfMoveClock) indicating one player has finished their turn
    fn increment_halfmove_clock(&mut self) {
        let _ = self.state.halfmove_clock.increment();
    }

    /// Set the [`EnPassantSquare`] for move generation and maintain its associated hash
    fn set_en_passant(&mut self, en_passant_square: EnPassantSquare) {
        debug_assert_eq!(
            self.state.en_passant_square, None,
            "attempting to `set_en_passant` when it's already set"
        );
        // Update the state and hash with the new square
        self.state.en_passant_square = Some(en_passant_square);
        self.state.hash.toggle_en_passant_square(en_passant_square);
    }

    /// Try to clear the [`EnPassantSquare`], if set, for future move generation and remove its key from the hash
    fn try_clear_en_passant(&mut self) {
        if let Some(en_passant_square) = self.state.en_passant_square {
            self.state.hash.toggle_en_passant_square(en_passant_square);
            self.state.en_passant_square = None;
        }
    }

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
        let previous_state = self.state;

        // Clear any single ply state
        self.try_clear_en_passant();

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

        previous_state
    }
}

#[cfg(test)]
mod test {
    use crate::castles::{
        CastleDirection::{KingSide, QueenSide},
        CastleRights,
    };
    use crate::chess_move::capture::Capture;
    use crate::chess_move::castle::Castle;
    use crate::chess_move::double_pawn_push::DoublePawnPush;
    use crate::chess_move::en_passant_capture::EnPassantCapture;
    use crate::chess_move::promoting_capture::PromotingCapture;
    use crate::chess_move::promotion::Promotion;
    use crate::chess_move::quiet::Quiet;
    use crate::chess_move::ChessMove;
    use crate::direction::PawnCaptureDirection;
    use crate::fen;
    use crate::half_move_clock::HalfMoveClock;
    use crate::pieces::{NonKingPieceType, Piece, PieceType, PieceType::*, PromotablePieceType};
    use crate::player_color::PlayerColor::{Black, White};
    use crate::square::{
        DoublePawnToSquare, EastShiftableFile, File, Square, Square::*, WestShiftableFile,
    };
    use test_case::test_case;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn make_move_switches_sides() {
        let mut pos = fen!(STARTPOS);
        let piece = Pawn.owned_by(White);
        let chess_move = ChessMove::Quiet(Quiet::new(F2, F3, piece).unwrap());
        assert_eq!(pos.player_to_move(), White);
        let _ = pos.make_move(chess_move);
        assert_eq!(pos.player_to_move(), Black);
    }

    #[test]
    fn make_quiet_works() {
        let mut pos = fen!(STARTPOS);
        let piece = Pawn.owned_by(White);
        let chess_move = ChessMove::Quiet(Quiet::new(F2, F3, piece).unwrap());
        assert_eq!(pos.owned_piece_on(F2), Some(piece));
        assert_eq!(pos.piece_type_on(F3), None);
        let _ = pos.make_move(chess_move);
        assert_eq!(pos.piece_type_on(F2), None);
        assert_eq!(pos.owned_piece_on(F3), Some(piece));
    }

    #[test_case(
        "4r3/1pP2kp1/3R4/1p3pP1/1r6/1P6/1K2p3/4R3 w - - 0 1",
        Promotion::new(PromotablePieceType::Queen, File::C, White)
    )]
    #[test_case(
        "4r3/1pP2kp1/3R4/1p3pP1/1r6/1P6/1K2p3/8 b - - 0 1",
        Promotion::new(PromotablePieceType::Queen, File::E, Black)
    )]
    fn make_promotion_works(fen: &str, promotion: Promotion) {
        let mut position = fen!(fen);
        let chess_move = ChessMove::Promotion(promotion);
        position.make_move(chess_move);
        assert_eq!(
            position.owned_piece_on(Square::from(promotion.to())),
            Some(PieceType::from(promotion.piece).owned_by(promotion.player()))
        );
        assert_eq!(
            position.owned_piece_on(Square::from(promotion.from())),
            None
        );
    }

    #[test_case(
        "1k6/3R4/PPP5/4p3/4P3/6p1/1p4P1/2RK4 b - - 0 1",
        PromotingCapture::new(
            Promotion::new_east_capture(PromotablePieceType::Queen, EastShiftableFile::B, Black),
            NonKingPieceType::Rook
        )
    )]
    #[test_case(
        "1k1r3r/p3P1p1/4p1p1/5p2/2Qq3P/5BP1/PP3PK1/2R5 w - - 0 1",
        PromotingCapture::new(
            Promotion::new_west_capture(PromotablePieceType::Queen, WestShiftableFile::E, White),
            NonKingPieceType::Rook
        )
    )]
    fn make_promoting_capture_works(fen: &str, promotion: PromotingCapture) {
        let mut position = fen!(fen);
        let chess_move = ChessMove::PromotingCapture(promotion);
        position.make_move(chess_move);
        assert_eq!(
            position.owned_piece_on(Square::from(promotion.promotion().to())),
            Some(
                PieceType::from(promotion.promotion().piece)
                    .owned_by(promotion.promotion().player())
            )
        );
        assert_eq!(
            position.owned_piece_on(Square::from(promotion.promotion().from())),
            None
        );
    }

    #[test]
    fn try_remove_castle_rights_works() {
        // Removing all
        let mut position = fen!("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1");
        position.try_remove_castle_rights(CastleRights::All);
        assert_eq!(position.state.castles, CastleRights::None);

        // Removing one at at time
        let mut position = fen!("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1");
        let previous_hash = position.state.hash;
        position.try_remove_castle_rights(CastleRights::WhiteKing);
        assert_eq!(position.state.castles, CastleRights::WhiteQueenBlackBoth);
        assert_ne!(position.state.hash, previous_hash);
        let previous_hash = position.state.hash;
        position.try_remove_castle_rights(CastleRights::WhiteQueen);
        assert_eq!(position.state.castles, CastleRights::BlackBoth);
        assert_ne!(position.state.hash, previous_hash);
        let previous_hash = position.state.hash;
        position.try_remove_castle_rights(CastleRights::BlackKing);
        assert_eq!(position.state.castles, CastleRights::BlackQueen);
        assert_ne!(position.state.hash, previous_hash);
        let previous_hash = position.state.hash;
        position.try_remove_castle_rights(CastleRights::BlackQueen);
        assert_eq!(position.state.castles, CastleRights::None);
        assert_ne!(position.state.hash, previous_hash);
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Castle::new(White, KingSide),
        CastleRights::WhiteBoth
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Castle::new(White, QueenSide),
        CastleRights::WhiteBoth
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Castle::new(Black, KingSide),
        CastleRights::BlackBoth
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Castle::new(Black, QueenSide),
        CastleRights::BlackBoth
    )]
    fn castle_removes_rights(fen: &str, castle: Castle, removed_rights: CastleRights) {
        let chess_move = ChessMove::Castle(castle);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Capture::new(Quiet::new(A1, A8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook),
        CastleRights::BlackQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Capture::new(Quiet::new(H1, H8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook),
        CastleRights::BlackKing
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Capture::new(Quiet::new(A8, A1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook),
        CastleRights::WhiteQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Capture::new(Quiet::new(H8, H1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook),
        CastleRights::WhiteKing
    )]
    fn capturing_castle_removes_rights(fen: &str, capture: Capture, removed_rights: CastleRights) {
        let chess_move = ChessMove::Capture(capture);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Quiet::new(A1, A7, Rook.owned_by(White)).unwrap(),
        CastleRights::WhiteQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Quiet::new(H1, H7, Rook.owned_by(White)).unwrap(),
        CastleRights::WhiteKing
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Quiet::new(A8, A2, Rook.owned_by(Black)).unwrap(),
        CastleRights::BlackQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Quiet::new(H8, H2, Rook.owned_by(Black)).unwrap(),
        CastleRights::BlackKing
    )]
    fn quiet_before_castle_removes_rights(fen: &str, quiet: Quiet, removed_rights: CastleRights) {
        let chess_move = ChessMove::Quiet(quiet);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Capture::new(Quiet::new(A1, A8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook),
        CastleRights::WhiteQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        Capture::new(Quiet::new(H1, H8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook),
        CastleRights::WhiteKing
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Capture::new(Quiet::new(A8, A1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook),
        CastleRights::BlackQueen
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        Capture::new(Quiet::new(H8, H1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook),
        CastleRights::BlackKing
    )]
    fn capturing_as_castle_removes_rights(
        fen: &str,
        capture: Capture,
        removed_rights: CastleRights,
    ) {
        let chess_move = ChessMove::Capture(capture);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1", Quiet::new(E1, F1, King.owned_by(White)).unwrap(), CastleRights::WhiteBoth)]
    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1", Quiet::new(E8, F8, King.owned_by(Black)).unwrap(), CastleRights::BlackBoth)]
    fn moving_king_before_castle_removes_rights(
        fen: &str,
        quiet: Quiet,
        removed_rights: CastleRights,
    ) {
        let chess_move = ChessMove::Quiet(quiet);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case("r3k2r/1pppPpp1/8/8/8/8/1PPPpPP1/R3K2R w KQkq - 0 1", Capture::new(Quiet::new(E1, E2, King.owned_by(White)).unwrap(), NonKingPieceType::Pawn), CastleRights::WhiteBoth)]
    #[test_case("r3k2r/1pppPpp1/8/8/8/8/1PPPpPP1/R3K2R b KQkq - 0 1", Capture::new(Quiet::new(E8, E7, King.owned_by(Black)).unwrap(), NonKingPieceType::Pawn), CastleRights::BlackBoth)]
    fn capturing_as_king_before_castle_removes_rights(
        fen: &str,
        capture: Capture,
        removed_rights: CastleRights,
    ) {
        let chess_move = ChessMove::Capture(capture);
        let mut position = fen!(fen);
        assert_eq!(position.state.castles & removed_rights, removed_rights);
        position.make_move(chess_move);
        assert_eq!(position.state.castles & removed_rights, CastleRights::None);
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 0 1",
        DoublePawnPush::new(White, File::E)
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 0 1",
        DoublePawnPush::new(Black, File::E)
    )]
    fn double_pawn_push_sets_en_passant_square(fen: &str, push: DoublePawnPush) {
        let chess_move = ChessMove::DoublePawnPush(push);
        let mut position = fen!(fen);
        assert_eq!(position.state.en_passant_square, None);
        position.make_move(chess_move);
        assert_eq!(
            position.state.en_passant_square,
            Some(push.en_passant_square())
        );
    }

    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 12 24",
        ChessMove::DoublePawnPush(DoublePawnPush::new(White, File::E))
    )]
    #[test_case(
        "r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 12 23",
        ChessMove::DoublePawnPush(DoublePawnPush::new(Black, File::E))
    )]
    #[test_case("r3k2r/1pppPpp1/8/8/8/8/1PPPpPP1/R3K2R w KQkq - 12 24", ChessMove::Capture(Capture::new(Quiet::new(E1, E2, King.owned_by(White)).unwrap(), NonKingPieceType::Pawn)))]
    #[test_case("r3k2r/1pppPpp1/8/8/8/8/1PPPpPP1/R3K2R b KQkq - 12 23", ChessMove::Capture(Capture::new(Quiet::new(E8, E7, King.owned_by(Black)).unwrap(), NonKingPieceType::Pawn)))]
    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 12 24", ChessMove::Capture(Capture::new(Quiet::new(A1, A8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook)))]
    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R w KQkq - 12 24", ChessMove::Capture(Capture::new(Quiet::new(H1, H8, Rook.owned_by(White)).unwrap(), NonKingPieceType::Rook)))]
    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 12 23", ChessMove::Capture(Capture::new(Quiet::new(A8, A1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook)))]
    #[test_case("r3k2r/1pppppp1/8/8/8/8/1PPPPPP1/R3K2R b KQkq - 12 23", ChessMove::Capture(Capture::new(Quiet::new(H8, H1, Rook.owned_by(Black)).unwrap(), NonKingPieceType::Rook)))]
    fn moves_reset_halfmove_clock(fen: &str, chess_move: ChessMove) {
        let mut position = fen!(fen);
        let _ = position.make_move(chess_move);
        assert_eq!(position.state.halfmove_clock, HalfMoveClock::default());
    }

    #[test_case("4R3/P5kp/2q2pp1/3BpP2/3nP3/Q3B1K1/1r5P/8 w - e6 0 1", EnPassantCapture::new_from(DoublePawnToSquare::F5, PawnCaptureDirection::West, White).unwrap())]
    fn en_passant_capture_work(fen: &str, capture: EnPassantCapture) {
        let mut position = fen!(fen);
        let chess_move = ChessMove::EnPassantCapture(capture);
        let _ = position.make_move(chess_move);
        assert_eq!(
            position.owned_piece_on(Square::from(capture.to())),
            Some(Pawn.owned_by(White))
        );
        assert_eq!(
            position.owned_piece_on(Square::from(capture.captured_square())),
            None
        );
        assert_eq!(position.owned_piece_on(Square::from(capture.from())), None);
        assert_eq!(position.state.en_passant_square, None);
    }
}
