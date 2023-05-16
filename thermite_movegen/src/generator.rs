use thermite_core::bitboard::Bitboard;
use thermite_core::board::Board;
use thermite_core::castles::CastleDirection;
#[cfg(feature = "q_moves")]
use thermite_core::move_type::MoveType;
use thermite_core::piece_type::PieceType;
use thermite_core::player::Player;
use thermite_core::sided_piece::SidedPiece;
use crate::create_moves::{create_double_pawn_push_move, create_en_passant_capture, create_move, create_pawn_attack_move, create_pawn_capture_promotions, create_pawn_promotions, create_pawn_push_move};

use crate::LegalMoveContainer;
use crate::move_list::PseudoLegalMoveContainer;

fn generate_moves_for_piece(board: &Board, move_list: &mut PseudoLegalMoveContainer, piece: PieceType, targets: Bitboard) {
    debug_assert!(piece != PieceType::King && piece != PieceType::Pawn);
    let attackers = board.piece_mask(piece) & board.side_mask(board.side_to_move());
    let occupied = board.occupied();
    let piece_moves = attackers
        .into_iter()
        .flat_map(|from| (targets & Bitboard::occluded_attacks_mask(piece, from, occupied))
            .into_iter()
            .map(move |to| create_move(board, piece, from, to)));
    move_list.extend(piece_moves);
}

fn generate_pawn_moves(board: &Board, move_list: &mut PseudoLegalMoveContainer, targets: Bitboard) {
    let side = board.side_to_move();
    let opposite_side = side.switch();
    let (promoting_from_rank, en_passant_rank) = if side == Player::White {
        (Bitboard::RANKS[6], Bitboard::RANKS[2])
    } else {
        (Bitboard::RANKS[1], Bitboard::RANKS[5])
    };
    let empty = !board.occupied();
    let enemies = board.side_mask(opposite_side) & targets;
    let pawns = board.piece_mask(PieceType::Pawn) & board.side_mask(side);
    let non_promoting_pawns = pawns & !promoting_from_rank;

    let pawn_west_attacks = non_promoting_pawns.pawn_west_attacks(side);
    move_list.extend((pawn_west_attacks & enemies).into_iter().map(|to| create_pawn_attack_move::<true>(board, to, opposite_side)));

    let pawn_east_attacks = non_promoting_pawns.pawn_east_attacks(side);
    move_list.extend((pawn_east_attacks & enemies).into_iter().map(|to| create_pawn_attack_move::<false>(board, to, opposite_side)));

    if let Some(en_passant_square) = board.as_ref().en_passant_square() {
        let en_passant_mask = en_passant_square.to_mask();
        if pawn_west_attacks & en_passant_mask != Bitboard::EMPTY {
            move_list.push(create_en_passant_capture::<true>(en_passant_square, en_passant_mask, opposite_side));
        }
        if pawn_east_attacks & en_passant_mask != Bitboard::EMPTY {
            move_list.push(create_en_passant_capture::<false>(en_passant_square, en_passant_mask, opposite_side));
        }
    }

    let pushed_pawns = non_promoting_pawns.pawn_push(side) & empty;
    move_list.extend((pushed_pawns & targets).into_iter().map(|to| create_pawn_push_move(to, opposite_side)));

    let double_pushed_pawns = (pushed_pawns & en_passant_rank).pawn_push(side) & empty & targets;
    move_list.extend(double_pushed_pawns.into_iter().map(|to| create_double_pawn_push_move(to, opposite_side)));

    let promotable_pawns = pawns & promoting_from_rank;
    let promoting_pawns = promotable_pawns.pawn_push(side) & empty & targets;
    move_list.extend(promoting_pawns.into_iter().flat_map(|to| create_pawn_promotions(to, opposite_side)));

    let promoting_west_capture_pawns = promotable_pawns.pawn_west_attacks(side) & enemies;
    move_list.extend(promoting_west_capture_pawns.into_iter().flat_map(|to| create_pawn_capture_promotions::<true>(to, SidedPiece::new(board.piece_on(to).unwrap(), opposite_side))));

    let promoting_east_capture_pawns = promotable_pawns.pawn_east_attacks(side) & enemies;
    move_list.extend(promoting_east_capture_pawns.into_iter().flat_map(|to| create_pawn_capture_promotions::<false>(to, SidedPiece::new(board.piece_on(to).unwrap(), opposite_side))));
}

