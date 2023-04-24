#![warn(missing_docs, clippy::pedantic, rustdoc::missing_doc_code_examples, clippy::nursery, clippy::cargo, clippy::style)]

use derive_more::{AsMut, AsRef};
use legal_position::LegalPosition;
use crate::ply_count::PlyCount;

mod player_color;
mod square;
mod pieces;
mod castles;
mod half_move_clock;
mod raw_position;
mod board_mask;
mod zobrist;
mod ply_count;
mod legal_position;

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;


#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Game {
    #[as_ref()]
    #[as_mut()]
    legal_position: LegalPosition,
    halfmove_count: PlyCount,
}

#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Searchable {
    #[as_ref()]
    #[as_mut()]
    game: Game,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
