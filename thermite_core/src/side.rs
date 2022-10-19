
/// The number of players in a game
pub const NUM_SIDES: usize = 2;

/// The color of the pieces for the side (or player) moving.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Side {
    /// The player controlling the white pieces.
    /// Typically the first side to move in a standard game.
    White,
    /// The player controlling the black pieces.
    /// Typically the second side to move in a standard game.
    Black,
}

impl Side {
    /// Array of all sides
    pub const SIDES: [Self; NUM_SIDES] = [Self::White, Self::Black];

    /// Switch the side to the next player to move.
    ///
    /// ```rust
    /// use thermite_core::side::Side;
    ///
    /// assert_eq!(Side::White.switch(), Side::Black);
    /// assert_eq!(Side::Black.switch(), Side::White);
    /// ```
    #[must_use]
    pub const fn switch(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl const PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        (*self) as u8 == (*other) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Side::White, Side::Black)]
    #[test_case(Side::Black, Side::White)]
    fn switch_works(side: Side, expected: Side) {
        assert_eq!(side.switch(), expected);
    }

    #[test_case(Side::White)]
    #[test_case(Side::Black)]
    fn switch_is_symmetric(input: Side) {
        assert_eq!(input.switch().switch(), input);
    }
}