// const features
#![feature(
    const_mut_refs,
    const_trait_impl,
    const_fmt_arguments_new,
    const_ops,
    const_convert,
    const_result_drop,
    const_option,
    const_try,
    const_fn_floating_point_arithmetic,
    const_for,
    const_intoiterator_identity,
    const_option_ext,
    const_slice_index,
    const_bool_to_option,
    const_num_from_num,
    const_refs_to_cell,
    const_eval_limit,
    generic_const_exprs,
)]
// other features#![feature(is_sorted, let_chains, rustdoc_missing_doc_code_examples)]
#![cfg_attr(test, feature(test))]
#![const_eval_limit = "0"]

#![warn(missing_docs, clippy::pedantic, rustdoc::missing_doc_code_examples, clippy::nursery, clippy::cargo, clippy::style)]
#![deny(clippy::all)]

#![allow(clippy::module_name_repetitions, clippy::debug_assert_with_mut_call, clippy::assertions_on_constants)]

//! Thermite chess essential types.
//! - [`Piece`](piece::Piece) - A piece on the board: King, Queen, Rook, Bishop, Knight, Pawn
//! - [`Player`](player::Player) - A player in the game, represented by the color of their pieces
//! - [`Square`](square::Square) - A single tile on a board, used for move notation
//! - [`Bitboard`](bitboard::Bitboard) - A mask of the board where each square can be either a `1` or a `0`
//! - [`ChessMove`](chess_move::ChessMove) - A from & to square for a piece and metadata necessary for making the move (eg. promotions)
//! - [`Castles`](castles::Castles) - The availability (or rights) for a side to castle [`CastleRights`](castles::CastleRights), keeps track of rook or king movement, and should also help move generation keep track of attacked squares
//! - [`Board`](board::Board) - The position: piece-arrangement, or where each piece is placed on the board, and side to move along with a myriad of featured gated metadata, for move-generation, evaluation, searching, and more.
//! - [`PawnApproximationEvaluation`](score::PawnApproximationEvaluation) - With `#[cfg(feature = "score")]` an evaluation of a [`Board`](board::Board)

/// A single player's chess move
pub type PlyCount = u16;
/// How many half moves a player can make before a pawn must be pushed or a piece be captured in order to avoid a draw
pub const STANDARD_MOVE_CLOCK: PlyCount = 50;

/// A counter for the sum of the number of a piece on a board
pub type PieceCount = u8;

/// Board mask based on a unsigned 64-bit integer, with each bit representing a single square on an 8x8 chess board.
pub mod bitboard;
/// The castle abilities for a game of a chess, containing the rights information for both sides.
pub mod castles;
/// Chess moves that can be made on a chess board
pub mod chess_move;
/// The types of chess moves
pub mod move_type;
/// Piece types for distinguishing what type of a piece is on a square.
pub mod piece_type;
/// A player in the game, or one side of the board, represented by their piece's color.
pub mod player;
/// Piece types for distinguishing what type of a piece a pawn should promote to.
pub mod promotion_piece_type;
/// A single tile on an 8x8 chess board.
pub mod square;
/// Zobrist hashing for matching board transpositions, or positions with the same piece arrangement and game state (side to move, en-passant, castle rights).
#[cfg(feature = "zobrist")]
pub mod zobrist;
