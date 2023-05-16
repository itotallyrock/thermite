use thermite_core::bitboard::Bitboard;
use thermite_core::board::Board;
use thermite_core::castles::{CastleDirection, Castles};
use thermite_core::chess_move::ChessMove;
use thermite_core::move_type::MoveType;
use thermite_core::piece_type::PieceType;
use thermite_core::player::Player;
use thermite_core::promotion_piece_type::PromotionPieceType;
use thermite_core::sided_piece::SidedPiece;
use thermite_core::square::Square;

use crate::pseudo_legal_move::PseudoLegalChessMove;

pub(crate) fn create_castle_move(castles: Castles, side: Player, castle_direction: CastleDirection) -> PseudoLegalChessMove {
    let from = castles.king_from_square(side);
    let to = castles.king_to_square(side, castle_direction);
    let move_type = MoveType::Castle { castle_direction };

    PseudoLegalChessMove::new(ChessMove {
        move_type,
        from,
        to,
    })
}

pub(crate) fn create_pawn_promotions(to: Square, opposite_side: Player) -> impl Iterator<Item=PseudoLegalChessMove> {
    PromotionPieceType::ALL.into_iter().map(move |promotion| PseudoLegalChessMove::new(ChessMove {
        from: to.to_mask().pawn_push(opposite_side).pop_square().unwrap(),
        to,
        move_type: MoveType::Promotion { promotion },
    }))
}

pub(crate) fn create_pawn_capture_promotions<const IS_WEST: bool>(to: Square, captured_piece: SidedPiece) -> impl Iterator<Item=PseudoLegalChessMove> {
    let SidedPiece { piece_type: captured_piece, player: opposite_side } = captured_piece;
    PromotionPieceType::ALL.into_iter().map(move |promotion| {
        let to_mask = to.to_mask();
        let from = if IS_WEST { to_mask.pawn_east_attacks(opposite_side) } else { to_mask.pawn_west_attacks(opposite_side) }.pop_square().unwrap();
        let move_type = MoveType::PromotingCapture { promotion, captured_piece };

        PseudoLegalChessMove::new(ChessMove {
            from,
            to,
            move_type,
        })
    })
}

pub(crate) fn create_double_pawn_push_move(to: Square, opposite_side: Player) -> PseudoLegalChessMove {
    let en_passant_mask = to.to_mask().pawn_push(opposite_side);
    let en_passant_square = en_passant_mask.clone().pop_square().unwrap();
    let from = en_passant_mask.pawn_push(opposite_side).pop_square().unwrap();
    let move_type = MoveType::DoublePawnPush { en_passant_square };

    PseudoLegalChessMove::new(ChessMove {
        from,
        to,
        move_type,
    })
}

pub(crate) fn create_pawn_push_move(to: Square, opposite_side: Player) -> PseudoLegalChessMove {
    let from = to.to_mask().pawn_push(opposite_side).pop_square().unwrap();
    let move_type = MoveType::Quiet { piece_type: PieceType::Pawn };

    PseudoLegalChessMove::new(ChessMove {
        from,
        to,
        move_type,
    })
}

pub(crate) fn create_pawn_attack_move<const IS_WEST: bool>(board: &Board, to: Square, opposite_side: Player) -> PseudoLegalChessMove {
    let to_mask = to.to_mask();
    let from = if IS_WEST { to_mask.pawn_east_attacks(opposite_side) } else { to_mask.pawn_west_attacks(opposite_side) }.pop_square().unwrap();
    let captured_piece = board.piece_on(to).unwrap();
    let move_type = MoveType::Capture { piece_type: PieceType::Pawn, captured_piece};

    PseudoLegalChessMove::new(ChessMove {
        from,
        to,
        move_type,
    })
}

pub(crate) fn create_en_passant_capture<const IS_WEST: bool>(en_passant_square: Square, en_passant_mask: Bitboard, opposite_side: Player) -> PseudoLegalChessMove {
    let from = if IS_WEST { en_passant_mask.pawn_east_attacks(opposite_side) } else { en_passant_mask.pawn_west_attacks(opposite_side) }.pop_square().unwrap();
    let captured_pawn_square = en_passant_mask.pawn_push(opposite_side).pop_square().unwrap();
    let move_type = MoveType::EnPassantCapture { captured_pawn_square };

    PseudoLegalChessMove::new(ChessMove { from, to: en_passant_square, move_type })
}

pub(crate) fn create_move(board: &Board, piece: PieceType, from: Square, to: Square) -> PseudoLegalChessMove {
    debug_assert!(piece != PieceType::Pawn);
    let piece_type = board.piece_on(from).expect("moving from empty square");
    let captured_piece = board.piece_on(to);
    let move_type = if let Some(captured_piece) = captured_piece {
        MoveType::Capture { piece_type, captured_piece }
    } else {
        MoveType::Quiet { piece_type }
    };

    PseudoLegalChessMove::new(ChessMove {
        from,
        to,
        move_type,
    })
}
