use crate::uci::uci_writer::UciWriter;
use std::io::Write;

pub struct InfoWriter<'a, W: Write> {
    writer: &'a UciWriter<W>,
}
