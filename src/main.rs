use std::io::{BufRead, BufReader, stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender, SendError};
use std::thread::JoinHandle;

pub enum UciCommand {
    IsReady,
}

pub enum UciResponse {
    ReadyOk,
}

pub enum UciReaderError {
    IoError(std::io::ErrorKind),
    SendError(SendError<UciCommand>),
    ThreadPanicked,
}

pub struct UciDriver<R: BufRead, W: Write> {
    receiver: Receiver<UciCommand>,
    uci_writer: UciWriter<W>,
    uci_reader: UciReader<R>,
}

pub struct UciWriter<W: Write> {
    writer: W,
}

pub type NodeCount = u64;

pub enum UciInfo {
    NodesPerSecond(NodeCount)
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
            UciResponse::ReadyOk => write!(self.writer, "readyok"),
        }
    }
}

struct UciReader<R: BufRead> {
    reader_thread_handle: JoinHandle<Result<R, UciReaderError>>,
}

impl<R: BufRead + Send + 'static> UciReader<R> {
    pub fn start(mut reader: R, command_sender: Sender<UciCommand>) -> Self {
        let reader_thread_handle = std::thread::spawn(move || {
            let mut line = String::with_capacity(256);
            loop {
                reader.read_line(&mut line).map_err(|err| UciReaderError::IoError(err.kind()))?;
                // TODO: parse line better
                let command = match line.as_str() {
                    "isready" => UciCommand::IsReady,
                    "quit" => break,
                    _ => panic!("unknown command"),
                };

                command_sender.send(command).map_err(|err| UciReaderError::SendError(err))?;
            }

            Ok(reader)
        });

        Self {
            reader_thread_handle,
        }
    }
}

impl<R: BufRead + Send + 'static, W: Write> UciDriver<R, W> {
    pub fn start(reader: R, writer: W) -> Self {
        let (sender, receiver) = channel();
        let uci_writer = UciWriter { writer };
        let uci_reader = UciReader::start(reader, sender);

        Self {
            uci_reader,
            uci_writer,
            receiver,
        }
    }

    pub fn handle_input<F: FnMut(UciCommand) -> UciResponse>(mut self, mut handler: F) -> std::io::Result<()> {
        for command in self.receiver.iter() {
            let response = handler(command);
            self.uci_writer.respond(response)?;
        }

        Ok(())
    }

}

pub struct Engine {

}

impl Engine {
    pub fn new() -> Self {
        todo!()
    }
    pub fn is_ready(&self) -> UciResponse {
        // TODO: Block while engine initializes
        UciResponse::ReadyOk
    }
}

pub trait UciEngine {
    fn is_ready(&self) -> UciResponse;
    fn stop
}

struct Driver;

impl Driver {
    /// Block the current thread receiving UCI commands and triggering engine evaluation
    pub fn start() {
        let uci_reader = UciDriver::start(BufReader::new(stdin()), stdout());
        let uci_engine = Engine::new();

        uci_reader.handle_input(|command| match command {
            UciCommand::IsReady => uci_engine.is_ready(),
        }).expect("uci reader thread was unable to read from input");
    }
}

fn main() {
    Driver::start();
}




/*
    uci -> uciok
    debug {on|off} -> ()
    isready -> readyok
    setoption name {name} [value {value}] -> ()
    ucinewgame -> ()
    position -> ()
    go [depth {N}] [nodes {N}] [infinite] [ponder] [searchmoves [...moves]] [wtime [N]] [btime [N]] [winc [N]] [binc [N]] [movestogo [N > 0]] [mate [N]] -> bestmove {move} [ponder {move}]
    stop -> ()
    ponderhit -> ()
 */

mod uci_commands {
    pub mod engine_input {
        struct Uci;
        struct IsReady;
    }

    pub mod engine_output {
        struct UciOk;
        struct ReadyOk;
    }

    struct CommandResponse<Input, Output>(dyn FnMut(Input) -> Output);

}