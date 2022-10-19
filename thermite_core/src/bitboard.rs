use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, BitXorAssign};
use crate::square::{NUM_FILES, NUM_RANKS, Square};

/// The raw bitboard value type
type BitboardInner = u64;

/// Board mask with single bits representing squares on a 64 tile board
#[derive(Copy, Clone, Default, Eq)]
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

/// A bitboard to square iterator container that will emit active squares from the mask (set bits).
#[derive(Clone, Debug)]
#[must_use]
pub struct MaskSquareIterator(BitboardInner);

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = MaskSquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        MaskSquareIterator(self.0)
    }
}

impl Iterator for MaskSquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop least significant bit
        #[allow(clippy::cast_possible_truncation)]
        let next_square = Square::try_from(self.0.trailing_zeros() as u8).ok()?;
        // Remove least significant bit
        self.0 ^= next_square.to_mask().0;

        Some(next_square)
    }
}

impl const From<BitboardInner> for Bitboard {
    /// Convert a raw inner bitboard to a an actual [Bitboard]
    #[must_use]
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