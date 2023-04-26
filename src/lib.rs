//! Thermite - Another rust chess engine

#![warn(
    missing_docs,
    clippy::pedantic,
    rustdoc::missing_doc_code_examples,
    clippy::nursery,
    clippy::cargo,
    clippy::style
)]

/// A on-off (0/1) mask of an 8x8 chess board (bitboard)
pub mod board_mask;
/// The castle abilities for a game of a chess, containing the rights information for both sides.
pub mod castles;
mod game;
mod half_move_clock;
mod legal_position;
mod pieces;
/// A player in the game, or one side of the board, represented by their piece's color.
pub mod player_color;
mod ply_count;
mod raw_position;
mod searchable;
/// A single tile on a board where a piece can be placed
pub mod square;
mod zobrist;
