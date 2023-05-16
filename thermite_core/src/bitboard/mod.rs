use std::fmt::{Debug, Formatter};

use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, From, Not};

use crate::piece_type::PieceType;
use crate::square::{BySquare, NUM_FILES, NUM_RANKS, Square};

use lazy_static::lazy_static;

mod shift;
mod direction;
mod attacks;

/// The raw bitboard value type
type BitboardInner = u64;

/// Board mask with single bits representing squares on a 64 tile board
#[derive(Copy, Clone, Eq, Default, PartialEq, PartialOrd, From, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not)]
#[must_use]
pub struct Bitboard(pub(crate) BitboardInner);

// Get an iterator over squares that share a diagonal or cardinal line, the piece type that connected, and the attacks from that piece from the first square
fn connectable_iter() -> impl Iterator<Item=(Square, Square, PieceType, Bitboard)> {
    Square::SQUARES
        .into_iter()
        .flat_map(|a_square| Square::SQUARES
            .into_iter()
            .map(move |b_square| (a_square, b_square)))
        .filter_map(|(a_square, b_square)| [PieceType::Rook, PieceType::Bishop]
            .into_iter()
            .map(|piece| (a_square, b_square, piece, Bitboard::attacks_mask(piece, a_square)))
            .find(|&(_, b_square, _, a_attacks)| a_attacks & b_square.to_mask() != Bitboard::EMPTY))
}

lazy_static! {
    static ref LINE_THROUGH: BySquare<BySquare<Bitboard>> = {
        let mut items: BySquare<BySquare<Bitboard>> = BySquare::default();

        for (a, b, piece, _) in connectable_iter() {
            *items.mut_square(a).mut_square(b) = Bitboard::occluded_attacks_mask(piece, a, b.to_mask()) & Bitboard::occluded_attacks_mask(piece, b, a.to_mask());
        }

        items
    };

    static ref LINE_BETWEEN: BySquare<BySquare<Bitboard>> = {
        let mut items: BySquare<BySquare<Bitboard>> = BySquare::default();

        for (a, b, piece, a_attacks) in connectable_iter() {
            *items.mut_square(a).mut_square(b) = (a_attacks & Bitboard::attacks_mask(piece, b)) | b.to_mask() | a.to_mask();
        }

        items
    };
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

    /// Get the board mask of the line through two squares, if any, the line extends from edge to edge.
    pub fn line_through(a: Square, b: Square) -> Self {
        *LINE_THROUGH.get_square(a).get_square(b)
    }

    /// Get the board mask of the line between two squares, if any, not including the start squares.
    pub fn line_between(start: Square, end: Square) -> Self {
        *LINE_BETWEEN.get_square(start).get_square(end)
    }

    /// If three squares share a common rank, file, or diagonal
    #[must_use]
    pub fn is_aligned(a: Square, b: Square, c: Square) -> bool {
        !(Self::line_through(a, b) & c.to_mask()).is_empty()
    }

    /// If a bit is set, return that [`Square`](Square) and unset the bit
    #[must_use]
    pub fn pop_square(&mut self) -> Option<Square> {
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

    /// If the current bitboard contains no set bits
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Self::EMPTY)
    }
}

/// A bitboard to square iterator container that will emit active squares from the mask (set bits).
#[derive(Clone, Debug)]
#[must_use]
pub struct MaskSquareIterator(Bitboard);

