#[cfg(any(feature = "piece_square_eval", feature = "material_eval"))]
use std::ops::{AddAssign, SubAssign};


use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::board::state::State;
use crate::castles::{CastleDirection, CastleRights};
use crate::chess_move::ChessMove;
use crate::move_type::MoveType;
#[cfg(feature = "piece_square_eval")]
use crate::piece_square_eval::piece_square_bonus_lookup;
use crate::piece_type::PieceType;
use crate::player::Player;
use crate::promotion_piece_type::PromotionPieceType;
use crate::sided_piece::SidedPiece;
use crate::square::Square;

impl Board {
    /// Place a piece on the board
    ///
    /// # Panics
    /// Panics in debug mode when attempting to add a piece to a non-empty square
    pub(super) fn add_piece(&mut self, to: Square, piece: SidedPiece) {
        debug_assert!(self.piece_on(to).is_none());
        debug_assert!(self.side_on(to).is_none());
        let to_mask = to.to_mask();
        *self.piece_squares.mut_square(to) = Some(piece.piece_type);
        *self.piece_masks.mut_piece(piece.piece_type) |= to_mask;
        *self.side_masks.mut_side(piece.player) |= to_mask;
        #[cfg(feature = "zobrist")]
        self.hasher.toggle_piece_square(to, piece);
        #[cfg(feature = "material_eval")]
        self.sided_piece_counts.add_piece(piece);
        #[cfg(feature = "piece_square_eval")]
        self.piece_square_eval.add_assign(piece_square_bonus_lookup(piece, to));
    }

    /// Clear a piece from the board
    ///
    /// # Panics
    /// Will panic in debug mode when removing a piece from an empty square
    pub(super) fn remove_piece(&mut self, from: Square, piece: SidedPiece) {
        debug_assert_eq!(self.piece_on(from), Some(piece.piece_type));
        debug_assert_eq!(self.side_on(from), Some(piece.player));
        let from_mask = from.to_mask();
        *self.piece_squares.mut_square(from) = None;
        *self.piece_masks.mut_piece(piece.piece_type) ^= from_mask;
        *self.side_masks.mut_side(piece.player) ^= from_mask;
        #[cfg(feature = "zobrist")]
        self.hasher.toggle_piece_square(from, piece);
        #[cfg(feature = "material_eval")]
        self.sided_piece_counts.remove_piece(piece);
        #[cfg(feature = "piece_square_eval")]
        self.piece_square_eval.sub_assign(piece_square_bonus_lookup(piece, from));
    }

    /// Move a piece on the board
    ///
    /// # Panics
    /// Will panic in debug mode if moving from a square not owned by the side or to an occupied square.
    pub(super) fn move_piece(&mut self, from: Square, to: Square, piece: SidedPiece) {
        debug_assert_eq!(self.piece_on(from), Some(piece.piece_type));
        debug_assert_eq!(self.side_on(from), Some(piece.player));
        debug_assert!(self.piece_on(to).is_none());
        debug_assert!(self.side_on(to).is_none());

        let to_mask = to.to_mask();
        let from_mask = from.to_mask();
        let from_to_mask = from_mask | to_mask;
        *self.piece_squares.mut_square(from) = None;
        *self.piece_squares.mut_square(to) = Some(piece.piece_type);
        *self.piece_masks.mut_piece(piece.piece_type) ^= from_to_mask;
        *self.side_masks.mut_side(piece.player) ^= from_to_mask;
        #[cfg(feature = "zobrist")]
        self.hasher.toggle_piece_square(from, piece);
        #[cfg(feature = "zobrist")]
        self.hasher.toggle_piece_square(to, piece);
        #[cfg(feature = "piece_square_eval")]
        self.piece_square_eval.sub_assign(piece_square_bonus_lookup(piece, from));
        #[cfg(feature = "piece_square_eval")]
        self.piece_square_eval.add_assign(piece_square_bonus_lookup(piece, to));
    }

    /// Check if moved/captured piece was a castle rook, and invalidate castle rights if necessary
    fn invalidate_rook_castle_move(&mut self, from: Square, piece: SidedPiece) {
        if piece.piece_type == PieceType::Rook {
            let king_rook_square= self.state.castles().rook_from_square(piece.player, CastleDirection::KingSide);
            let queen_rook_square= self.state.castles().rook_from_square(piece.player, CastleDirection::QueenSide);
            // Check if either rook is no longer on the target square and remove castle rights
            if from == king_rook_square && CastleRights::for_side(piece.player).can_castle(piece.player, CastleDirection::KingSide) {
                self.state.remove_castle_rights(CastleRights::for_side(piece.player) & CastleRights::BothKings);
            } else if from == queen_rook_square && CastleRights::for_side(piece.player).can_castle(piece.player, CastleDirection::QueenSide) {
                self.state.remove_castle_rights(CastleRights::for_side(piece.player) & CastleRights::BothQueens);
            }
        }
    }

