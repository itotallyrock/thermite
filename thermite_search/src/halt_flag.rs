use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct HaltFlag(AtomicBool);

impl HaltFlag {
    /// Send the halt signal
    pub fn halt(&self) {
        self.0.store(true, Ordering::Release)
    }
    /// Clear the halt signal
    pub fn reset(&self) {
        self.0.store(false, Ordering::Release)
    }
    /// If the halt flag has been set
    pub fn is_halted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

#[macro_export]
macro_rules! halt_check {
    ($search_constraints:ident, $state:expr, $halt_flag:expr) => {
        if $search_constraints.should_halt($state, $halt_flag) {
            #[cfg(feature = "tracing")]
            use tracing::info;
            #[cfg(feature = "tracing")]
            info!("halting search");
            return Err(SearchError::Halted);
        }
    };
}