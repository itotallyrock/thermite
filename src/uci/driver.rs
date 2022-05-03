use std::io::{BufRead, Write};
use std::sync::mpsc::{channel, Receiver};

use crate::uci::UciCommand;
use crate::uci::uci_reader::UciReader;
use crate::uci::uci_writer::UciWriter;

pub struct UciDriver<R: BufRead, W: Write> {
    pub receiver: Receiver<UciCommand>,
    pub uci_writer: UciWriter<W>,
    uci_reader: UciReader<R>,
}

impl<R: BufRead + Send + 'static, W: Write> UciDriver<R, W> {
    pub fn start(reader: R, writer: W) -> Self {
        let (sender, receiver) = channel();
        let uci_writer = UciWriter::new(writer);
        let uci_reader = UciReader::start(reader, sender);

        Self {
            uci_reader,
            uci_writer,
            receiver,
        }
    }

    pub fn shutdown(self) -> anyhow::Result<()> {
        self.uci_reader.shutdown()?;

        Ok(())
    }
}