    /// Check if moved piece was a king, and invalidate castle rights if necessary
    fn invalidate_king_castle_move(&mut self, from: Square, piece: SidedPiece) {
        let full_side_castle_rights = CastleRights::for_side(piece.player);
        // Check if the king moved from its starting square and remove castle rights
        if piece.piece_type == PieceType::King
            && from == self.state.castles().king_from_square(piece.player)
            && *self.state.castles().as_ref() & full_side_castle_rights != CastleRights::None {
            self.state.remove_castle_rights(full_side_castle_rights);
        }
    }

    pub(super) fn update_checkers(&mut self, gives_check: bool) {
        self.state.checkers = if gives_check {
            let attackers = self.side_mask(self.side_to_move);
            let defending_king_square = self.king_square(self.side_to_move.switch()).expect("missing king in legal position");

            self.attackers_to(defending_king_square, self.occupied()) & attackers
        } else {
            Bitboard::EMPTY
        };
    }

    pub(super) fn update_move_gen_masks(&mut self) {
        // Update blockers and pinners
        self.update_blockers_pinners_for_side(Player::White);
        self.update_blockers_pinners_for_side(Player::Black);
        // Update masks for squares by piece that give check
        self.update_check_squares();
    }

    fn update_blockers_pinners_for_side(&mut self, side: Player) {
        let attackers = self.side_mask(side.switch());
        let king_square = self.king_square(side).expect("missing king in legal position");
        let mut blockers = Bitboard::EMPTY;
        let mut pinners = Bitboard::EMPTY;

        // Get all snipers aligned with the king
        let cardinal_snipers = self.pieces_mask([PieceType::Rook,PieceType::Queen]) & Bitboard::attacks_mask(PieceType::Rook, king_square);
        let ordinal_snipers = self.pieces_mask([PieceType::Bishop,PieceType::Queen]) & Bitboard::attacks_mask(PieceType::Bishop, king_square);
        let all_snipers = cardinal_snipers | ordinal_snipers;
        let mut attacking_snipers = attackers & all_snipers;
        // All pieces, for both sides, that aren't attacking snipers
        let non_attacking_sniper_mask = self.occupied() ^ attacking_snipers;

        // Iterate over all attacking snipers and see if there is only a single blocker
        while let Some(sniper_square) = attacking_snipers.pop_square() {
            let blockers_mask = Bitboard::line_between(king_square, sniper_square) & non_attacking_sniper_mask;
            let has_single_blocker = blockers_mask.0.count_ones() == 1;

            if has_single_blocker {
                blockers |= blockers_mask;
                // If the blocker is also the same side as the attacking sniper, save the sniper as a pinner
                if blockers_mask & attackers != Bitboard::EMPTY {
                    pinners |= sniper_square.to_mask();
                }
            }
        }

        *self.state.blockers_for.mut_side(side) = blockers;
        *self.state.pinners_for.mut_side(side) = pinners;
    }

    fn update_check_squares(&mut self) {
        let defending_side = self.side_to_move.switch();
        let defending_king_square = self.king_square(defending_side).expect("missing king in legal position");

        // Update non-sliders
        *self.state.check_squares.mut_piece(PieceType::Pawn) = Bitboard::pawn_attacks_mask(defending_king_square, defending_side);
        *self.state.check_squares.mut_piece(PieceType::Knight) = Bitboard::attacks_mask(PieceType::Knight, defending_king_square);

        // Update sliders
        let occupied_mask = self.occupied();
        let ordinal_attacks = Bitboard::occluded_attacks_mask(PieceType::Bishop, defending_king_square, occupied_mask);
        let cardinal_attacks = Bitboard::occluded_attacks_mask(PieceType::Rook, defending_king_square, occupied_mask);
        *self.state.check_squares.mut_piece(PieceType::Bishop) = ordinal_attacks;
        *self.state.check_squares.mut_piece(PieceType::Rook) = cardinal_attacks;
        *self.state.check_squares.mut_piece(PieceType::Queen) = ordinal_attacks | cardinal_attacks;
    }

