use enum_iterator::Sequence;
use enum_map::Enum;

/// The color of the pieces for the side (or player) moving.
#[derive(Enum, Sequence, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum PlayerColor {
    /// The player controlling the white pieces.
    /// Typically the first side to move in a standard game.
    White,
    /// The player controlling the black pieces.
    /// Typically the second side to move in a standard game.
    Black,
}

impl PlayerColor {
    /// Switch the side to the next player to move.
    ///
    /// ```rust
    /// use thermite_core::player_color::PlayerColor;
    ///
    /// assert_eq!(PlayerColor::White.switch(), PlayerColor::Black);
    /// assert_eq!(PlayerColor::Black.switch(), PlayerColor::White);
    /// ```
    #[must_use]
    pub const fn switch(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::player_color::PlayerColor;
    use test_case::test_case;

    #[test_case(PlayerColor::White, PlayerColor::Black)]
    #[test_case(PlayerColor::Black, PlayerColor::White)]
    fn switch_works(input: PlayerColor, expected: PlayerColor) {
        assert_eq!(input.switch(), expected);
    }
}
