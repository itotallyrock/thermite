use std::ops::Not;

/// The color of the pieces used to indicate a player
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    /// The player that moves first, playing as the white pieces
    White,
    /// The player that moves second, playing as the black pieces
    Black,
}

impl Side {
    /// How many sides are there
    pub const COUNT: usize = 2;
    /// Array of sides for iteration
    pub const SIDES: [Side; Self::COUNT] = [Side::White, Side::Black];
}

impl Not for Side {
    type Output = Self;

    /// Get the opposite side
    fn not(self) -> Self::Output {
        match self {
            Side::White => Self::Black,
            Side::Black => Self::White,
        }
    }
}
