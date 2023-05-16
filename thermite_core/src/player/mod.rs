use std::fmt::{Display, Formatter, Write};

pub use by_player::ByPlayer;

mod by_player;

/// The number of players in a game
pub const NUM_PLAYERS: usize = 2;

/// The color of the pieces for the side (or player) moving.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum Player {
    /// The player controlling the white pieces.
    /// Typically the first side to move in a standard game.
    White,
    /// The player controlling the black pieces.
    /// Typically the second side to move in a standard game.
    Black,
}

impl Player {
    /// Array of all sides
    pub const PLAYERS: [Self; NUM_PLAYERS] = [Self::White, Self::Black];

    /// Switch the side to the next player to move.
    ///
    /// ```rust
    /// use thermite_core::player::Player;
    ///
    /// assert_eq!(Player::White.switch(), Player::Black);
    /// assert_eq!(Player::Black.switch(), Player::White);
    /// ```
    #[must_use]
    pub const fn switch(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => f.write_char('w'),
            Self::Black => f.write_char('b'),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use test::{Bencher, black_box};

    use test_case::test_case;

    use super::*;

    #[test_case(Player::White, Player::Black)]
    #[test_case(Player::Black, Player::White)]
    fn switch_works(side: Player, expected: Player) {
        assert_eq!(side.switch(), expected);
    }

    #[test_case(Player::White)]
    #[test_case(Player::Black)]
    fn switch_is_symmetric(input: Player) {
        assert_eq!(input.switch().switch(), input);
    }

    #[bench]
    fn switch_bench(bencher: &mut Bencher) {
        let input = black_box(Player::Black);
        bencher.iter(|| assert_eq!(black_box(Player::switch(input)), black_box(Player::White)));
    }
}
