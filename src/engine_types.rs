use std::num::{NonZeroU64, NonZeroU8};
use crate::game::SimpleChessMove;

/// Number of positions (nodes) visited in a search tree (non-zero)
pub type NodeCount = NonZeroU64;
/// Number of sequential moves (plies)
pub type PlyCount = u8;
/// Number of moves to search deep (non-zero plies)
pub type SearchDepth = NonZeroU8;
/// Simple move list for moves provided from the GUI
pub type SimpleMoveList = Vec<SimpleChessMove>;
