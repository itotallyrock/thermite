use derive_more::{AddAssign, SubAssign};

use crate::score::{GameStage, PositionEvaluation};

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, AddAssign, SubAssign)]
pub struct TaperedPawnApproximationEvaluation {
    /// TODO
    pub mid_game: PositionEvaluation,
    /// TODO
    pub end_game: PositionEvaluation,
}

impl TaperedPawnApproximationEvaluation {
    pub(crate) const EMPTY: Self = Self {
        mid_game: PositionEvaluation::new_centipawns(0),
        end_game: PositionEvaluation::new_centipawns(0),
    };

    /// TODO
    #[must_use]
    pub fn evaluate(&self, game_stage: GameStage) -> PositionEvaluation {
        (self.mid_game * -game_stage) + (self.end_game * game_stage)
    }
}
