#![warn(
    missing_docs,
    clippy::pedantic,
    rustdoc::missing_doc_code_examples,
    clippy::nursery,
    clippy::cargo,
    clippy::style
)]

mod board_mask;
mod castles;
mod game;
mod half_move_clock;
mod legal_position;
mod pieces;
mod player_color;
mod ply_count;
mod raw_position;
mod searchable;
mod square;
mod zobrist;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
