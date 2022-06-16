use std::fmt::{Display, Formatter};
use std::num::{NonZeroU64, NonZeroU8};

use crate::game::SimpleChessMove;

/// Number of principle variations, typically for outputting multiple lines
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PvCount {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

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

/// Positional evaluation
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Score {
    Centipawns(CentiPawns),
    Mate(SearchDepth),
}

#[derive(Copy, Clone, Debug)]
pub enum InvalidPvCount {
    TooSmall,
    TooBig,
}

impl TryFrom<u8> for PvCount {
    type Error = InvalidPvCount;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Err(InvalidPvCount::TooSmall),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            _ => Err(InvalidPvCount::TooBig),
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Centipawns(centipawns) => write!(f, "cp {}", centipawns),
            Score::Mate(mate_plies) => write!(f, "mate {}", mate_plies),
        }
    }
}
