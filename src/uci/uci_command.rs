use crate::uci::{SearchParameters, UciConfig, UciPosition};

/// Incoming command from the GUI
#[derive(Clone, Eq, PartialEq, Debug)]
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

/// Print unrecognized command error message
#[macro_export]
macro_rules! unknown_command_error {
    ($input:expr) => {{
        eprintln!("unrecognized command: '{}'", $input);
    }};
}