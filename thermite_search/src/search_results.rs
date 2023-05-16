use arrayvec::ArrayVec;
use thermite_core::score::PositionEvaluation;
use crate::PvMoveContainer;

/// TODO
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct SearchResult {
    pub principle_variation: PvMoveContainer,
    /// TODO
    pub evaluation: Option<PositionEvaluation>,
}

#[cfg(feature = "multipv")]
pub const MAX_MULTI_PV_LINES: usize = 5;

#[cfg(feature = "multipv")]
pub type SearchResults = ArrayVec<SearchResult, MAX_MULTI_PV_LINES>;
#[cfg(not(feature = "multipv"))]
pub type SearchResults = SearchResult;
