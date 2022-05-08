use crate::uci::UciConfig;

pub enum UciCommand {
    Uci,
    IsReady,
    SetOption(UciConfig),
    Other(String),
}

#[macro_export]
macro_rules! unknown_command_error {
    ($input:expr) => {{
        eprintln!("unrecognized command: '{}'", $input);
    }};
}