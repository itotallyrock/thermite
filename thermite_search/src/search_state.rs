use std::time::Instant;
use thermite_core::PlyCount;
use crate::NodeCount;

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct SearchState {
    /// When the search was started
    pub started_at: Option<Instant>,
    /// How many positions, or nodes, this search has covered (including quiescent nodes)
    pub nodes: NodeCount,
    /// The maximum depth searched to
    pub max_depth: PlyCount,
    /// The current depth the search is searching at
    pub search_depth: PlyCount,
    /// If multi-pv which PV index are we on
    #[cfg(feature = "multipv")]
    pub pv_index: usize,
    #[cfg(feature = "q_search")]
    pub q_search_depth: PlyCount,
}

impl SearchState {
    /// Increment the number of visited [nodes](NodeCount)
    pub fn increment_nodes(&mut self) {
        self.nodes = self.nodes.saturating_add(1);
    }

    /// Increment the current [search depth](PlyCount), if higher than the maximum depth reached increment that as well
    pub fn increment_search_depth(&mut self, #[cfg(feature = "q_search")] is_q_search: bool) {
        self.search_depth = self.search_depth.saturating_add(1);
        if self.search_depth > self.max_depth {
            self.max_depth = self.search_depth;
        }

        #[cfg(feature = "q_search")]
        if is_q_search || self.q_search_depth > 0 {
            self.q_search_depth = self.q_search_depth.saturating_add(1);
        }
    }
    /// Reduce the current [search depth](PlyCount)
    pub fn decrement_search_depth(&mut self) {
        self.search_depth = self.search_depth.saturating_sub(1);

        #[cfg(feature = "q_search")]
        if self.q_search_depth > 0 {
            self.q_search_depth = self.q_search_depth.saturating_sub(1);
        }
    }
    /// Get the current [search depth](PlyCount)
    pub fn current_depth(&self) -> PlyCount {
        self.search_depth
    }

    /// How many [nodes](NodeCount) has the search visited per second, on average, over the length of the search
    pub fn nodes_per_second(&self) -> NodeCount {
        (self.nodes as f64 / self.started_at.unwrap().elapsed().as_secs_f64()) as NodeCount
    }
}
