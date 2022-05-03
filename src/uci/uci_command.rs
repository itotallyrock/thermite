use crate::uci::UciConfig;

pub enum UciCommand {
    Uci,
    IsReady,
    SetOption(UciConfig)
}