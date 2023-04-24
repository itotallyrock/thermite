#![warn(missing_docs, clippy::pedantic, rustdoc::missing_doc_code_examples, clippy::nursery, clippy::cargo, clippy::style)]

use derive_more::{AsMut, AsRef};
use half_move_clock::Game;

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
