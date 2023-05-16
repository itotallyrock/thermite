use std::fmt::{Display, Formatter};
use crate::castles::{CastleDirection, NUM_CASTLES};
use crate::player::Player;
use std::ops::{BitAnd, BitOr, Not};
use std::str::FromStr;

/// Keeps track of available castle abilities (king-side or queen-side castle) for both sides.
#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
#[must_use]
pub enum CastleRights {
    /// No castle abilities for either side
    #[default]
    None = 0,
    /// White's king-side castle ability
    WhiteKing = 1 << 0,
    /// White's queen-side castle ability
    WhiteQueen = 1 << 1,
    /// White can castle on both king and queen side
    WhiteBoth = Self::WhiteKing as u8 | Self::WhiteQueen as u8,
    /// Black's king-side castle ability
    BlackKing = 1 << 2,
    /// Both sides' king-side castle abilities
    BothKings = Self::WhiteKing as u8 | Self::BlackKing as u8,
    /// White queen-side and black king-side castle abilities
    WhiteQueenBlackKing = Self::WhiteQueen as u8 | Self::BlackKing as u8,
    /// White queen & king-side as well as black's king-side castle abilities
    WhiteBothBlackKing = Self::WhiteBoth as u8 | Self::BlackKing as u8,
    /// Black's queen-side castle ability
    BlackQueen = 1 << 3,
    /// White's king-side and black's queen-side castle abilities
    WhiteKingBlackQueen = Self::WhiteKing as u8 | Self::BlackQueen as u8,
    /// Both sides' queen-side castle abilities
    BothQueens = Self::WhiteQueen as u8 | Self::BlackQueen as u8,
    /// White's king & queen-side as well as black's queen-side castle abilities
    WhiteBothBlackQueen = Self::WhiteBoth as u8 | Self::BlackQueen as u8,
    /// Black can castle on both king and queen side
    BlackBoth = Self::BlackKing as u8 | Self::BlackQueen as u8,
    /// White's king-side and both black's king & queen-side castle abilities
    WhiteKingBlackBoth = Self::WhiteKing as u8 | Self::BlackBoth as u8,
    /// White's queen-side and both black's king & queen-side castle abilities
    WhiteQueenBlackBoth = Self::WhiteQueen as u8 | Self::BlackBoth as u8,
    /// If both players can castle in all directions
    All = Self::WhiteBoth as u8 | Self::BlackBoth as u8,
}

impl CastleRights {
    /// Array of the four castle options starting with white, king then queen, then black king and queen.
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    ///
    /// assert_eq!(CastleRights::CASTLES[0], CastleRights::WhiteKing);
    /// assert_eq!(CastleRights::CASTLES[1], CastleRights::WhiteQueen);
    /// assert_eq!(CastleRights::CASTLES[2], CastleRights::BlackKing);
    /// assert_eq!(CastleRights::CASTLES[3], CastleRights::BlackQueen);
    /// ```
    pub const CASTLES: [Self; NUM_CASTLES] = [Self::WhiteKing, Self::WhiteQueen, Self::BlackKing, Self::BlackQueen];

    /// Array of all possible castle combinations in their bit-flag order
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    ///
    /// assert_eq!(CastleRights::ALL_CASTLES[0], CastleRights::None);
    /// assert_eq!(CastleRights::ALL_CASTLES[5], CastleRights::BothKings);
    /// assert_eq!(CastleRights::ALL_CASTLES[10], CastleRights::BothQueens);
    /// assert_eq!(CastleRights::ALL_CASTLES[15], CastleRights::All);
    /// ```
    pub const ALL_CASTLES: [Self; NUM_CASTLES * NUM_CASTLES] = [
        Self::None,
        Self::WhiteKing,
        Self::WhiteQueen,
        Self::WhiteBoth,
        Self::BlackKing,
        Self::BothKings,
        Self::WhiteQueenBlackKing,
        Self::WhiteBothBlackKing,
        Self::BlackQueen,
        Self::WhiteKingBlackQueen,
        Self::BothQueens,
        Self::WhiteBothBlackQueen,
        Self::BlackBoth,
        Self::WhiteKingBlackBoth,
        Self::WhiteQueenBlackBoth,
        Self::All,
    ];

