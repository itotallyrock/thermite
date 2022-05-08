use std::time::Duration;
use crate::engine_types::{NodeCount, PlyCount, PvCount, SearchDepth, SimpleMoveList};
use crate::game::SimpleChessMove;
use crate::uci::UciScore;

#[derive(Clone, Debug)]
pub enum UciInfo {
    /// Raw message
    String(String),
    /// How many nodes per second the engine is covering
    NodesPerSecond(NodeCount),
    /// The root move currently being explored
    CurrentMove(SimpleChessMove),
    /// The current root move index, starting at 1
    CurrentMoveNumber(SearchDepth),
    /// How many moves ahead the search has looked
    SearchDepth(PlyCount),
    /// If searching selectively deeper, to what depth
    SelectiveSearchDepth(PlyCount),
    /// How much time has been spent searching
    TimeSearched(Duration),
    /// The current best line
    PrincipleVariation(SimpleMoveList),
    /// List of moves that refutes the first move in the refutation move list
    Refutation(SimpleMoveList),
    /// Multi-PV Nth best variation index
    MultiPvIndex(PvCount),
    /// Evaluation for the current move
    Evaluation(UciScore),
    /// Hash table usage
    HashTableUsage(f64),
    /// The current CPU's average usage across all cores
    CpuUsage(f64),
    /// End-game table-base hits
    EndgameTableBaseHits(NodeCount),
}