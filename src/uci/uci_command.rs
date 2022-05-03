use crate::uci::UciConfig;

pub enum UciCommand {
    IsReady,
    Quit,
    SetOption(UciConfig)
}