use thiserror::Error;

use std::io::BufRead;
use std::sync::mpsc::{Sender, SendError};
use std::thread::JoinHandle;

use crate::uci::UciCommand;

#[derive(Error, Debug)]
pub enum UciReaderError {
    #[error("IO Error")]
    IoError(std::io::ErrorKind),
    #[error("Send Error")]
    SendError(#[source] SendError<UciCommand>),
    #[error("Reader Thread Panicked")]
    ThreadPanicked,
}

pub struct UciReader<R: BufRead> {
    reader_thread_handle: JoinHandle<Result<R, UciReaderError>>,
}

impl<R: BufRead + Send + 'static> UciReader<R> {
    pub fn start(mut reader: R, command_sender: Sender<UciCommand>) -> Self {
        let reader_thread_handle = std::thread::spawn(move || {
            let mut line = String::with_capacity(256);
            loop {
                reader.read_line(&mut line).map_err(|err| UciReaderError::IoError(err.kind()))?;
                // TODO: parse line better
                let command = match line.as_str().trim() {
                    "uci" => UciCommand::Uci,
                    "isready" => UciCommand::IsReady,
                    "quit" => break,
                    _ => UciCommand::Other(line.clone()),
                };

                // Send the command to the receiver
                command_sender.send(command)
                    .map_err(|err| UciReaderError::SendError(err))?;

                // Clear the line buffer for the next command
                line.clear();
            }

            Ok(reader)
        });

        Self {
            reader_thread_handle,
        }
    }

    pub fn shutdown(self) -> Result<(), UciReaderError> {
        self.reader_thread_handle.join()
            .map_err(|_| UciReaderError::ThreadPanicked)
            .and_then(|reader_error| reader_error)
            .map(|_| ())
    }
}

