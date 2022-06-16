use crate::engine_types::Score;
use std::fmt::{Display, Formatter};

/// The application of a score in terms of its range limitations.
///
/// Used to indicate a range of possible scores.
#[derive(Copy, Clone, Debug)]
pub enum ScoreBoundsType {
    /// The score represents an exact evaluation
    Exact,
    /// The score represents an upper bound, or maximum value
    Upper,
    /// The score represents a lower bound, or minimum value
    Lower,
}

/// A [score](Score) with its [bounds type](ScoreBoundsType)
#[derive(Copy, Clone, Debug)]
pub struct UciScore {
    score: Score,
    bounds_type: ScoreBoundsType,
}

impl UciScore {
    /// Create a new score
    pub fn new(score: Score, bounds_type: ScoreBoundsType) -> Self {
        Self { score, bounds_type }
    }
}

impl Display for UciScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.bounds_type {
            ScoreBoundsType::Exact => write!(f, "{}", self.score),
            ScoreBoundsType::Upper => write!(f, "upperbound {}", self.score),
            ScoreBoundsType::Lower => write!(f, "lowerbound {}", self.score),
        }
    }
}
