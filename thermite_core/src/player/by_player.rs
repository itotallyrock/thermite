use crate::player::{Player, NUM_PLAYERS};

/// A container data-structure that holds an instance of `T` for each player (one for white, one for black)
#[derive(Copy, Clone, Eq, Debug)]
pub struct ByPlayer<T> {
    items: [T; NUM_PLAYERS],
}

impl<T> ByPlayer<T> {
    /// Crate a new `ByPlayer` with pre-set values for both white and black
    pub const fn new_with(white: T, black: T) -> Self {
        Self { items: [white, black] }
    }
    /// Get the inner `T` for a given player
    pub const fn get_side(&self, side: Player) -> &T {
        &self.items[side as usize]
    }
    /// Get a mutable references to the inner `T` for a given player
    pub const fn mut_side(&mut self, side: Player) -> &mut T {
        &mut self.items[side as usize]
    }
}

impl<T: Copy> ByPlayer<T> {
    /// Create a `ByPlayer` where both players have the same value
    pub const fn new(item: T) -> Self {
        Self { items: [item; NUM_PLAYERS] }
    }
}

impl<T: PartialEq> PartialEq for ByPlayer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.items[0] == other.items[0] && self.items[1] == other.items[1]
    }
}

impl<T: Default + Copy> Default for ByPlayer<T> {
    fn default() -> Self {
        Self {
            items: [T::default(); NUM_PLAYERS],
        }
    }
}

impl<T> From<[T; NUM_PLAYERS]> for ByPlayer<T> {
    fn from(value: [T; NUM_PLAYERS]) -> Self {
        Self { items: value }
    }
}

