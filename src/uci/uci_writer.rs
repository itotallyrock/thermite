use std::io::Write;

use crate::uci::{UciInfo, UciResponse};

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
            UciInfo::NodesPerSecond(nodes_per_second) => write!(self.writer, "info nodes {nodes_per_second}"),
        }
    }
    pub fn respond(&mut self, response: UciResponse) -> std::io::Result<()> {
        match response {
            UciResponse::ReadyOk => write!(self.writer, "readyok")?,
        }
        self.writer.flush()
    }
}
