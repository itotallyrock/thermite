mod shift;
mod direction;
mod attacks;

use crate::square::{Square, NUM_FILES, NUM_RANKS, BySquare, NUM_SQUARES};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use crate::piece_type::PieceType;

/// The raw bitboard value type
type BitboardInner = u64;

/// Board mask with single bits representing squares on a 64 tile board
#[derive(Copy, Clone, Eq)]
#[must_use]
pub struct Bitboard(pub(crate) BitboardInner);

const fn line_generator<const BETWEEN: bool>() -> BySquare<BySquare<Bitboard>> {
    let mut items: BySquare<BySquare<Bitboard>> = BySquare::default();
    let mut a_square_offset = 0;
    while a_square_offset < NUM_SQUARES {
        let a_square = Square::SQUARES[a_square_offset];
        let mut b_square_offset = 0;
        while b_square_offset < NUM_SQUARES {
            const SLIDING_PIECES: [PieceType; 2] = [PieceType::Rook, PieceType::Bishop];
            let b_square = Square::SQUARES[b_square_offset];
            let b_mask = b_square.to_mask();
            let mut piece_index = 0;
            while piece_index < SLIDING_PIECES.len() {
                let piece = SLIDING_PIECES[piece_index];
                let a_attacks = Bitboard::attacks_mask(piece, a_square);
                // If there is a connectable line through the two squares
                if a_attacks & b_mask != Bitboard::EMPTY {
                    *items.mut_square(a_square).mut_square(b_square) = if BETWEEN {
                        Bitboard::occluded_attacks_mask(piece, a_square, b_mask) & Bitboard::occluded_attacks_mask(piece, b_square, a_square.to_mask())
                    } else {
                        (a_attacks & Bitboard::attacks_mask(piece, b_square)) | b_mask | a_square.to_mask()
                    };
                }
                piece_index += 1;
            }
            if BETWEEN {
                *items.mut_square(a_square).mut_square(b_square) |= b_mask;
            }

            b_square_offset += 1;
        }
        a_square_offset += 1;
    }

    items
}

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

    const LINE_THROUGH: BySquare<BySquare<Self>> = line_generator::<false>();
    const LINE_BETWEEN: BySquare<BySquare<Self>> = line_generator::<true>();

    /// Get the board mask of the line through two squares, if any, the line extends from edge to edge.
    pub const fn line_through(a: Square, b: Square) -> Self {
        *Self::LINE_THROUGH.get_square(a).get_square(b)
    }

    /// Get the board mask of the line between two squares, if any, not including the start squares.
    pub const fn line_between(start: Square, end: Square) -> Self {
        *Self::LINE_BETWEEN.get_square(start).get_square(end)
    }

    /// If three squares share a common rank, file, or diagonal
    #[must_use]
    pub const fn is_aligned(a: Square, b: Square, c: Square) -> bool {
        Self::line_through(a, b) & c.to_mask() != Self::EMPTY
    }

    /// If a bit is set, return that [`Square`](Square) and unset the bit
    #[must_use]
    pub const fn pop_square(&mut self) -> Option<Square> {
        let square_offset = self.0.trailing_zeros();
        #[allow(clippy::cast_possible_truncation)]
         let square = Square::try_from(square_offset as u8).ok()?;
        // Remove bit
        self.0 ^= Self::A1.0 << square_offset;

        Some(square)
    }

    /// How many set [`Square`](squares) the mask contains
    #[must_use]
    pub const fn num_squares(&self) -> u8 {
        #[allow(clippy::cast_possible_truncation)]
        { self.0.count_ones() as u8 }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let squares = self.0.0.count_ones() as usize;

        (squares, Some(squares))
    }
}

