use crate::uci::{SearchParameters, UciConfig, UciPosition};

pub enum UciCommand {
    Uci,
    IsReady,
    Debug(bool),
    UciNewGame,
    SetOption(UciConfig),
    Position(UciPosition),
    Go(SearchParameters),
    PonderHit,
    Stop,
    Other(String),
}

#[macro_export]
macro_rules! unknown_command_error {
    ($input:expr) => {{
        eprintln!("unrecognized command: '{}'", $input);
    }};
}