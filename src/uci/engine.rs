use crate::uci::search_parameters::SearchParameters;
use crate::uci::search_result::SearchResult;
use crate::uci::uci_options::{UciConfig, UciOption};
use crate::uci::UciPosition;
use crate::unknown_command_error;

pub trait UciChessEngine {

    /// The name for this engine
    fn name() -> String;

    /// The authors for this engine
    fn authors() -> String;

    /// What options this engine can accept
    fn available_options() -> Vec<UciOption> {
        Vec::new()
    }

    /// Enable/disable debug output of info to the GUI
    fn set_debug(&mut self, _: bool) {}

    /// Assign a value to an available option
    fn set_option(&mut self, _: UciConfig) {}

    /// Set the current position. Should not be sent during search.
    fn set_position(&mut self, position: UciPosition);

    /// Clear any internal state specific to a single game.
    ///
    /// Should reset internal state and position data, clear transposition tables, reset evaluation.
    /// Will never be called during a search.
    /// [Self::setup] should be called afterwards.
    fn new_game(&mut self) {}

    /// Do any hard blocking initialization tasks.
    /// Can be called multiple times, will never be called during a search.
    /// Called during startup and after starting a new game [Self::new_game]
    fn setup(&mut self) {}

    /// Search the current position given some search parameters
    fn start_search(&mut self, params: SearchParameters);

    /// Stop the search and provide the best move and optionally the opponents response
    fn stop_search(&mut self) -> SearchResult;

    /// Switch search mode to active instead of ponder using prior search parameters
    /// Called when the opponent made the expected move the engine was pondering.
    fn ponder_hit(&mut self);

    /// The GUI has asked this engine to exit
    fn shutdown(self) where Self: Sized {}

    /// Function called when an unknown uci command is sent.
    /// Used to handle custom commands from the UCI driver.
    fn custom_command_handler(&mut self, input: &str) {
        unknown_command_error!(input.trim());
    }
}