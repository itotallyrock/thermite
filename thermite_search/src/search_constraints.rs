use std::time::Duration;
#[cfg(feature = "tracing")]
use tracing::debug;
use thermite_core::PlyCount;
use crate::halt_flag::HaltFlag;
use crate::NodeCount;
use crate::search_state::SearchState;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct SearchConstraints {
    /// How long the search can run for
    time_limit: Duration,
    /// How many nodes the search can visit
    node_limit: NodeCount,
    /// How many layers deep can the search go
    depth_limit: PlyCount,
}

impl SearchConstraints {

    /// Whether or not the search constraints have been met or the halt flag has been set
    pub fn should_halt(&self, state: &SearchState, halt_flag: &HaltFlag) -> bool {
        if halt_flag.is_halted() {
            #[cfg(feature = "tracing")]
            debug!("halting due to set halt flag");
            return true;
        }

        if state.nodes > self.node_limit {
            #[cfg(feature = "tracing")]
            debug!("halting due to node limit {}", self.node_limit);
            return true;
        }

        if let Some(start_time) = state.started_at && start_time.elapsed() > self.time_limit {
            #[cfg(feature = "tracing")]
            debug!("halting due to time limit {:.3}ms", self.time_limit.as_secs_f32());
            return true;
        }

        false
    }

    /// Create new set of infinite search constraints
    pub fn new() -> Self {
        Self {
            time_limit: Duration::MAX,
            node_limit: NodeCount::MAX,
            depth_limit: PlyCount::MAX,
        }
    }

    /// TODO
    pub fn with_depth(mut self, depth_limit: PlyCount) -> Self {
        self.depth_limit = depth_limit;

        self
    }

    /// Create a new set of constraints with the depth reduced by one [ply](PlyCount)
    pub fn with_reduced_depth(mut self) -> Self {
        self.depth_limit = self.depth_limit.saturating_sub(1);

        self
    }

    /// Create a new set of constraints with the depth incremented by one [ply](PlyCount) to allow searching one more move/[ply](PlyCount) deeper
    pub fn with_extended_depth(mut self) -> Self {
        self.depth_limit = self.depth_limit.saturating_add(1);

        self
    }

    /// Create a new set of constraints with a set [time limit](Duration)
    pub fn with_time(mut self, time_limit: Duration) -> Self {
        self.time_limit = time_limit;

        self
    }

    /// Create a new set of constraints with a maximum number of searchable [nodes](NodeCount)
    pub fn with_nodes(mut self, node_limit: NodeCount) -> Self {
        self.node_limit = node_limit;

        self
    }

    /// Get the [depth](PlyCount) limit of the constraints
    pub fn depth(&self) -> PlyCount {
        self.depth_limit
    }

    /// Get the [duration](Duration) limit of the constraints
    pub fn duration(&self) -> Duration {
        self.time_limit
    }

    /// Get the [nodes](NodeCount) limit of the constraints
    pub fn nodes(&self) -> NodeCount {
        self.node_limit
    }
}