macro_rules! generate_castle {
    ($castles:ident, $direction:expr, $side:expr, $board:ident, $move_list:ident) => {{
        use $crate::create_moves::create_castle_move;
        if $castles.as_ref().can_castle($side, $direction) && $castles.get_unoccupied_path($side, $direction) & $board.occupied() == Bitboard::EMPTY {
            $move_list.push(create_castle_move($castles, $side, $direction));
        }
    }};
}

fn generate_king_moves(board: &Board, move_list: &mut PseudoLegalMoveContainer, targets: Bitboard) {
    let side = board.side_to_move();
    let king_square = board.king_square(side).expect("missing king in legal position");
    let king_attacks = Bitboard::attacks_mask(PieceType::King, king_square) & targets;
    move_list.extend(king_attacks.into_iter().map(|to| create_move(board, PieceType::King, king_square, to)));

    // Generate castles
    let castles = board.as_ref().castles();
    generate_castle!(castles, CastleDirection::KingSide, side, board, move_list);
    generate_castle!(castles, CastleDirection::QueenSide, side, board, move_list);
}

fn generate_non_king_moves(board: &Board, move_list: &mut PseudoLegalMoveContainer, targets: Bitboard) {
    generate_pawn_moves(board, move_list, targets);
    generate_moves_for_piece(board, move_list, PieceType::Knight, targets);
    generate_moves_for_piece(board, move_list, PieceType::Bishop, targets);
    generate_moves_for_piece(board, move_list, PieceType::Rook, targets);
    generate_moves_for_piece(board, move_list, PieceType::Queen, targets);
}

fn generate_king_evasions(board: &Board, move_list: &mut PseudoLegalMoveContainer) {
    let side = board.side_to_move();
    let king_square = board.king_square(side).expect("missing king in legal position");
    let king_attacks = Bitboard::attacks_mask(PieceType::King, king_square) & !board.side_mask(side);
    move_list.extend(king_attacks.into_iter().map(|to| create_move(board, PieceType::King, king_square, to)));
}

fn generate_evasions(board: &Board, move_list: &mut PseudoLegalMoveContainer) {
    let checkers = board.checkers();
    debug_assert_ne!(checkers, Bitboard::EMPTY);

    // If we are only checked by a single piece we can generate all moves that block or capture it
    // Otherwise, only the king can move to remove the double-check
    if checkers.num_squares() == 1 {
        let king_square = board.king_square(board.side_to_move()).expect("missing king in legal position");
        let checker_square = checkers.clone().pop_square().unwrap();
        let targets = Bitboard::line_between(king_square, checker_square);
        generate_non_king_moves(board, move_list, targets);
    }

    // Generate king moves
    generate_king_evasions(board, move_list);
}

fn generate_non_evasions(board: &Board, move_list: &mut PseudoLegalMoveContainer) {
    let targets = !board.side_mask(board.side_to_move());
    generate_non_king_moves(board, move_list, targets);
    generate_king_moves(board, move_list, targets);
}

/// Extension to allow generating moves on a board
pub trait MoveGenerator {
    /// Get all of the legal/playable [moves](thermite_core::chess_move::ChessMove) for the [player](Player) to move
    fn generate_legal(&self) -> LegalMoveContainer;
    #[cfg(feature = "q_moves")]
    fn generate_quiescent_moves(&self) -> LegalMoveContainer;
}

impl MoveGenerator for Board {
    fn generate_legal(&self) -> LegalMoveContainer {
        let mut pseudo_legal_move_list = PseudoLegalMoveContainer::default();
        if self.in_check() {
            generate_evasions(self, &mut pseudo_legal_move_list);
        } else {
            generate_non_evasions(self, &mut pseudo_legal_move_list);
        }
        // Filter out illegal moves
        pseudo_legal_move_list.into_iter().filter_map(|m| m.into_legal(self)).collect()
    }

    #[cfg(feature = "q_moves")]
    fn generate_quiescent_moves(&self) -> LegalMoveContainer {
        // TODO: Generate these smarter
        let mut move_list = self.generate_legal();
        move_list.retain(|m| self.gives_check(*m) || matches!(m.move_type, MoveType::Capture { .. } | MoveType::EnPassantCapture { .. } | MoveType::PromotingCapture { .. }));

        move_list
    }
}