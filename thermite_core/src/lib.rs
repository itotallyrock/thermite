//! Thermite - Another rust chess engine
#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::style
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

/// A on-off (0/1) mask of an 8x8 chess board (bitboard)
pub mod bitboard;
/// The castle abilities for a game of a chess, containing the rights information for both sides.
pub mod castles;
/// A generated or parsed legal-move that can be performed on a specific [`LegalPosition`](position::LegalPosition)
pub mod chess_move;
/// Generalized directional movement on the board
pub mod direction;
/// The checkmate/checkmated plies or the approximate material/positional advantage for a given side
pub mod evaluation;
/// A clock for keeping track of half moves without a capture or pawn push before a draw
pub mod half_move_clock;
/// A counter for keeping track of visited chess positions
pub mod node_count;
/// A counter for the sum of the number of a piece on a board
pub mod piece_count;
/// A piece that can be placed on the board
pub mod pieces;
/// A player in the game, or one side of the board, represented by their piece's color.
pub mod player_color;
/// The depth or number of single moves deep into a game
pub mod ply_count;
/// The total representation of a single legal state of a game of chess and its internal logic
pub mod position;
/// A single tile on a board where a piece can be placed
pub mod square;
/// Board transposition hashing (if a position is identical in terms of play but could be arrived at via different moves)
pub mod zobrist;
