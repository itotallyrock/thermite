mod attacks;
mod direction;
mod shifts;

pub use direction::{
    CardinalDirection, CardinalDirectionConvertError, Direction, OrdinalDirection,
    OrdinalDirectionConvertError,
};

use crate::square::Square;
use derive_more::{
    AsRef, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Constructor, LowerHex,
    Not, Shl, ShlAssign, Shr, ShrAssign, UpperHex,
};

/// Board mask with single bits representing squares on a 64 tile board
#[derive(
    Constructor,
    Copy,
    Clone,
    Eq,
    Default,
    PartialEq,
    PartialOrd,
    Hash,
    Debug,
    UpperHex,
    LowerHex,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Not,
    AsRef,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
)]
#[must_use]
pub struct BoardMask(u64);

impl BoardMask {
    /// An empty [`bitboard`](BoardMask)
    pub const EMPTY: Self = Self(0);
    /// An entirely activated [`bitboard`](BoardMask) (every square is set)
    pub const FULL: Self = Self(!Self::EMPTY.0);
    /// A single tile activated on A1 on an otherwise empty board
    pub const A1: Self = Self(1);

    /// If a bit is set, return that [`Square`](Square) and unset the bit
    #[must_use]
    pub fn pop_square(&mut self) -> Option<Square> {
        // ALLOW: Trailing zeros for u64 can at most be 64 which is always within u8's 255 max
        #[allow(clippy::cast_possible_truncation)]
        let square_offset = self.0.trailing_zeros() as u8;
        let square = Square::try_from(square_offset).ok()?;
        // Remove bit
        *self ^= Self::A1 << square_offset;

        Some(square)
    }

    /// How many set [`Square`](squares) the mask contains
    #[must_use]
    pub const fn num_squares(&self) -> u8 {
        // ALLOW: Count ones for u64 can at most be 64 which is always within u8's 255 max
        #[allow(clippy::cast_possible_truncation)]
        {
            self.0.count_ones() as u8
        }
    }

    /// If the current [`bitboard`](BoardMask) contains no set bits
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::EMPTY)
    }
}

/// A [`bitboard`](BoardMask) to square iterator container that will emit active squares from the mask (set bits).
#[derive(Clone, Debug)]
#[must_use]
pub struct MaskSquareIterator(BoardMask);

impl IntoIterator for BoardMask {
    type Item = Square;
    type IntoIter = MaskSquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        MaskSquareIterator(self)
    }
}

impl Iterator for MaskSquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_square()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let squares = self.0.num_squares() as usize;

        (squares, Some(squares))
    }
}

impl ExactSizeIterator for MaskSquareIterator {}

#[cfg(test)]
mod test {
    use crate::bitboard::BoardMask;
    use crate::square::{Square, Square::*};
    use std::ops::Not;

    use test_case::test_case;

    #[test]
    fn is_empty_works() {
        assert!(BoardMask::EMPTY.is_empty());
        assert!(!BoardMask::EMPTY.not().is_empty());
        assert!(!BoardMask::new(0x12300).is_empty());
        assert!(!BoardMask::new(0x8400400004000).is_empty());
        assert!(!BoardMask::new(0x22000812).is_empty());
    }

    #[test]
    fn num_squares_works() {
        assert_eq!(BoardMask::EMPTY.num_squares(), 0);
        assert_eq!(BoardMask::EMPTY.not().num_squares(), 64);
        assert_eq!(BoardMask::new(0x12300).num_squares(), 4);
        assert_eq!(BoardMask::new(0x8400400004000).num_squares(), 4);
        assert_eq!(BoardMask::new(0x22000812).num_squares(), 5);
    }

    #[test_case(0x0, vec![])]
    #[test_case(0x400400000, vec![G3, C5])]
    #[test_case(0x22000812, vec![B1, E1, D2, B4, F4])]
    #[test_case(0x8400400004000, vec![G2, C5, G6, D7])]
    fn into_iter_works(mask: u64, expected: Vec<Square>) {
        assert_eq!(
            BoardMask::new(mask).into_iter().collect::<Vec<_>>(),
            expected
        );
    }
}
