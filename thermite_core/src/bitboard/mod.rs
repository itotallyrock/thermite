mod shift;
mod direction;
mod attacks;

use crate::square::{Square, NUM_FILES, NUM_RANKS};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// The raw bitboard value type
type BitboardInner = u64;

/// Board mask with single bits representing squares on a 64 tile board
#[derive(Copy, Clone, Eq)]
#[must_use]
pub struct Bitboard(pub(crate) BitboardInner);

impl Bitboard {
    /// An empty bitboard
    pub const EMPTY: Self = Self(0);
    /// An entirely activated bitboard (every square is set)
    pub const FULL: Self = Self(!Self::EMPTY.0);
    /// A single tile activated on A1 on an otherwise empty board
    pub const A1: Self = Self(1);
    /// Mask of each rank, starting at rank 1 to rank 8
    #[rustfmt::skip]
    pub const RANKS: [Self; NUM_RANKS] = [Self(0xFF), Self(0xFF00), Self(0x00FF_0000), Self(0xFF00_0000), Self(0x00FF_0000_0000), Self(0xFF00_0000_0000), Self(0x00FF_0000_0000_0000), Self(0xFF00_0000_0000_0000)];
    /// Mask of each file, starting at the A file to the H file
    #[rustfmt::skip]
    pub const FILES: [Self; NUM_FILES] = [Self(0x8080_8080_8080_8080), Self(0x4040_4040_4040_4040), Self(0x2020_2020_2020_2020), Self(0x1010_1010_1010_1010), Self(0x0808_0808_0808_0808), Self(0x0404_0404_0404_0404), Self(0x0202_0202_0202_0202), Self(0x0101_0101_0101_0101)];

    /// If a bit is set, return that [`Square`](Square) and unset the bit
    #[must_use]
    pub const fn pop_square(&mut self) -> Option<Square> {
        #[allow(clippy::cast_possible_truncation)]
        Square::try_from(self.0.trailing_zeros() as u8).ok()
    }
}

impl const Default for Bitboard {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl const PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl const PartialOrd for Bitboard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        #[allow(clippy::comparison_chain)] // Allow because cmp is non-const
        let ordering = if self.0 > other.0 {
            Ordering::Greater
        } else if self.0 < other.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        };

        Some(ordering)
    }
}

impl const Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}

/// A bitboard to square iterator container that will emit active squares from the mask (set bits).
#[derive(Clone, Debug)]
#[must_use]
pub struct MaskSquareIterator(Bitboard);

impl const IntoIterator for Bitboard {
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
}

impl const From<BitboardInner> for Bitboard {
    /// Convert a raw inner bitboard to a an actual [Bitboard]
    fn from(inner: BitboardInner) -> Self {
        Self(inner)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&format!("{:#X}", self.0))
        } else {
            f.write_str(&format!("{:X}", self.0))
        }
    }
}

