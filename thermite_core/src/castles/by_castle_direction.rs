use crate::castles::direction::{CastleDirection, NUM_CASTLE_DIRECTIONS};

/// A container data-structure that holds an instance of `T` for each [castle direction](CastleDirection) (one for `CastleDirection::KingSide   `, `CastleDirection::QueenSide  `)
#[derive(Copy, Clone, Eq, Debug)]
pub struct ByCastleDirection<T> {
    items: [T; NUM_CASTLE_DIRECTIONS],
}

impl<T> ByCastleDirection<T> {
    /// Crate a new `ByCastleDirection` with pre-set values for all [castle direction types](CastleDirection)
    pub const fn new_with(king_side: T, queen_side: T) -> Self {
        Self { items: [king_side, queen_side] }
    }
    /// Get the inner `T` for a given [castle direction type](CastleDirection)
    pub const fn get_direction(&self, castle_direction: CastleDirection) -> &T {
        &self.items[castle_direction as usize]
    }
    /// Get a mutable references to the inner `T` for a given [castle direction type](CastleDirection)
    pub const fn mut_castle_direction(&mut self, castle_direction: CastleDirection) -> &mut T {
        &mut self.items[castle_direction as usize]
    }
}

impl<T: Copy> ByCastleDirection<T> {
    /// Create a `ByCastleDirection` where all [castle directions](CastleDirection) have the same value
    pub const fn new(item: T) -> Self {
        Self { items: [item; NUM_CASTLE_DIRECTIONS] }
    }
}

impl<T> const From<[T; NUM_CASTLE_DIRECTIONS]> for ByCastleDirection<T> {
    fn from(value: [T; NUM_CASTLE_DIRECTIONS]) -> Self {
        Self { items: value }
    }
}

impl<T: ~const Default + Copy> const Default for ByCastleDirection<T> {
    fn default() -> Self {
        Self {
            items: [T::default(); NUM_CASTLE_DIRECTIONS],
        }
    }
}

impl<T: ~const PartialEq> const PartialEq for ByCastleDirection<T> {
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(NUM_CASTLE_DIRECTIONS == 2);
        self.items[0] == other.items[0] && self.items[1] == other.items[1]
    }
}
