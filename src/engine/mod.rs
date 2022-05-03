use crate::uci::{SearchParameters, SearchResult, UciChessEngine, UciConfig, UciOption};

pub struct ThermiteEngine {

}

impl ThermiteEngine {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl UciChessEngine for ThermiteEngine {

    fn available_options() -> Vec<UciOption> {
        todo!()
    }

    fn set_option(&mut self, config: UciConfig) {
        todo!()
    }

    fn setup(&mut self) {

    }

    fn start_search(&mut self, params: SearchParameters) {
        todo!()
    }

    fn stop_search(&mut self) -> SearchResult {
        todo!()
    }

    fn shutdown(self) {
        todo!()
    }
}