impl ExactSizeIterator for MaskSquareIterator {}

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
    use test_case::test_case;

    #[test_case(A2, A4, A6, true)]
    #[test_case(A2, A4, A8, true)]
    #[test_case(B2, A4, A8, false)]
    #[test_case(A2, B4, A8, false)]
    #[test_case(A2, A4, B8, false)]
    #[test_case(A2, B4, B8, false)]
    #[test_case(B2, B4, B8, true)]
    #[test_case(H1, A1, C1, true)]
    #[test_case(H1, A1, C2, false)]
    #[test_case(H8, A1, D4, true)]
    fn is_aligned_works(a: Square, b: Square, c: Square, expected: bool) {
        assert_eq!(Bitboard::is_aligned(a, b, c), expected);
    }

    #[test_case(C4, F7, Bitboard(0x4081000000000))]
    #[test_case(E6, F8, Bitboard(0x400000000000000))]
    // A1-H8 diagonal
    #[test_case(A1, H8, Bitboard(0x40201008040200))]
    #[test_case(A1, G7, Bitboard(0x201008040200))]
    #[test_case(A1, F6, Bitboard(0x1008040200))]
    #[test_case(A1, E5, Bitboard(0x8040200))]
    #[test_case(B2, E5, Bitboard(0x8040000))]
    #[test_case(B2, D4, Bitboard(0x40000))]
    #[test_case(B3, D4, Bitboard(0x0))]
    // G2-G6 vertical
    #[test_case(G2, G6, Bitboard(0x4040400000))]
    #[test_case(G3, G6, Bitboard(0x4040000000))]
    #[test_case(G4, G6, Bitboard(0x4000000000))]
    #[test_case(G4, G5, Bitboard(0x0))]
    // F5-A5 horizontal
    #[test_case(F5, A5, Bitboard(0x1e00000000))]
    #[test_case(E5, A5, Bitboard(0xe00000000))]
    #[test_case(D5, A5, Bitboard(0x600000000))]
    #[test_case(D5, B5, Bitboard(0x400000000))]
    #[test_case(D5, C5, Bitboard(0x0))]
    // Non aligned between
    #[test_case(A5, B7, Bitboard(0x0))]
    #[test_case(H1, C8, Bitboard(0x0))]
    #[test_case(E4, C1, Bitboard(0x0))]
    #[test_case(E4, D1, Bitboard(0x0))]
    #[test_case(E4, F1, Bitboard(0x0))]
    #[test_case(E4, G1, Bitboard(0x0))]
    fn line_between_works(a: Square, b: Square, expected: Bitboard) {
        assert_eq!(Bitboard::line_between(a, b), expected);
    }

    // Non aligned
    #[test_case(A1, B5, Bitboard(0x0))]
    #[test_case(A1, B4, Bitboard(0x0))]
    #[test_case(A1, C4, Bitboard(0x0))]
    // Diagonal A1-H8
    #[test_case(A1, D4, Bitboard(0x8040201008040201))]
    #[test_case(B2, D4, Bitboard(0x8040201008040201))]
    #[test_case(C3, D4, Bitboard(0x8040201008040201))]
    #[test_case(D4, C3, Bitboard(0x8040201008040201))]
    #[test_case(D4, E5, Bitboard(0x8040201008040201))]
    #[test_case(D4, H8, Bitboard(0x8040201008040201))]
    #[test_case(A1, H8, Bitboard(0x8040201008040201))]
    // Diagonal A8-H1
    #[test_case(A8, D5, Bitboard(0x102040810204080))]
    #[test_case(B7, D5, Bitboard(0x102040810204080))]
    #[test_case(C6, D5, Bitboard(0x102040810204080))]
    #[test_case(D5, C6, Bitboard(0x102040810204080))]
    #[test_case(D5, E4, Bitboard(0x102040810204080))]
    #[test_case(D5, H1, Bitboard(0x102040810204080))]
    #[test_case(A8, H1, Bitboard(0x102040810204080))]
    // Non-major diagonal D8-H4
    #[test_case(E7, G5, Bitboard(0x810204080000000))]
    #[test_case(G5, E7, Bitboard(0x810204080000000))]
    #[test_case(G5, H4, Bitboard(0x810204080000000))]
    #[test_case(D8, H4, Bitboard(0x810204080000000))]
    // Vertical G1-G4
    #[test_case(G1, G4, Bitboard(0x4040404040404040))]
    #[test_case(G1, G3, Bitboard(0x4040404040404040))]
    #[test_case(G1, G2, Bitboard(0x4040404040404040))]
    #[test_case(G4, G1, Bitboard(0x4040404040404040))]
    // Horizontal A5-F5
    #[test_case(A5, F5, Bitboard(0xff00000000))]
    #[test_case(A5, E5, Bitboard(0xff00000000))]
    #[test_case(A5, D5, Bitboard(0xff00000000))]
    #[test_case(A5, C5, Bitboard(0xff00000000))]
    #[test_case(B5, C5, Bitboard(0xff00000000))]
    #[test_case(C5, F5, Bitboard(0xff00000000))]
    fn line_through_works(a: Square, b: Square, expected: Bitboard) {
        assert_eq!(Bitboard::line_through(a, b), expected);
    }
}