impl IntoIterator for Bitboard {
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

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str(&format!("{:#X}", self.0))
        } else {
            f.write_str(&format!("{:X}", self.0))
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::square::Square::*;

    use super::*;

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

    #[test_case(C4, F7, Bitboard(0x0020_1008_0000_0000))]
    #[test_case(E6, F8, Bitboard(0x2000_0000_0000_0000))]
    // A1-H8 diagonal
    #[test_case(A1, H8, Bitboard(0x8040_2010_0804_0200))]
    #[test_case(A1, G7, Bitboard(0x0040_2010_0804_0200))]
    #[test_case(A1, F6, Bitboard(0x2010_0804_0200))]
    #[test_case(A1, E5, Bitboard(0x0010_0804_0200))]
    #[test_case(B2, E5, Bitboard(0x0010_0804_0000))]
    #[test_case(B2, D4, Bitboard(0x0804_0000))]
    #[test_case(B3, D4, Bitboard(0x0800_0000))]
    // G2-G6 vertical
    #[test_case(G2, G6, Bitboard(0x4040_4040_0000))]
    #[test_case(G3, G6, Bitboard(0x4040_4000_0000))]
    #[test_case(G4, G6, Bitboard(0x4040_0000_0000))]
    #[test_case(G4, G5, Bitboard(0x0040_0000_0000))]
    // F5-A5 horizontal
    #[test_case(F5, A5, Bitboard(0x001F_0000_0000))]
    #[test_case(E5, A5, Bitboard(0x000F_0000_0000))]
    #[test_case(D5, A5, Bitboard(0x0007_0000_0000))]
    #[test_case(D5, B5, Bitboard(0x0006_0000_0000))]
    #[test_case(D5, C5, Bitboard(0x0004_0000_0000))]
    // Non aligned between
    #[test_case(A5, B7, B7.to_mask())]
    #[test_case(H1, C8, C8.to_mask())]
    #[test_case(E4, C1, C1.to_mask())]
    #[test_case(E4, D1, D1.to_mask())]
    #[test_case(E4, F1, F1.to_mask())]
    #[test_case(E4, G1, G1.to_mask())]
    fn line_between_works(a: Square, b: Square, expected: Bitboard) {
        assert_eq!(Bitboard::line_between(a, b), expected);
    }

    // Non aligned
    #[test_case(A1, B5, Bitboard(0x0))]
    #[test_case(A1, B4, Bitboard(0x0))]
    #[test_case(A1, C4, Bitboard(0x0))]
    // Diagonal A1-H8
    #[test_case(A1, D4, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(B2, D4, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(C3, D4, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(D4, C3, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(D4, E5, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(D4, H8, Bitboard(0x8040_2010_0804_0201))]
    #[test_case(A1, H8, Bitboard(0x8040_2010_0804_0201))]
    // Diagonal A8-H1
    #[test_case(A8, D5, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(B7, D5, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(C6, D5, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(D5, C6, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(D5, E4, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(D5, H1, Bitboard(0x0102_0408_1020_4080))]
    #[test_case(A8, H1, Bitboard(0x0102_0408_1020_4080))]
    // Non-major diagonal D8-H4
    #[test_case(E7, G5, Bitboard(0x0810_2040_8000_0000))]
    #[test_case(G5, E7, Bitboard(0x0810_2040_8000_0000))]
    #[test_case(G5, H4, Bitboard(0x0810_2040_8000_0000))]
    #[test_case(D8, H4, Bitboard(0x0810_2040_8000_0000))]
    // Vertical G1-G4
    #[test_case(G1, G4, Bitboard(0x4040_4040_4040_4040))]
    #[test_case(G1, G3, Bitboard(0x4040_4040_4040_4040))]
    #[test_case(G1, G2, Bitboard(0x4040_4040_4040_4040))]
    #[test_case(G4, G1, Bitboard(0x4040_4040_4040_4040))]
    // Horizontal A5-F5
    #[test_case(A5, F5, Bitboard(0x00FF_0000_0000))]
    #[test_case(A5, E5, Bitboard(0x00FF_0000_0000))]
    #[test_case(A5, D5, Bitboard(0x00FF_0000_0000))]
    #[test_case(A5, C5, Bitboard(0x00FF_0000_0000))]
    #[test_case(B5, C5, Bitboard(0x00FF_0000_0000))]
    #[test_case(C5, F5, Bitboard(0x00FF_0000_0000))]
    fn line_through_works(a: Square, b: Square, expected: Bitboard) {
        assert_eq!(Bitboard::line_through(a, b), expected);
    }
}