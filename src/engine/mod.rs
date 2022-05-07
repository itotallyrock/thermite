use std::num::NonZeroUsize;
use crate::uci::{SearchParameters, SearchResult, UciChessEngine, UciConfig, UciOption, UciOptionType};

/// Hard maximum number of threads for the container for threads
const MAX_THREADS: usize = 32;
/// Minimum number of search threads and default if available parallelism is unknown
const MIN_THREADS: usize = 1;

/// The name for the engine
const ENGINE_NAME: &str = "Thermite";

/// The authors of the engine, comma separated
const ENGINE_AUTHORS: &str = "Jeffrey Meyer";

/// Thermite chess search engine
pub struct ThermiteEngine {

}

impl ThermiteEngine {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl UciChessEngine for ThermiteEngine {

    fn name() -> String {
        ENGINE_NAME.into()
    }

    fn authors() -> String {
        ENGINE_AUTHORS.into()
    }

    fn available_options() -> Vec<UciOption> {
        let default_parallelism = std::thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(MIN_THREADS).unwrap())
            .get() as _;

        vec![
            UciOption {
                name: "Threads".into(),
                option: UciOptionType::Spin {
                    min: MIN_THREADS as _,
                    max: MAX_THREADS as _,
                    default: default_parallelism,
                }
            }
        ]
    }

    fn set_option(&mut self, config: UciConfig) {
        todo!()
    }

    fn setup(&mut self) {
        // Nothing to do here yet
        // Eventually we'll spawn some search threads and initialize the evaluation and move-gen
    }

    fn start_search(&mut self, params: SearchParameters) {
        todo!()
    }

    fn stop_search(&mut self) -> SearchResult {
        todo!()
    }

    fn shutdown(self) {
        // Nothing to do here yet
        // Eventually we'll need to join the search threads and cleanup held resources
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn min_threads_is_positive_non_zero() {
        assert!(MIN_THREADS >= 1);
    }

    #[test]
    fn min_threads_is_less_than_or_equal_to_max_threads() {
        assert!(MIN_THREADS <= MAX_THREADS);
    }
}