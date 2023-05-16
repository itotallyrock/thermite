pub use positional_evaluation::PositionEvaluation;
pub use game_stage::GameStage;
pub use sided_piece_counts::SidedPieceCounts;
pub use tapered_pawn_approximation_evaluation::TaperedPawnApproximationEvaluation;

mod positional_evaluation;
mod tapered_pawn_approximation_evaluation;
mod sided_piece_counts;
mod game_stage;

/// The underlying type for the evaluation of a position
pub type EvaluationInner = i32;