    /// Get the rights for a specific side
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    /// use thermite_core::player::Player;
    ///
    /// assert_eq!(CastleRights::for_side(Player::White), CastleRights::WhiteBoth);
    /// assert_eq!(CastleRights::for_side(Player::Black), CastleRights::BlackBoth);
    /// ```
    pub const fn for_side(side: Player) -> Self {
        match side {
            Player::White => Self::WhiteBoth,
            Player::Black => Self::BlackBoth,
        }
    }

    /// If the castle rights specify the ability for a side to castle in a given direction.
    ///
    /// ```
    /// use thermite_core::castles::{CastleDirection, CastleRights};
    /// use thermite_core::player::Player;
    /// // Test if white can king-side castle
    /// assert_eq!(CastleRights::WhiteKing.can_castle(Player::White, CastleDirection::KingSide), true);
    /// // Test if black can queen-side castle
    /// assert_eq!(CastleRights::BlackQueen.can_castle(Player::Black, CastleDirection::QueenSide), true);
    /// ```
    ///
    /// Combination `CastleRights` such as `None` or `All` are supported as well.
    /// ```
    /// use thermite_core::castles::{CastleDirection, CastleRights};
    /// use thermite_core::player::Player;
    ///
    /// // CastleRights::None is always false
    /// assert_eq!(CastleRights::None.can_castle(Player::White, CastleDirection::KingSide), false);
    /// assert_eq!(CastleRights::None.can_castle(Player::White, CastleDirection::QueenSide), false);
    /// assert_eq!(CastleRights::None.can_castle(Player::Black, CastleDirection::KingSide), false);
    /// assert_eq!(CastleRights::None.can_castle(Player::Black, CastleDirection::QueenSide), false);
    ///
    /// // CastleRights::All is always true
    /// assert_eq!(CastleRights::All.can_castle(Player::White, CastleDirection::KingSide), true);
    /// assert_eq!(CastleRights::All.can_castle(Player::White, CastleDirection::QueenSide), true);
    /// assert_eq!(CastleRights::All.can_castle(Player::Black, CastleDirection::KingSide), true);
    /// assert_eq!(CastleRights::All.can_castle(Player::Black, CastleDirection::QueenSide), true);
    /// ```
    #[must_use]
    pub const fn can_castle(&self, side: Player, direction: CastleDirection) -> bool {
        let truthy_mask = match (side, direction) {
            (Player::White, CastleDirection::KingSide) => Self::WhiteKing as u8,
            (Player::White, CastleDirection::QueenSide) => Self::WhiteQueen as u8,
            (Player::Black, CastleDirection::KingSide) => Self::BlackKing as u8,
            (Player::Black, CastleDirection::QueenSide) => Self::BlackQueen as u8,
        };

        truthy_mask & *self as u8 != 0u8
    }

    /// Remove a mask of rights from a set of castle rights
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    /// assert_eq!(CastleRights::All.filter(CastleRights::All), CastleRights::None);
    /// assert_eq!(CastleRights::All.filter(CastleRights::None), CastleRights::All);
    /// assert_eq!(CastleRights::None.filter(CastleRights::None), CastleRights::None);
    /// assert_eq!(CastleRights::None.filter(CastleRights::All), CastleRights::None);
    /// assert_eq!(CastleRights::BothKings.filter(CastleRights::WhiteKing), CastleRights::BlackKing);
    /// assert_eq!(CastleRights::WhiteBoth.filter(CastleRights::WhiteKing), CastleRights::WhiteQueen);
    /// ```
    pub fn filter(&self, other: Self) -> Self {
        *self & !other
    }

}

impl FromStr for CastleRights {
    type Err = IllegalCastleRights;

    /// Attempt to parse a UCI string into a [`CastleRight`](CastleRights).
    ///
    /// ```
    /// use std::str::FromStr;
    /// use thermite_core::castles::{CastleRights, IllegalCastleRights};
    ///
    /// assert_eq!(CastleRights::from_str("KQkq"), Ok(CastleRights::All));
    /// assert_eq!(CastleRights::from_str("KQ"), Ok(CastleRights::WhiteBoth));
    /// assert_eq!(CastleRights::from_str("q"), Ok(CastleRights::BlackQueen));
    /// assert_eq!(CastleRights::from_str("32"), Err(IllegalCastleRights));
    /// ```
    ///
    /// # Errors
    /// Will error if input is not a valid UCI castle right.
    /// Must be a combination of `'K'`, `'Q'`, `'k'`, and `'q'` or `'-'`.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input.as_bytes() {
            b"-" => Self::None,
            b"K" => Self::WhiteKing,
            b"Q" => Self::WhiteQueen,
            b"KQ" => Self::WhiteBoth,
            b"k" => Self::BlackKing,
            b"Kk" => Self::BothKings,
            b"Qk" => Self::WhiteQueenBlackKing,
            b"KQk" => Self::WhiteBothBlackKing,
            b"q" => Self::BlackQueen,
            b"Kq" => Self::WhiteKingBlackQueen,
            b"Qq" => Self::BothQueens,
            b"KQq" => Self::WhiteBothBlackQueen,
            b"kq" => Self::BlackBoth,
            b"Kkq" => Self::WhiteKingBlackBoth,
            b"Qkq" => Self::WhiteQueenBlackBoth,
            b"KQkq" => Self::All,
            _ => return Err(IllegalCastleRights),
        })
    }
}

