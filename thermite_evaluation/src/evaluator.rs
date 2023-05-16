use thermite_core::board::Board;
use thermite_core::piece_type::{ByPieceType, NUM_PIECE_TYPES, PieceType};
use thermite_core::player::Player;
use thermite_core::score::{PositionEvaluation, TaperedPawnApproximationEvaluation};

pub trait Evaluator {
    fn piece_value(&self, piece: PieceType) -> PositionEvaluation;
    fn material_evaluation(&self) -> PositionEvaluation;
}

const PIECE_VALUES: ByPieceType<TaperedPawnApproximationEvaluation> = ByPieceType::new_with(
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(126), end_game: PositionEvaluation::new_centipawns(208) },
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(781), end_game: PositionEvaluation::new_centipawns(854) },
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(825), end_game: PositionEvaluation::new_centipawns(915) },
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(1276), end_game: PositionEvaluation::new_centipawns(1380) },
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(2538), end_game: PositionEvaluation::new_centipawns(2682) },
    TaperedPawnApproximationEvaluation { mid_game: PositionEvaluation::new_centipawns(0), end_game: PositionEvaluation::new_centipawns(0) },
);

impl Evaluator for Board {
    fn piece_value(&self, piece: PieceType) -> PositionEvaluation {
        assert_ne!(piece, PieceType::King);
        PIECE_VALUES.get_piece(piece).evaluate(self.sided_piece_counts.game_stage())
    }

    fn material_evaluation(&self) -> PositionEvaluation {
        const MATERIAL_PIECES: [PieceType; NUM_PIECE_TYPES - 1] = [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen];

        MATERIAL_PIECES.into_iter()
            .flat_map(|piece| Player::PLAYERS.into_iter().map(move |side| (side, piece)))
            .map(|(side, piece)| {
                let piece_count = *self.sided_piece_counts.piece_counts.get_side(side).get_piece(piece);
                let side_relative_eval = self.piece_value(piece) * piece_count;
                if self.side_to_move() == side {
                    side_relative_eval
                } else {
                    -side_relative_eval
                }
            })
            .sum()
    }
}