use std::io::Write;
use crate::engine_types::Score;

use crate::uci::{ScoreBoundsType, SearchResult, UciInfo, UciOptionType, UciResponse};

pub struct UciWriter<W: Write> {
    writer: W,
}


impl<W: Write> UciWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer
        }
    }
    pub fn debug(&mut self, infos: &[UciInfo]) -> std::io::Result<()> {
        write!(self.writer, "info")?;
        for info in infos {
            match info {
                UciInfo::String(string) => {
                    write!(self.writer, " string {}", string)?;
                    break;
                },
                UciInfo::NodesPerSecond(nodes_per_second) => write!(self.writer, " nodes {}", nodes_per_second)?,
                UciInfo::CurrentMove(current_move) => write!(self.writer, " currmove {}", current_move)?,
                UciInfo::CurrentMoveNumber(current_move_number ) => write!(self.writer, " currmovenumber {}", current_move_number)?,
                UciInfo::SearchDepth(search_depth) => write!(self.writer, " depth {}", search_depth)?,
                UciInfo::SelectiveSearchDepth(selective_search_depth) => write!(self.writer, " seldepth {}", selective_search_depth)?,
                UciInfo::TimeSearched(time_searched) => write!(self.writer, " time {}", time_searched.as_millis())?,
                UciInfo::PrincipleVariation(principle_variation) => {
                    debug_assert!(principle_variation.len() > 0, "sent empty pv list");
                    write!(self.writer, " pv")?;
                    for pv_move in principle_variation {
                        write!(self.writer, " {}", pv_move)?;
                    }
                },
                UciInfo::Refutation(refutation_line) => {
                    debug_assert!(refutation_line.len() > 1, "sent empty refutation list");
                    write!(self.writer, " refutation")?;
                    for refuting_move in refutation_line {
                        write!(self.writer, " {}", refuting_move)?;
                    }
                }
                UciInfo::MultiPvIndex(pv_index) => write!(self.writer, " multipv {}", pv_index)?,
                UciInfo::Evaluation(uci_score) => {
                    match uci_score.score {
                        Score::Centipawns(centipawns) => write!(self.writer, " score cp {}", centipawns)?,
                        Score::Mate(mate_plies) => write!(self.writer, " score mate {}", mate_plies)?,
                    }
                    match uci_score.bounds_type {
                        ScoreBoundsType::Exact => {},
                        ScoreBoundsType::Upper => write!(self.writer, " upperbound")?,
                        ScoreBoundsType::Lower => write!(self.writer, " lowerbound")?,
                    }
                }
                UciInfo::HashTableUsage(usage_percent) => write!(self.writer, " hashfull {}", usage_percent / 10.)?,
                UciInfo::CpuUsage(usage_percent) => write!(self.writer, " cpuload {}", usage_percent / 10.)?,
                UciInfo::EndgameTableBaseHits(hits) => write!(self.writer, " tbhits {}", hits)?,
            }
        }
        self.writer.flush()
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
