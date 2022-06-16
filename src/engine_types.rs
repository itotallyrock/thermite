use std::fmt::{Display, Formatter};
use std::num::{NonZeroU64, NonZeroU8};

use crate::game::SimpleChessMove;

/// Number of hits in the endgame table database
pub type TableBaseHits = u64;
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

/// Engine's evaluation/rating for a given position.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Score {
    /// Approximate valuation of the position as an integer of 1/100ths of a pawn
    Centipawns(CentiPawns),
    /// The number of moves until a mate
    Mate(SearchDepth),
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Centipawns(centipawns) => write!(f, "cp {}", centipawns),
            Score::Mate(mate_plies) => write!(f, "mate {}", mate_plies),
        }
    }
}
