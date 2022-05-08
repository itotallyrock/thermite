use std::io::Write;

use crate::uci::{SearchResult, UciInfo, UciResponse};
use crate::uci::uci_options::UciOptionType;

pub struct UciWriter<W: Write> {
    writer: W,
}


impl<W: Write> UciWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer
        }
    }
    pub fn debug(&mut self, info: UciInfo) -> std::io::Result<()> {
        match info {
            UciInfo::NodesPerSecond(nodes_per_second) => write!(self.writer, "info nodes {}", nodes_per_second),
        }
    }
    pub fn respond(&mut self, response: UciResponse) -> std::io::Result<()> {
        match response {
            UciResponse::ReadyOk => writeln!(self.writer, "readyok")?,
            UciResponse::UciOk => writeln!(self.writer, "uciok")?,
            UciResponse::EngineName(engine_name) => writeln!(self.writer, "id name {}", engine_name)?,
            UciResponse::EngineAuthors(authors) => writeln!(self.writer, "id author {}", authors)?,
            UciResponse::Option(uci_option) => match uci_option.option {
                UciOptionType::Button => writeln!(self.writer, "option name {} type button", uci_option.name)?,
                UciOptionType::Check { default } => writeln!(self.writer, "option name {} type check default {}", uci_option.name, default)?,
                UciOptionType::Spin { min, max, default } => writeln!(self.writer, "option name {} type spin min {} max {} default {}", uci_option.name, min, max, default)?,
                UciOptionType::Combo { options, default } => writeln!(self.writer, "option name {} type combo {} default {}", uci_option.name, options.into_iter().map(|variant| format!("var {}", variant)).collect::<String>(), default)?,
                UciOptionType::String { default } => writeln!(self.writer, "option name {} type string default {}", uci_option.name, default)?,
            },
            UciResponse::BestMove(SearchResult { best_move, ponder_move }) => match ponder_move {
                None => writeln!(self.writer, "bestmove {}", best_move)?,
                Some(ponder_move) => writeln!(self.writer, "bestmove {} ponder {}", best_move, ponder_move)?,
            },
        }
        self.writer.flush()
    }
}
