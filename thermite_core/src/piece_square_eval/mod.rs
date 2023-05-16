use crate::piece_type::{ByPieceType, NUM_PIECE_TYPES, PieceType};
use crate::player::{ByPlayer, NUM_PLAYERS, Player};
use crate::score::TaperedPawnApproximationEvaluation;
use crate::square::{BySquare, NUM_SQUARES, Square};

use lazy_static::lazy_static;
use crate::sided_piece::SidedPiece;

lazy_static! {
    /// TODO
    static ref PIECE_SQUARE_BONUSES: ByPieceType<BySquare<TaperedPawnApproximationEvaluation>> = ByPieceType::from([BySquare::from([TaperedPawnApproximationEvaluation::EMPTY; NUM_SQUARES]); NUM_PIECE_TYPES]);// TODO: White POV tables

    /// TODO
    static ref PIECE_SQUARE_TABLE: BySquare<ByPieceType<ByPlayer<TaperedPawnApproximationEvaluation>>> = {
        let mut table: BySquare<ByPieceType<ByPlayer<TaperedPawnApproximationEvaluation>>> = BySquare::from([ByPieceType::from([ByPlayer::from([TaperedPawnApproximationEvaluation::EMPTY; NUM_PLAYERS]); NUM_PIECE_TYPES]); NUM_SQUARES]);
        let mut white_square_index = 0;
        while white_square_index < NUM_SQUARES {
            let white_square = Square::SQUARES[white_square_index];
            let black_square = Square::SQUARES[(NUM_SQUARES - 1) - white_square_index];

            let mut piece_index = 0;
            while piece_index < NUM_PIECE_TYPES {
                let piece = PieceType::PIECES[piece_index];
                let piece_bonuses = PIECE_SQUARE_BONUSES.get_piece(piece);
                let whites_pov = *piece_bonuses.get_square(white_square);
                let blacks_pov = *piece_bonuses.get_square(black_square);

                *table.mut_square(white_square).mut_piece(piece).mut_side(Player::White) = whites_pov;
                *table.mut_square(white_square).mut_piece(piece).mut_side(Player::Black) = blacks_pov;

                piece_index += 1;
            }
            white_square_index += 1;
        }

        table
    };
}

/// Lookup the piece-square bonus evaluation for a piece placed on a square for a given side
#[must_use]
pub(crate) fn piece_square_bonus_lookup(piece: SidedPiece, square: Square) -> TaperedPawnApproximationEvaluation {
    *PIECE_SQUARE_TABLE.get_square(square).get_piece(piece.piece_type).get_side(piece.player)
}
