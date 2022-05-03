use std::io::{BufReader, stdin, stdout};
use crate::uci::{UciChessEngine, UciCommand, UciDriver, UciResponse};

pub struct Driver;

impl Driver {
    /// Block the current thread receiving UCI commands and triggering engine evaluation
    pub fn start<E: UciChessEngine>(mut chess_engine: E) -> anyhow::Result<()> {
        let mut uci_driver = UciDriver::start(BufReader::new(stdin()), stdout());

        for command in uci_driver.receiver.iter() {
            match command {

                UciCommand::Uci => {
                    // Respond with engine details
                    uci_driver.uci_writer.respond(UciResponse::EngineName(E::name()))?;
                    uci_driver.uci_writer.respond(UciResponse::EngineAuthors(E::authors()))?;

                    // Respond with available options
                    for uci_option in E::available_options() {
                        uci_driver.uci_writer.respond(UciResponse::Option(uci_option))?;
                    }

                    // This engine driver only supports UCI so always respond with ok
                    uci_driver.uci_writer.respond(UciResponse::UciOk)?;
                },

                UciCommand::IsReady => {
                    // Setup the chess engine and once its completed tell the GUI we're ready
                    chess_engine.setup();
                    uci_driver.uci_writer.respond(UciResponse::ReadyOk)?;
                },

                UciCommand::SetOption(config) => {
                    chess_engine.set_option(config);
                },
            }
        }

        chess_engine.shutdown();
        uci_driver.shutdown()
    }
}