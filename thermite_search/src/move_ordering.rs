use std::collections::HashMap;
use thermite_core::board::Board;
use thermite_core::chess_move::ChessMove;
use thermite_core::move_type::MoveType;
use thermite_core::piece_type::PieceType;
use thermite_core::player::{ByPlayer, Player};
use thermite_core::PlyCount;
use thermite_core::promotion_piece_type::PromotionPieceType;
use thermite_core::square::BySquare;
use thermite_movegen::LegalMoveContainer;
use crate::NodeCount;

impl MoveOrderingState {
    /// TODO
    #[cfg(feature = "killer_heuristic")]
    pub(crate) fn add_killer_move(&mut self, chess_move: ChessMove) {
        if let Some(killer_entry) = self.killer_table.get_mut(&chess_move) {
            *killer_entry += 1;
        } else {
            self.killer_table.insert(chess_move, 1);
        }
    }

    /// TODO
    #[cfg(feature = "history_heuristic")]
    pub(crate) fn update_history_table(&mut self, side_to_move: Player, chess_move: ChessMove, depth: PlyCount) {
        let entry = self.history_table.mut_side(side_to_move).mut_square(chess_move.from).mut_square(chess_move.to);
        *entry += depth * depth;
    }

    pub(crate) fn sort_moves(&self, move_list: &mut LegalMoveContainer, board: &Board) {
        // TODO: Take some list of PV moves or TT modify hash and lookup PV or cut nodes to push to the top
        move_list.sort_by_cached_key(|&chess_move| self.score_chess_move(chess_move, board));
        move_list.reverse();
    }

    fn score_chess_move(&self, chess_move: ChessMove, board: &Board) -> MoveScoreType {
        let mut move_score = 0;
        #[cfg(feature = "killer_heuristic")] {
            let killer_move_bonus = if let Some(&kills) = self.killer_table.get(&chess_move) { kills as MoveScoreType * 500_000 } else { 0 };
            move_score += killer_move_bonus;
        }

        let move_type_bonus = match chess_move.move_type {
            MoveType::Quiet { .. } | MoveType::DoublePawnPush { .. } => self.score_quiet(chess_move, board.side_to_move()),
            MoveType::Capture { captured_piece, .. } => Self::piece_value(captured_piece),
            MoveType::EnPassantCapture { .. } => Self::piece_value(PieceType::Pawn),
            MoveType::PromotingCapture { promotion, captured_piece } => Self::promotion_piece_value(promotion) + Self::piece_value(captured_piece),
            MoveType::Castle { .. } => 20_000,
            MoveType::Promotion { promotion } => Self::promotion_piece_value(promotion),
        };
        move_score += move_type_bonus;

        let check_bonus = if board.gives_check(chess_move) { 300_000 } else { 0 };
        move_score += check_bonus;

        move_score
    }

    fn score_quiet(&self, chess_move: ChessMove, side_to_move: Player) -> MoveScoreType {
        let mut score = 0;
        #[cfg(feature = "history_heuristic")]
        {
            let history_bonus = *self.history_table.get_side(side_to_move).get_square(chess_move.from).get_square(chess_move.to) as MoveScoreType;
            score += history_bonus;
        }

        // TODO: Piece square table

        score
    }

    fn piece_value(piece_type: PieceType) -> MoveScoreType {
        match piece_type {
            PieceType::Pawn => 10_000,
            PieceType::Knight | PieceType::Bishop => 30_000,
            PieceType::Rook => 50_000,
            PieceType::Queen => 90_000,
            PieceType::King => panic!("King cannot be valued"),
        }
    }

    fn promotion_piece_value(promotion: PromotionPieceType) -> MoveScoreType {
        match promotion {
            PromotionPieceType::Queen => 300_000,
            PromotionPieceType::Rook => 10_000,
            PromotionPieceType::Bishop => 20_000,
            PromotionPieceType::Knight => 250_000,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct MoveOrderingState {
    /// TODO
    #[cfg(feature = "killer_heuristic")]
    killer_table: HashMap<ChessMove, NodeCount>,
    /// TODO
    #[cfg(feature = "history_heuristic")]
    history_table: ByPlayer<BySquare<BySquare<PlyCount>>>,
}

/// TODO
type MoveScoreType = i32;
