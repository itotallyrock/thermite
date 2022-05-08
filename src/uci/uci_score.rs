use crate::engine_types::Score;

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