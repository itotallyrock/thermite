
#![cfg_attr(test, feature(test))]

#![warn(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::style)]

//! Thermite chess essential types.
//! - `Piece` - A piece on the board: King, Queen, Rook, Bishop, Knight, Pawn
//! - `Side` - A player in the game, represented by the color of their pieces
//! - `Square` - A single tile on a board, used for move notation
//! - `Bitboard` - A mask of the board where each square can be either a `1` or a `0`
//! - `ChessMove` - A from & to square for a piece and metadata necessary for making the move (eg. promotions)
//! - `CastleRights` - The availability for a side to castle, keeps track of rook or king movement, and should also help move generation keep track of attacked squares
//! - `Board` - The position: piece-arrangement, or where each piece is placed on the board, and side to move along with a myriad of featured gated metadata, for move-generation, evaluation, searching, and more.
//! - `Score` - With `#[cfg(feature = "score")]` an evaluation of a [`Board`](crate::board::Board)