    /// Capture a piece
    ///
    /// # Panics
    /// Will panic during debug mode when the capture is invalid.
    /// - If it would panic to remove the `captured_piece` for the opposite side (`side.switch()`)
    /// - If it would panic to move the `piece` from `from` to `to` for `side`
    /// - If the captured piece is a king
    fn capture_piece(&mut self, from: Square, to: Square, piece: SidedPiece, captured_piece_type: PieceType) {
        debug_assert_ne!(captured_piece_type, PieceType::King, "attempting to capture a king");
        debug_assert!(self.piece_on(to).is_some(), "capturing to empty square");
        debug_assert!(self.piece_on(to).unwrap() == captured_piece_type, "captured piece is not on target square");
        // Update the pieces
        self.remove_piece(to, SidedPiece::new(captured_piece_type, piece.player.switch()));
        self.move_piece(from, to, piece);

        // Update state
        self.state.set_capture(captured_piece_type);
        self.state.clear_halfmove_clock();

        // Check if piece that captured was castle rook
        self.invalidate_rook_castle_move(from, piece);
        // Check if piece captured was castle rook
        self.invalidate_rook_castle_move(to, SidedPiece::new(captured_piece_type, piece.player.switch()));
        // Check if piece that captured was the king with castle rights
        self.invalidate_king_castle_move(from, piece);
    }

    /// Make a normal from-to move, that doesn't capture or promote or have any special state handling (ie. double-pawn-push)
    fn quiet_move(&mut self, from: Square, to: Square, piece: SidedPiece) {
        // Update pieces
        self.move_piece(from, to, piece);

        // Check if piece that moved was a castle rook or king to invalidate rights
        self.invalidate_rook_castle_move(from, piece);
        self.invalidate_king_castle_move(from, piece);
    }

    /// Double pawn push (jump the first square) for pawns on their side's starting rank with both forward square unobstructed.
    ///
    /// # Panics
    /// Will panic during debug mode when the from, to, and en-passant squares aren't on the correct ranks, if the jumped square isn't empty, or if the from and to aren't on the same file.
    fn double_pawn_push(&mut self, from: Square, to: Square, en_passant_square: Square, player: Player) {
        debug_assert_eq!(from.file().abs_diff(to.file()), 0, "double pawn push can only move forward");
        debug_assert!(self.piece_on(en_passant_square).is_none(), "double pawn push cannot go through an occupied square");
        debug_assert!(
            (from.rank() == 1 && to.rank() == 3 && en_passant_square.rank() == 2 && player == Player::White) || (from.rank() == 6 && to.rank() == 4 && en_passant_square.rank() == 5 && player == Player::Black),
            "illegal ranks for from, to, or en-passant squares"
        );
        // Update pieces
        self.move_piece(from, to, SidedPiece::new(PieceType::Pawn, player));

        // Update state
        self.state.set_en_passant(en_passant_square);
        self.state.clear_halfmove_clock();
    }

    /// Pawn en-passant capture opposing teams previously double pushed pawn.
    ///
    /// # Panics
    /// Will panic during debug mode when there is no en-passant square in the state, when making an illegal pawn move, if the captured pawn's square isn't correct.
    fn en_passant_capture(&mut self, from: Square, to: Square, pawn_square: Square, player: Player) {
        debug_assert_eq!(from.file().abs_diff(to.file()), 1, "pawn can only capture one square to the left/right");
        debug_assert_eq!(to.file(), pawn_square.file(), "pawn square is on the wrong file");
        debug_assert!(
            (player == Player::White && from.rank() == 4 && to.rank() == 5) || (player == Player::Black && from.rank() == 3 && to.rank() == 2),
            "illegal pawn and to ranks for side moving"
        );
        // Update pieces
        self.move_piece(from, to, SidedPiece::new(PieceType::Pawn, player));
        self.remove_piece(pawn_square, SidedPiece::new(PieceType::Pawn, player.switch()));

        // Update state
        self.state.set_capture(PieceType::Pawn);
        self.state.clear_halfmove_clock();
    }

    /// Castle with rook on the side of the king
    fn castle(&mut self, king_from: Square, king_to: Square, castle_direction: CastleDirection, player: Player) {
        debug_assert!(self.state.castles().as_ref().can_castle(player, castle_direction), "attempting castle without rights");
        let rook_from_square = self.as_ref().castles().rook_from_square(player, castle_direction);
        let rook_to_square = self.as_ref().castles().rook_to_square(player, castle_direction);
        self.move_piece(king_from, king_to, SidedPiece::new(PieceType::King, player));
        self.move_piece(rook_from_square, rook_to_square, SidedPiece::new(PieceType::Rook, player));

        // Remove all rights for this side when castling
        self.state.remove_castle_rights(CastleRights::for_side(player));
    }

    /// Push a pawn to the final rank promoting to a [`PromotionPieceType`](promotion_piece_type::PromotionPieceType)
    fn promote(&mut self, from: Square, to: Square, promotion: PromotionPieceType, player: Player) {
        debug_assert_eq!(from.rank(), if player == Player::White { 6 } else { 1 }, "pawn promotion doesnt start on correct rank");
        debug_assert_eq!(to.rank(), if player == Player::White { 7 } else { 0 }, "pawn promotion doesnt go to correct rank");
        debug_assert_eq!(from.file(), to.file(), "pawn promotion can only move vertically");
        // Update pieces
        self.remove_piece(from, SidedPiece::new(PieceType::Pawn, player));
        self.add_piece(to, SidedPiece::new(promotion.into(), player));

        // Update state
        self.state.clear_halfmove_clock();
    }

