use crate::uci::{UciChessEngine, UciCommand, UciDriver, UciResponse};
use std::io::{stdin, stdout, BufReader};

pub struct Driver;

impl Driver {
    /// Block the current thread receiving UCI commands and triggering engine evaluation
    pub fn start<E: UciChessEngine>(mut chess_engine: E) -> anyhow::Result<()> {
        let mut uci_driver = UciDriver::start(BufReader::new(stdin()), stdout());
        let mut is_searching = false;
        let mut is_pondering = false;

        for command in uci_driver.receiver.iter() {
            match command {
                UciCommand::Uci => {
                    // Respond with engine details
                    uci_driver
                        .uci_writer
                        .respond(UciResponse::EngineName(E::name()))?;
                    uci_driver
                        .uci_writer
                        .respond(UciResponse::EngineAuthors(E::authors()))?;

                    // Respond with available options
                    for uci_option in E::available_options() {
                        uci_driver
                            .uci_writer
                            .respond(UciResponse::Option(uci_option))?;
                    }

                    // This engine driver only supports UCI so always respond with ok
                    uci_driver.uci_writer.respond(UciResponse::UciOk)?;
                }

                UciCommand::IsReady => {
                    // Setup the chess engine and once its completed tell the GUI we're ready
                    if !is_searching {
                        chess_engine.setup();
                    }
                    uci_driver.uci_writer.respond(UciResponse::ReadyOk)?;
                }

                UciCommand::SetOption(config) => {
                    chess_engine.set_option(config);
                }

                UciCommand::Debug(enabled) => {
                    chess_engine.set_debug(enabled);
                }

                UciCommand::UciNewGame | UciCommand::Stop | UciCommand::Position(_) => {
                    if is_searching {
                        let search_result = chess_engine.stop_search();
                        is_searching = false;
                        is_pondering = false;
                        uci_driver
                            .uci_writer
                            .respond(UciResponse::BestMove(search_result))?;
                    }
                    match command {
                        UciCommand::UciNewGame => chess_engine.new_game(),
                        UciCommand::Position(position) => chess_engine.set_position(position),
                        _ => unreachable!("inside nested match"),
                    }
                }

                UciCommand::Go(search_parameters) => {
                    is_pondering = search_parameters.ponder;
                    is_searching = true;
                    chess_engine.start_search(search_parameters);
                }

                UciCommand::PonderHit => {
                    if is_pondering {
                        is_pondering = false;
                        chess_engine.ponder_hit();
                    }
                }

                UciCommand::Quit => unreachable!(),

                UciCommand::Other(input) => {
                    chess_engine.custom_command_handler(&input);
                }
            }
        }

        chess_engine.shutdown();
        uci_driver.shutdown()
    }

}
