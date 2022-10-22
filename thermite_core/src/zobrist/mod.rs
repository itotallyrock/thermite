use crate::piece_type::PieceType;
use crate::side::Side;
use crate::square::Square;
use crate::zobrist::keys::{castle_lookup, en_passant_lookup, piece_square_lookup, SIDE_KEY};
use keys::EMPTY_ZOBRIST_KEY;
use std::fmt::{Debug, Formatter};
use std::hash::Hasher;

mod keys;

/// The raw hash value for a chess position
type ZobristInner = u64;

/// Positional hasher based on board features.
/// The same position arrived at through different sets of moves will still have the same Zobrist hash.
/// This is because the only features relevant to this hash are stateless, or can be interpreted as
/// the board's state:
/// - Side to move
/// - Pieces placed on squares
/// - En-passant square if any
/// - Castle rights for both sides
#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ZobristHasher(ZobristInner);

impl ZobristHasher {
    const fn toggle(&mut self, mask: ZobristInner) {
        self.0 ^= mask;
    }

    /// Toggle the placement of a piece on a given square for a given side.
    /// Adds the piece placement to the hash; otherwise, removes the piece if it is already included.
    pub const fn toggle_piece_square(&mut self, square: Square, piece: PieceType, side: Side) {
        self.toggle(piece_square_lookup(square, piece, side));
    }

    /// Toggle the side to move between white and black.
    pub const fn switch_sides(&mut self) {
        self.toggle(SIDE_KEY);
    }

    /// Toggle the side to move between white and black.
    pub const fn toggle_en_passant_square(&mut self, square: Square) {
        self.toggle(en_passant_lookup(square));
    }

    /// Toggle the castle rights for a side to castle in one direction
    pub const fn toggle_castle_ability(&mut self, side: Side, king_side: bool) {
        self.toggle(castle_lookup(side, king_side));
    }
}

impl Hasher for ZobristHasher {
    /// Get the pre-computed hash value
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        panic!("All keys currently must be u64, use write_u64 instead.");
    }

    fn write_u64(&mut self, i: u64) {
        self.0 ^= i;
    }
}

impl ZobristHasher {
    /// The default base key for an empty featureless position
    #[must_use]
    pub const fn empty() -> Self {
        Self(EMPTY_ZOBRIST_KEY)
    }
}

impl Debug for ZobristHasher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(ZobristHasher)).field(&format!("{:#X}", self.0)).finish()
    }
}
