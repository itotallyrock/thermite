use std::num::{NonZeroU64, NonZeroU8};
use crate::game::SimpleChessMove;

/// Number of principle variations, typically for outputting multiple lines
pub type PvCount = NonZeroU8;
/// Number of positions (nodes) visited in a search tree (non-zero)
pub type NodeCount = NonZeroU64;
/// Number of sequential moves (plies)
pub type PlyCount = u8;
/// Number of moves to search deep (non-zero plies)
pub type SearchDepth = NonZeroU8;
/// Simple move list for moves provided from the GUI
pub type SimpleMoveList = Vec<SimpleChessMove>;

/// Approximate board evaluation in 1 / 100th pawns
pub type CentiPawns = i16;

/// Positional evaluation
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Score {
    Centipawns(CentiPawns),
    Mate(SearchDepth),
}