    /// Capture towards the final rank with a pawn to promote to a [`PromotionPieceType`](promotion_piece_type::PromotionPieceType)
    fn promoting_capture(&mut self, from: Square, to: Square, promotion: PromotionPieceType, captured_piece: PieceType, player: Player) {
        debug_assert_eq!(from.rank(), if player == Player::White { 6 } else { 1 }, "pawn promotion capture doesnt start on correct rank");
        debug_assert_eq!(to.rank(), if player == Player::White { 7 } else { 0 }, "pawn promotion capture doesnt go to correct rank");
        debug_assert_eq!(from.file().abs_diff(to.file()), 1, "pawn promotion capture must move diagonally forward");
        // Update pieces
        self.remove_piece(from, SidedPiece::new(PieceType::Pawn, player));
        self.remove_piece(to, SidedPiece::new(captured_piece, player.switch()));
        self.add_piece(to, SidedPiece::new(promotion.into(), player));

        // Update state
        self.state.clear_halfmove_clock();

        // Make sure rook wasn't captured to invalidate castle rights
        self.invalidate_rook_castle_move(to, SidedPiece::new(captured_piece, player.switch()));
    }

    /// Perform a legal [move](ChessMove) on the board returning a copy of the [`State`] from before the move was made.
    pub fn make_move(&mut self, chess_move: ChessMove) -> State {
        // Create copy of current state
        let previous_state = self.state;
        let gives_check = self.gives_check(chess_move);

        // Clear previous state
        self.state.clear_en_passant();
        self.state.clear_capture();

        // Increment move counter
        self.halfmove_count += 1;

        // Increment the clock before we move in case we reset it
        self.state.increment_halfmove_clock();

        let ChessMove { move_type, from, to } = chess_move;
        match move_type {
            MoveType::Quiet { piece_type } => self.quiet_move(from, to, SidedPiece::new(piece_type, self.side_to_move)),
            MoveType::DoublePawnPush { en_passant_square } => self.double_pawn_push(from, to, en_passant_square, self.side_to_move),
            MoveType::Capture { piece_type, captured_piece } => self.capture_piece(from, to, SidedPiece::new(piece_type, self.side_to_move), captured_piece),
            MoveType::EnPassantCapture { captured_pawn_square } => self.en_passant_capture(from, to, captured_pawn_square, self.side_to_move),
            MoveType::Castle { castle_direction } => self.castle(from, to, castle_direction, self.side_to_move),
            MoveType::Promotion { promotion } => self.promote(from, to, promotion, self.side_to_move),
            MoveType::PromotingCapture { promotion, captured_piece } => self.promoting_capture(from, to, promotion, captured_piece, self.side_to_move),
        }

        // Update check masks
        self.update_checkers(gives_check);

        // Switch side for the next player
        self.switch_sides();

        // Update checkers, pinners, pinned, and other move gen state
        self.update_move_gen_masks();

        // Give the previous state so we can undo irreversible state
        previous_state
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use test::{black_box, Bencher};
    use crate::board::Board;
    use crate::chess_move::ChessMove;
    use crate::move_type::MoveType;
    use crate::piece_type::PieceType;
    use crate::player::Player;
    use crate::sided_piece::SidedPiece;
    use crate::square::Square;
    use crate::square::Square::E2;

    #[bench]
    fn make_startpos_e2e4_bench(bencher: &mut Bencher) {
        let chess_move = black_box(ChessMove { from: Square::E2, to: Square::E4, move_type: MoveType::DoublePawnPush { en_passant_square: Square::E3 } });
        bencher.iter(|| black_box(Board::starting_position()).make_move(chess_move));
    }

    #[bench]
    fn update_blockers_pinners_for_side_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).update_blockers_pinners_for_side(Player::White));
        bencher.iter(|| black_box(Board::starting_position()).update_blockers_pinners_for_side(Player::Black));
    }

    #[bench]
    fn update_check_squares_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).update_check_squares());
    }

    #[bench]
    fn invalidate_rook_castle_move_e2e4_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).invalidate_rook_castle_move(black_box(E2), black_box(SidedPiece::new(PieceType::Pawn, Player::White))));
        // TODO: Try with rook
    }

    #[bench]
    fn invalidate_king_castle_move_e2e4_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).invalidate_king_castle_move(black_box(E2), black_box(SidedPiece::new(PieceType::Pawn, Player::White))));
        // TODO: Try with king
    }

    #[bench]
    fn update_checkers_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).update_checkers(black_box(true)));
    }

    #[bench]
    fn update_move_gen_masks_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| black_box(Board::starting_position()).update_move_gen_masks());
    }

}