use crate::piece_type::{PieceType, NUM_PIECE_TYPES};

/// A container data-structure that holds an instance of `T` for each piece type (one for pawn, rook, bishop, knight, queen, king)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ByPieceType<T> {
    items: [T; NUM_PIECE_TYPES],
}

impl<T> ByPieceType<T> {
    /// Crate a new `ByPieceType` with pre-set values for all piece types
    pub const fn new_with(pawn: T, knight: T, bishop: T, rook: T, queen: T, king: T) -> Self {
        Self { items: [pawn, knight, bishop, rook, queen, king] }
    }
    /// Get the inner `T` for a given piece type
    pub const fn get_piece(&self, piece: PieceType) -> &T {
        &self.items[piece as usize]
    }
    /// Get a mutable references to the inner `T` for a given piece type
    pub const fn mut_piece(&mut self, piece: PieceType) -> &mut T {
        &mut self.items[piece as usize]
    }
}

impl<T: Copy> ByPieceType<T> {
    /// Create a `ByPieceType` where all pieces have the same value
    pub const fn new(item: T) -> Self {
        Self { items: [item; NUM_PIECE_TYPES] }
    }
}

impl<T> const From<[T; NUM_PIECE_TYPES]> for ByPieceType<T> {
    fn from(value: [T; NUM_PIECE_TYPES]) -> Self {
        Self { items: value }
    }
}

#[cfg(test)]
impl<T> ByPieceType<T> {
    /// Get the underlying container
    pub fn into_inner(self) -> [T; NUM_PIECE_TYPES] {
        self.items
    }
}