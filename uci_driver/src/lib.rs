#![feature(impl_trait_in_assoc_type)]

use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

use thermite_core::board::Board;
use thermite_core::chess_move::ChessMove;
use thermite_search::search_constraints::SearchConstraints;

pub type UciNumber = i64;
pub type UciString = String;
pub type UciBool = bool;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UciOptionValue {
    Check(UciBool),
    Spin(UciNumber),
    Combo(UciString),
    Button,
    String(UciString),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UciOptionTypeConfiguration {
    Check { default: Option<UciBool> },
    Spin { default: Option<UciNumber>, min: Option<UciNumber>, max: Option<UciNumber> },
    Combo { default: Option<UciString>, options: Vec<UciString> },
    Button,
    String { default: Option<UciString> },
}

/// Requested option change for engine configuration send from the GUI
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UciOption {
    pub name: String,
    pub value: UciOptionValue,
}

/// Value for the engine to display as configurable to the GUI upon startup
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UciOptionConfiguration {// TODO: Consider using KeyValuePair from a stdlib Map type so we can store them as a dictionary
    pub name: &'static str,
    pub config: UciOptionTypeConfiguration,
}

impl Display for UciOptionConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "option name {} type ", self.name)?;
        match &self.config {
            UciOptionTypeConfiguration::Button => write!(f, "button")?,
            UciOptionTypeConfiguration::Check { default } => {
                write!(f, "check")?;
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
            }
            UciOptionTypeConfiguration::Spin { default, min, max } => {
                write!(f, "spin")?;
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
                if let Some(min) = min {
                    write!(f, " min {min}")?;
                }
                if let Some(max) = max {
                    write!(f, " max {max}")?;
                }
            }
            UciOptionTypeConfiguration::Combo { default, options } => {
                write!(f, "combo")?;
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
                for option in options {
                    write!(f, " var {option}")?;
                }
            }
            UciOptionTypeConfiguration::String { default } => {
                write!(f, "string")?;
                if let Some(default) = default {
                    write!(f, " default {default}")?;
                }
            }
        }

        Ok(())
    }
}

pub struct UciSearchOptions {
    pub is_pondering: bool,
    pub multi_pv_count: Option<usize>,
    pub constraints: SearchConstraints,
    // TODO: other uci search parameters
}

pub enum UciGuiCommand {
    Uci,
    IsReady,
    Debug(UciBool),
    Position(Box<Board>),
    UciNewGame,
    SetOption(UciOption),
    Go(UciSearchOptions),
    Stop,
    PonderHit,
    Quit,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UciCommandParseError {
    UnrecognizedCommand,
}

impl Display for UciCommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            UciCommandParseError::UnrecognizedCommand => write!(f, "unrecognized command"),
        }
    }
}

impl TryFrom<String> for UciGuiCommand {
    type Error = UciCommandParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            // TODO: support real position, go, setoption
            "position startpos" => Ok(UciGuiCommand::Position(Box::new(Board::starting_position()))),
            "setoption name Threads value 5" => Ok(UciGuiCommand::SetOption(UciOption { name: String::from("Threads"), value: UciOptionValue::Spin(5) })),
            "go depth 5" => Ok(UciGuiCommand::Go(UciSearchOptions { is_pondering: false, multi_pv_count: None, constraints: SearchConstraints::new().with_depth(5) })),
            // Real
            "debug on" => Ok(UciGuiCommand::Debug(true)),
            "debug off" => Ok(UciGuiCommand::Debug(false)),
            "ucinewgame" => Ok(UciGuiCommand::UciNewGame),
            "uci" => Ok(UciGuiCommand::Uci),
            "isready" => Ok(UciGuiCommand::IsReady),
            "ponderhit" => Ok(UciGuiCommand::PonderHit),
            "stop" => Ok(UciGuiCommand::Stop),
            "quit" => Ok(UciGuiCommand::Quit),
            _ => Err(UciCommandParseError::UnrecognizedCommand),
        }
    }
}

pub struct UciReader {
    command_receiver: Receiver<Result<UciGuiCommand, UciCommandParseError>>,
    reader_handle: JoinHandle<Result<(), ()>>,
}

impl UciReader {
    pub fn create<R: BufRead + Send + 'static>(reader: R) -> Self {
        let (sender, command_receiver) = std::sync::mpsc::channel();
        let reader_handle = std::thread::spawn(move || reader.lines()
            .flatten()
            .map(UciGuiCommand::try_from)
            .try_for_each(|command_result| sender.send(command_result))
            .or(Err(())));

        Self {
            command_receiver,
            reader_handle,
        }
    }
}

impl IntoIterator for UciReader {
    type Item = Result<UciGuiCommand, UciCommandParseError>;
    type IntoIter = impl Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { command_receiver, reader_handle } = self;

        // Once the command receiver has closed (chain after iterator is exhausted) we can join the reader handle
        command_receiver.into_iter().chain(std::iter::once_with(move || {
            let _ = reader_handle.join();
            None
        }).flatten())
    }
}

pub struct UciInfo {
    // depth: Option<PlyCount>,
    // selective_depth: Option<PlyCount>,
    // time: Option<Duration>,
    // TODO: The rest of the info options (score, nodes, maybe nps (or compute when give time and nodes), multi-pv line, pv)
}

impl UciInfo {
    // TODO: Use builder pattern to create only valid UciInfo

    pub fn new_root_result() -> Self {
        // TODO: Return what would be output from the root iterative deepen function (pv)
        // TODO: Take PvMoveContainer
        // TODO: Take multipv index if multipv
        todo!()
    }

    pub fn new_status_update() -> Self {
        // TODO: Return what would be output periodically during a long search (nodes, nps, etc)
        // nodes, time, nps, currmove, currmovenumber
        todo!()
    }
}

impl Display for UciInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "info")
        // TODO: write fields if set
    }
}

pub struct UciWriter<W: Write> {
    writer: W,
}

impl<W: Write> UciWriter<W> {
    pub fn create(writer: W) -> Self {
        Self {
            writer,
        }
    }

    pub fn write_id(&mut self, name: &str, author: &str) -> std::io::Result<()> {
        writeln!(self.writer, "id name {name}")?;
        writeln!(self.writer, "id author {author}")?;
        writeln!(self.writer)
    }

    pub fn write_ready_ok(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "readyok")
    }

    pub fn write_uci_ok(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "uciok")
    }

    // TODO: Take some iter or container of UciOptionConfiguration
    pub fn write_options(&mut self, config: impl Iterator<Item=UciOptionConfiguration> + Sized) -> std::io::Result<()> {
        for option in config {
            writeln!(self.writer, "{option}")?;
        }

        Ok(())
    }

    pub fn write_info(&mut self, info: UciInfo) -> std::io::Result<()> {
        writeln!(self.writer, "{info}")
    }

    pub fn write_best_move(&mut self, best_move: ChessMove, refutation: Option<ChessMove>) -> std::io::Result<()> {
        if let Some(refutation) = refutation {
            writeln!(self.writer, "bestmove {best_move} ponder {refutation}")
        } else {
            writeln!(self.writer, "bestmove {best_move}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
