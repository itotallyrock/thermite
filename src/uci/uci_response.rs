use crate::uci::{SearchResult, UciOption};

/// Reply sent to the GUI from this chess engine
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UciResponse {
    EngineName(String),
    EngineAuthors(String),
    Option(UciOption),
    UciOk,
    ReadyOk,
    BestMove(SearchResult),
}