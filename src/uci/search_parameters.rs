use crate::engine_types::{NodeCount, SearchDepth, SimpleMoveList};
use std::time::Duration;

/// General search constraints that apply to all searches.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct SearchConstraints {
    /// Limit the search to a subset of the available starting moves
    pub search_moves: Option<SimpleMoveList>,
    /// Maximum number of positions to explore
    pub max_nodes: Option<NodeCount>,
    /// Maximum number of plies to search in advance
    pub max_depth: Option<SearchDepth>,
    /// Hard maximum for how long to spend searching.
    /// Engine can return the best move sooner than this time, but really doesn't have a reason to.
    pub move_time: Option<Duration>,
    /// Limit search to finding a mate within certain number of moves
    pub mate_plies: Option<SearchDepth>,
}

/// Time controls for a standard or sudden death game
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct TimeControls {
    /// White's total remaining time
    pub white_remaining: Duration,
    /// Black's total remaining time
    pub black_remaining: Duration,
    /// White's move increment
    pub white_increment: Duration,
    /// Black's move increment
    pub black_increment: Duration,
}

/// Limits in for how long the engine should search for
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TimeConstraints {
    /// Run search infinitely until stop is manually triggered
    Infinite,
    /// Standard FIDE clock rules
    Standard {
        time_controls: TimeControls,
        moves_to_go: Option<SearchDepth>,
    },
}

/// Search options and constraints
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SearchParameters {
    /// If the search mode should ponder while opponent makes their moves
    pub ponder: bool,
    /// The time constraints for this search
    pub time_constraints: TimeConstraints,
    /// The general search constraints
    pub search_constraints: SearchConstraints,
}