impl BitAnd for CastleRights {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::try_from(self as u8 & rhs as u8).ok().unwrap()
    }
}

impl BitOr for CastleRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::try_from(self as u8 | rhs as u8).ok().unwrap()
    }
}

impl Not for CastleRights {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::None => Self::All,
            Self::WhiteKing => Self::WhiteQueenBlackBoth,
            Self::WhiteQueen => Self::WhiteKingBlackBoth,
            Self::WhiteBoth => Self::BlackBoth,
            Self::BlackKing => Self::WhiteBothBlackQueen,
            Self::BothKings => Self::BothQueens,
            Self::WhiteQueenBlackKing => Self::WhiteKingBlackQueen,
            Self::WhiteBothBlackKing => Self::BlackQueen,
            Self::BlackQueen => Self::WhiteBothBlackKing,
            Self::WhiteKingBlackQueen => Self::WhiteQueenBlackKing,
            Self::BothQueens => Self::BothKings,
            Self::WhiteBothBlackQueen => Self::BlackKing,
            Self::BlackBoth => Self::WhiteBoth,
            Self::WhiteKingBlackBoth => Self::WhiteQueen,
            Self::WhiteQueenBlackBoth => Self::WhiteKing,
            Self::All => Self::None,
        }
    }
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-"),
            Self::WhiteKing => write!(f, "K"),
            Self::WhiteQueen => write!(f, "Q"),
            Self::WhiteBoth => write!(f, "KQ"),
            Self::BlackKing => write!(f, "k"),
            Self::BothKings => write!(f, "Kk"),
            Self::WhiteQueenBlackKing => write!(f, "Qk"),
            Self::WhiteBothBlackKing => write!(f, "KQk"),
            Self::BlackQueen => write!(f, "q"),
            Self::WhiteKingBlackQueen => write!(f, "Kq"),
            Self::BothQueens => write!(f, "Qq"),
            Self::WhiteBothBlackQueen => write!(f, "KQq"),
            Self::BlackBoth => write!(f, "kq"),
            Self::WhiteKingBlackBoth => write!(f, "Kkq"),
            Self::WhiteQueenBlackBoth => write!(f, "Qkq"),
            Self::All => write!(f, "KQkq"),
        }
    }
}

/// Invalid flag input value for the `u8` provided to `CastleRights::try_from`.
/// Wasn't a valid combination of the 4 castle abilities or none.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IllegalCastleRights;

impl TryFrom<u8> for CastleRights {
    type Error = IllegalCastleRights;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::ALL_CASTLES.get(value as usize).copied().ok_or(IllegalCastleRights)
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use crate::castles::{CastleDirection, CastleRights, NUM_CASTLES};
    use std::ops::Not;
    use test_case::test_case;
    use test::{Bencher, black_box};
    use crate::player::Player;

    #[test]
    fn option_castle_is_one_byte() {
        assert_eq!(std::mem::size_of::<Option<CastleRights>>(), 1);
        assert_eq!(std::mem::size_of_val(&Some(CastleRights::BlackKing)), 1);
        assert_eq!(std::mem::size_of_val::<Option<CastleRights>>(&None), 1);
    }

    #[test]
    fn castles_ordering_is_consistent() {
        assert_eq!(CastleRights::CASTLES[0], CastleRights::WhiteKing);
        assert_eq!(CastleRights::CASTLES[1], CastleRights::WhiteQueen);
        assert_eq!(CastleRights::CASTLES[2], CastleRights::BlackKing);
        assert_eq!(CastleRights::CASTLES[3], CastleRights::BlackQueen);
        assert!(CastleRights::CASTLES.is_sorted());
    }