impl const BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl const BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl const BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl const BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl const BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl const BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::square::Square::*;

    #[test]
    fn is_aligned_works() {
        assert!(Bitboard::is_aligned(A2, A4, A6));
        assert!(Bitboard::is_aligned(A2, A4, A8));
        assert!(!Bitboard::is_aligned(B2, A4, A8));
        assert!(!Bitboard::is_aligned(A2, B4, A8));
        assert!(!Bitboard::is_aligned(A2, A4, B8));
        assert!(!Bitboard::is_aligned(A2, B4, B8));
        assert!(Bitboard::is_aligned(B2, B4, B8));
        assert!(Bitboard::is_aligned(H1, A1, C1));
        assert!(!Bitboard::is_aligned(H1, A1, C2));
        assert!(Bitboard::is_aligned(H8, A1, D4));
        assert!(!Bitboard::is_aligned(H8, A1, D5));
        assert!(!Bitboard::is_aligned(H8, A2, D4));
        assert!(!Bitboard::is_aligned(H7, A1, D4));
    }

    #[test]
    fn line_between_works() {
        // A1-H8 diagonal
        assert_eq!(Bitboard::line_between(A1, H8), Bitboard(0x40201008040200));
        assert_eq!(Bitboard::line_between(A1, G7), Bitboard(0x201008040200));
        assert_eq!(Bitboard::line_between(A1, F6), Bitboard(0x1008040200));
        assert_eq!(Bitboard::line_between(A1, E5), Bitboard(0x8040200));
        assert_eq!(Bitboard::line_between(B2, E5), Bitboard(0x8040000));
        assert_eq!(Bitboard::line_between(B2, D4), Bitboard(0x40000));
        assert_eq!(Bitboard::line_between(B3, D4), Bitboard(0x0));
        // G2-G6 vertical
        assert_eq!(Bitboard::line_between(G2, G6), Bitboard(0x4040400000));
        assert_eq!(Bitboard::line_between(G3, G6), Bitboard(0x4040000000));
        assert_eq!(Bitboard::line_between(G4, G6), Bitboard(0x4000000000));
        assert_eq!(Bitboard::line_between(G4, G5), Bitboard(0x0));
        // F5-A5 horizontal
        assert_eq!(Bitboard::line_between(F5, A5), Bitboard(0x1e00000000));
        assert_eq!(Bitboard::line_between(E5, A5), Bitboard(0xe00000000));
        assert_eq!(Bitboard::line_between(D5, A5), Bitboard(0x600000000));
        assert_eq!(Bitboard::line_between(D5, B5), Bitboard(0x400000000));
        assert_eq!(Bitboard::line_between(D5, C5), Bitboard(0x0));
        // Non aligned between
        assert_eq!(Bitboard::line_between(A5, B7), Bitboard(0x0));
        assert_eq!(Bitboard::line_between(H1, C8), Bitboard(0x0));
        assert_eq!(Bitboard::line_between(E4, C1), Bitboard(0x0));
        assert_eq!(Bitboard::line_between(E4, D1), Bitboard(0x0));
        assert_eq!(Bitboard::line_between(E4, F1), Bitboard(0x0));
        assert_eq!(Bitboard::line_between(E4, G1), Bitboard(0x0));
    }

    #[test]
    fn line_through_works() {
        // Non aligned
        assert_eq!(Bitboard::line_through(A1, B5), Bitboard(0x0));
        assert_eq!(Bitboard::line_through(A1, B4), Bitboard(0x0));
        assert_eq!(Bitboard::line_through(A1, C4), Bitboard(0x0));
        // Diagonal A1-H8
        assert_eq!(Bitboard::line_through(A1, D4), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(B2, D4), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(C3, D4), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(D4, C3), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(D4, E5), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(D4, H8), Bitboard(0x8040201008040201));
        assert_eq!(Bitboard::line_through(A1, H8), Bitboard(0x8040201008040201));
        // Diagonal A8-H1
        assert_eq!(Bitboard::line_through(A8, D5), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(B7, D5), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(C6, D5), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(D5, C6), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(D5, E4), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(D5, H1), Bitboard(0x102040810204080));
        assert_eq!(Bitboard::line_through(A8, H1), Bitboard(0x102040810204080));
        // Non-major diagonal D8-H4
        assert_eq!(Bitboard::line_through(E7, G5), Bitboard(0x810204080000000));
        assert_eq!(Bitboard::line_through(G5, E7), Bitboard(0x810204080000000));
        assert_eq!(Bitboard::line_through(G5, H4), Bitboard(0x810204080000000));
        assert_eq!(Bitboard::line_through(D8, H4), Bitboard(0x810204080000000));
        // Vertical G1-G4
        assert_eq!(Bitboard::line_through(G1, G4), Bitboard(0x4040404040404040));
        assert_eq!(Bitboard::line_through(G1, G3), Bitboard(0x4040404040404040));
        assert_eq!(Bitboard::line_through(G1, G2), Bitboard(0x4040404040404040));
        assert_eq!(Bitboard::line_through(G4, G1), Bitboard(0x4040404040404040));
        // Horizontal A5-F5
        assert_eq!(Bitboard::line_through(A5, F5), Bitboard(0xff00000000));
        assert_eq!(Bitboard::line_through(A5, E5), Bitboard(0xff00000000));
        assert_eq!(Bitboard::line_through(A5, D5), Bitboard(0xff00000000));
        assert_eq!(Bitboard::line_through(A5, C5), Bitboard(0xff00000000));
        assert_eq!(Bitboard::line_through(B5, C5), Bitboard(0xff00000000));
        assert_eq!(Bitboard::line_through(C5, F5), Bitboard(0xff00000000));
    }
}