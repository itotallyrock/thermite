use crate::piece_type::{ByPieceType, PieceType};
use crate::PieceCount;
use crate::player::{ByPlayer, Player};
use crate::score::game_stage::GameStageInner;
use crate::score::GameStage;
use crate::sided_piece::SidedPiece;

/// A container for keeping track of piece counts for each [`PieceType`] for each [`Player`]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SidedPieceCounts {
    /// The [`PieceCount`] for a given position by [`ByPlayer`] and [`ByPieceType`]
    pub piece_counts: ByPlayer<ByPieceType<PieceCount>>,
}

impl SidedPieceCounts {
    #[must_use]
    /// Create an empty (no piece counts for either side) [`SidedPieceCounts`]
    pub fn empty() -> Self {
        Self {
            piece_counts: ByPlayer::default(),
        }
    }

    /// Add a given [`PieceType`] for a [`Player`]
    pub const fn add_piece(&mut self, piece: SidedPiece) {
        let SidedPiece { piece_type, player } = piece;
        let piece_count = self.piece_counts.mut_side(player).mut_piece(piece_type);
        *piece_count = piece_count.saturating_add(1);
    }

    /// Remove a given [`PieceType`] for a [`Player`]
    pub const fn remove_piece(&mut self, piece: SidedPiece) {
        let SidedPiece { piece_type, player } = piece;
        let piece_count = self.piece_counts.mut_side(player).mut_piece(piece_type);
        *piece_count = piece_count.saturating_sub(1);
    }

    /// The the number of [pieces](PieceType) on the board for a [`Player`]
    #[must_use]
    pub const fn piece_count(&self, piece: SidedPiece) -> PieceCount {
        let SidedPiece { piece_type, player } = piece;
        *self.piece_counts.get_side(player).get_piece(piece_type)
    }

    /// Compute the [`GameStage`] for the current piece count given a standard starting piece count
    #[must_use]
    pub fn game_stage(&self) -> GameStage {
        /// The initial positional piece counts to normalize against
        const STARTING_PIECE_COUNTS: ByPieceType<PieceCount> = ByPieceType::new_with(8, 2, 2, 2, 1, 1);
        /// How much each `normalized_piece_count` is worth towards the total normalized `game_stage`
        #[allow(clippy::cast_precision_loss)]
        const STEP_WEIGHT: GameStageInner = ((PieceType::PIECES.len() - 1) * Player::PLAYERS.len()) as GameStageInner;

        // Start a 1.0 representing a board with only 2 kings (or empty)
        let mut game_stage: GameStageInner = 1.0;
        // Iterate over each piece (excluding the king) and side to its normalized_piece_count a subtract from the total stage approaching 0.0 if no pieces have been captured
        let mut piece_index = 0;
        let mut side_index = 0;
        while piece_index < PieceType::PIECES.len() {
            let piece_type = PieceType::PIECES[piece_index];
            if piece_type == PieceType::King {
                piece_index += 1;
                continue;
            }

            while side_index < Player::PLAYERS.len() {
                let side = Player::PLAYERS[side_index];
                let piece_count = *self.piece_counts.get_side(side).get_piece(piece_type);
                let normalized_piece_count = piece_count as GameStageInner / *STARTING_PIECE_COUNTS.get_piece(piece_type) as GameStageInner;
                game_stage -= normalized_piece_count / STEP_WEIGHT;
                debug_assert!(game_stage >= 0.0 && game_stage <= 1.0);

                side_index += 1;
            }

            piece_index += 1;
        }

        GameStage(game_stage)
    }
}