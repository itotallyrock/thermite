use std::io::{BufReader, stdin, stdout};
use crate::uci::{UciChessEngine, UciCommand, UciDriver, UciResponse};

pub struct Driver;

impl Driver {
    /// Block the current thread receiving UCI commands and triggering engine evaluation
    pub fn start<E: UciChessEngine>(mut chess_engine: E) -> anyhow::Result<()> {
        let mut uci_driver = UciDriver::start(BufReader::new(stdin()), stdout());

        for command in uci_driver.receiver.iter() {
            match command {
                UciCommand::IsReady => {
                    // Setup the chess engine and once its completed tell the GUI we're ready
                    chess_engine.setup();
                    uci_driver.uci_writer.respond(UciResponse::ReadyOk)?;
                },
                UciCommand::Quit => {
                    // Shutdown the chess engine because quit was received
                    chess_engine.shutdown();
                    break;
                },
                UciCommand::SetOption(config) => {
                    chess_engine.set_option(config);
                },
            }
        }

        uci_driver.shutdown()
    }
}