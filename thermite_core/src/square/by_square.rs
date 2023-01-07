use crate::square::{NUM_SQUARES, Square};

/// A container data-structure that holds an instance of `T` for each square
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BySquare<T> {
    items: [T; NUM_SQUARES],
}

impl<T> BySquare<T> {
    /// Get the inner `T` for a given square
    pub const fn get_square(&self, square: Square) -> &T {
        &self.items[square as usize]
    }
    /// Get a mutable references to the inner `T` for a given square
    pub const fn mut_square(&mut self, square: Square) -> &mut T {
        &mut self.items[square as usize]
    }
}

impl<T: Copy> BySquare<T> {
    /// Create a `BySquare` where all squares have the same value
    pub const fn new(item: T) -> Self {
        Self { items: [item; NUM_SQUARES] }
    }
}

impl<T> const From<[T; NUM_SQUARES]> for BySquare<T> {
    fn from(value: [T; NUM_SQUARES]) -> Self {
        Self { items: value }
    }
}

impl<T: ~const Default + Copy> const Default for BySquare<T> {
    fn default() -> Self {
        Self {
            items: [T::default(); NUM_SQUARES],
        }
    }
}