    #[test]
    fn castles_bitor_works() {
        assert_eq!(CastleRights::WhiteQueen | CastleRights::WhiteKing, CastleRights::WhiteBoth);
        assert_eq!(CastleRights::WhiteBoth | CastleRights::BlackBoth, CastleRights::All);
        assert_eq!(CastleRights::None | CastleRights::BlackBoth, CastleRights::BlackBoth);
        assert_eq!(CastleRights::None | CastleRights::All, CastleRights::All);
        assert_eq!(CastleRights::None | CastleRights::None, CastleRights::None);
    }

    #[test_case(CastleRights::All, CastleRights::None)]
    #[test_case(CastleRights::WhiteBoth, CastleRights::BlackBoth)]
    #[test_case(CastleRights::BothKings, CastleRights::BothQueens)]
    #[test_case(CastleRights::WhiteBothBlackQueen, CastleRights::BlackKing)]
    #[test_case(CastleRights::WhiteKingBlackQueen, CastleRights::WhiteQueenBlackKing)]
    fn castles_not_works(input: CastleRights, expected: CastleRights) {
        assert_eq!(input.not(), expected);
        assert_eq!(expected.not(), input);
    }

    #[test_case(CastleRights::None, 0b0000)]
    #[test_case(CastleRights::WhiteKing, 0b0001)]
    #[test_case(CastleRights::WhiteQueen, 0b0010)]
    #[test_case(CastleRights::WhiteBoth, 0b0011)]
    #[test_case(CastleRights::BlackKing, 0b0100)]
    #[test_case(CastleRights::BothKings, 0b0101)]
    #[test_case(CastleRights::WhiteQueenBlackKing, 0b0110)]
    #[test_case(CastleRights::WhiteBothBlackKing, 0b0111)]
    #[test_case(CastleRights::BlackQueen, 0b1000)]
    #[test_case(CastleRights::WhiteKingBlackQueen, 0b1001)]
    #[test_case(CastleRights::BothQueens, 0b1010)]
    #[test_case(CastleRights::WhiteBothBlackQueen, 0b1011)]
    #[test_case(CastleRights::BlackBoth, 0b1100)]
    #[test_case(CastleRights::WhiteKingBlackBoth, 0b1101)]
    #[test_case(CastleRights::WhiteQueenBlackBoth, 0b1110)]
    #[test_case(CastleRights::All, 0b1111)]
    fn castles_are_expected_u8(castle_rights: CastleRights, expected_repr: u8) {
        assert_eq!(castle_rights as u8, expected_repr);
    }

    #[test]
    fn num_set_bits_matches_num_castles() {
        assert_eq!((CastleRights::All as u8).count_ones(), NUM_CASTLES.try_into().unwrap());
        assert_eq!((CastleRights::None as u8).count_ones(), 0);
    }

    #[bench]
    fn for_side_white(bencher: &mut Bencher) {
        let side = black_box(Player::White);
        bencher.iter(|| assert_eq!(CastleRights::for_side(side), black_box(CastleRights::WhiteBoth)));
    }

    #[bench]
    fn for_side_black(bencher: &mut Bencher) {
        let side = black_box(Player::Black);
        bencher.iter(|| assert_eq!(CastleRights::for_side(side), black_box(CastleRights::BlackBoth)));
    }

    #[bench]
    fn can_castle_all_white_king(bencher: &mut Bencher) {
        let rights = black_box(CastleRights::All);
        let side = black_box(Player::White);
        let direction = black_box(CastleDirection::KingSide);
        bencher.iter(|| assert_eq!(CastleRights::can_castle(&rights, side, direction), black_box(true)));
    }

    #[bench]
    fn can_castle_none_black_queen(bencher: &mut Bencher) {
        let rights = black_box(CastleRights::None);
        let side = black_box(Player::Black);
        let direction = black_box(CastleDirection::QueenSide);
        bencher.iter(|| assert_eq!(CastleRights::can_castle(&rights, side, direction), black_box(false)));
    }

    #[bench]
    fn filter_all_all(bencher: &mut Bencher) {
        let start = black_box(CastleRights::All);
        let mask = black_box(CastleRights::All);
        bencher.iter(|| assert_eq!(CastleRights::filter(&start, mask), black_box(CastleRights::None)));
    }
}
