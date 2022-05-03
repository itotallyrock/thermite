use std::num::NonZeroUsize;
use crate::uci::{SearchParameters, SearchResult, UciChessEngine, UciConfig, UciOption, UciOptionType};

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
        "Thermite".into()
    }

    fn authors() -> String {
        "Jeffrey Meyer".into()
    }

    fn available_options() -> Vec<UciOption> {
        let max_parallelism = std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get() as _;
        vec![
            UciOption {
                name: "Threads".into(),
                option: UciOptionType::Spin {
                    min: 1,
                    max: max_parallelism,
                    default: max_parallelism,
                }
            }
        ]
    }

    fn set_option(&mut self, config: UciConfig) {
        todo!()
    }

    fn setup(&mut self) {
        // Nothing to do here yet
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