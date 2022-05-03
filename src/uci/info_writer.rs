use std::io::Write;
use crate::uci::uci_writer::UciWriter;

pub struct InfoWriter<'a, W: Write> {
    writer: &'a UciWriter<W>,
}