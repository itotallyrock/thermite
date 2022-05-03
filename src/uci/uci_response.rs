use crate::uci::UciOption;

pub enum UciResponse {
    EngineName(String),
    EngineAuthors(String),
    Option(UciOption),
    UciOk,
    ReadyOk,
}