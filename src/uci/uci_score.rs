use crate::engine_types::Score;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum ScoreBoundsType {
    Exact,
    Upper,
    Lower,
}

#[derive(Clone, Debug)]
pub struct UciScore {
    pub score: Score,
    pub bounds_type: ScoreBoundsType,
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
