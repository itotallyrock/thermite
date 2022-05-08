use crate::uci::search_parameters::SearchParameters;
use crate::uci::search_result::SearchResult;
use crate::uci::uci_options::{UciConfig, UciOption};
use crate::unknown_command_error;

pub trait UciChessEngine {
    fn name() -> String;
    fn authors() -> String;
    fn available_options() -> Vec<UciOption>;
    fn set_option(&mut self, config: UciConfig);
    fn setup(&mut self);
    fn start_search(&mut self, params: SearchParameters);
    fn stop_search(&mut self) -> SearchResult;
    /// The GUI has asked this engine to exit
    fn shutdown(self);
    /// Function called when an unknown uci command is sent.
    /// Used to handle custom commands from the UCI driver.
    fn custom_command_handler(&mut self, input: &str) {
        unknown_command_error!(input.trim());
    }
}