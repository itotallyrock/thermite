use std::fmt::{Display, Formatter};
use crate::bitboard::Bitboard;

/// A single tile on a chess board
#[rustfmt::skip]
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// How many rows on the board
pub const NUM_RANKS: usize = 8;
/// How many columns on the board
pub const NUM_FILES: usize = 8;
/// How many squares there are on the board
pub const NUM_SQUARES: usize = NUM_RANKS * NUM_FILES;

impl Square {
    /// Array of every square starting from the bottom left going files A->H and cycling through ranks 1->8
    ///
    /// ```
    /// use thermite_core::square::Square;
    ///
    /// assert_eq!(Square::SQUARES[0], Square::A1);
    /// assert_eq!(Square::SQUARES[1], Square::B1);
    /// assert_eq!(Square::SQUARES[2], Square::C1);
    /// assert_eq!(Square::SQUARES[56], Square::A8);
    /// assert_eq!(Square::SQUARES[63], Square::H8);
    /// ```
    #[rustfmt::skip]
    pub const SQUARES: [Self; NUM_SQUARES] = [
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
    ];

    /// Convert a square to a single bit set bitboard
    ///
    /// ```
    /// use thermite_core::bitboard::Bitboard;
    /// use thermite_core::square::Square;
    ///
    /// assert_eq!(Square::A1.to_mask(), Bitboard::from(0b1));
    /// assert_eq!(Square::B1.to_mask(), Bitboard::from(0b10));
    /// assert_eq!(Square::C1.to_mask(), Bitboard::from(0b100));
    /// assert_eq!(Square::H1.to_mask(), Bitboard::from(0b10000000));
    /// assert_eq!(Square::H8.to_mask(), Bitboard::from(0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000));
    /// ```
    #[must_use]
    pub const fn to_mask(self) -> Bitboard {
        Bitboard::from(1u64 << (self as u32))
    }

    /// Get the rank index offset 0-7 for ranks 1-8
    ///
    /// ```
    /// use thermite_core::square::Square;
    ///
    /// assert_eq!(Square::A1.rank(), 0);
    /// assert_eq!(Square::B3.rank(), 2);
    /// assert_eq!(Square::F5.rank(), 4);
    /// assert_eq!(Square::E8.rank(), 7);
    /// ```
    #[must_use]
    pub const fn rank(self) -> usize {
        (self as usize) / NUM_RANKS
    }

    /// Get the file index offset 0-7 for files A-H
    ///
    /// ```
    /// use thermite_core::square::Square;
    ///
    /// assert_eq!(Square::A3.file(), 0);
    /// assert_eq!(Square::H8.file(), 7);
    /// assert_eq!(Square::F5.file(), 5);
    /// ```
    #[must_use]
    pub const fn file(self) -> usize {
        (self as usize) % NUM_FILES
    }

    /// 2 character, case-insensitive, file-rank string parse into a `Square`
    ///
    /// ```
    /// use thermite_core::square::{IllegalSquare, Square};
    ///
    /// assert_eq!(Square::parse("A1"), Ok(Square::A1));
    /// assert_eq!(Square::parse("C8"), Ok(Square::C8));
    /// assert_eq!(Square::parse("A9"), Err(IllegalSquare));
    /// assert_eq!(Square::parse("L1"), Err(IllegalSquare));
    /// ```
    ///
    /// # Errors
    /// Will error if input is not a valid square on a typical 8x8 board.
    /// - Files A-H
    /// - Ranks 1-8
    pub const fn parse(input: &str) -> Result<Self, IllegalSquare> {
        Ok(match input.as_bytes() {
            b"a1" | b"A1" => Self::A1,
            b"b1" | b"B1" => Self::B1,
            b"c1" | b"C1" => Self::C1,
            b"d1" | b"D1" => Self::D1,
            b"e1" | b"E1" => Self::E1,
            b"f1" | b"F1" => Self::F1,
            b"g1" | b"G1" => Self::G1,
            b"h1" | b"H1" => Self::H1,
            b"a2" | b"A2" => Self::A2,
            b"b2" | b"B2" => Self::B2,
            b"c2" | b"C2" => Self::C2,
            b"d2" | b"D2" => Self::D2,
            b"e2" | b"E2" => Self::E2,
            b"f2" | b"F2" => Self::F2,
            b"g2" | b"G2" => Self::G2,
            b"h2" | b"H2" => Self::H2,
            b"a3" | b"A3" => Self::A3,
            b"b3" | b"B3" => Self::B3,
            b"c3" | b"C3" => Self::C3,
            b"d3" | b"D3" => Self::D3,
            b"e3" | b"E3" => Self::E3,
            b"f3" | b"F3" => Self::F3,
            b"g3" | b"G3" => Self::G3,
            b"h3" | b"H3" => Self::H3,
            b"a4" | b"A4" => Self::A4,
            b"b4" | b"B4" => Self::B4,
            b"c4" | b"C4" => Self::C4,
            b"d4" | b"D4" => Self::D4,
            b"e4" | b"E4" => Self::E4,
            b"f4" | b"F4" => Self::F4,
            b"g4" | b"G4" => Self::G4,
            b"h4" | b"H4" => Self::H4,
            b"a5" | b"A5" => Self::A5,
            b"b5" | b"B5" => Self::B5,
            b"c5" | b"C5" => Self::C5,
            b"d5" | b"D5" => Self::D5,
            b"e5" | b"E5" => Self::E5,
            b"f5" | b"F5" => Self::F5,
            b"g5" | b"G5" => Self::G5,
            b"h5" | b"H5" => Self::H5,
            b"a6" | b"A6" => Self::A6,
            b"b6" | b"B6" => Self::B6,
            b"c6" | b"C6" => Self::C6,
            b"d6" | b"D6" => Self::D6,
            b"e6" | b"E6" => Self::E6,
            b"f6" | b"F6" => Self::F6,
            b"g6" | b"G6" => Self::G6,
            b"h6" | b"H6" => Self::H6,
            b"a7" | b"A7" => Self::A7,
            b"b7" | b"B7" => Self::B7,
            b"c7" | b"C7" => Self::C7,
            b"d7" | b"D7" => Self::D7,
            b"e7" | b"E7" => Self::E7,
            b"f7" | b"F7" => Self::F7,
            b"g7" | b"G7" => Self::G7,
            b"h7" | b"H7" => Self::H7,
            b"a8" | b"A8" => Self::A8,
            b"b8" | b"B8" => Self::B8,
            b"c8" | b"C8" => Self::C8,
            b"d8" | b"D8" => Self::D8,
            b"e8" | b"E8" => Self::E8,
            b"f8" | b"F8" => Self::F8,
            b"g8" | b"G8" => Self::G8,
            b"h8" | b"H8" => Self::H8,
            _ => return Err(IllegalSquare),
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A1 => write!(f, "a1"),
            Self::B1 => write!(f, "b1"),
            Self::C1 => write!(f, "c1"),
            Self::D1 => write!(f, "d1"),
            Self::E1 => write!(f, "e1"),
            Self::F1 => write!(f, "f1"),
            Self::G1 => write!(f, "g1"),
            Self::H1 => write!(f, "h1"),

            Self::A2 => write!(f, "a2"),
            Self::B2 => write!(f, "b2"),
            Self::C2 => write!(f, "c2"),
            Self::D2 => write!(f, "d2"),
            Self::E2 => write!(f, "e2"),
            Self::F2 => write!(f, "f2"),
            Self::G2 => write!(f, "g2"),
            Self::H2 => write!(f, "h2"),

            Self::A3 => write!(f, "a3"),
            Self::B3 => write!(f, "b3"),
            Self::C3 => write!(f, "c3"),
            Self::D3 => write!(f, "d3"),
            Self::E3 => write!(f, "e3"),
            Self::F3 => write!(f, "f3"),
            Self::G3 => write!(f, "g3"),
            Self::H3 => write!(f, "h3"),

            Self::A4 => write!(f, "a4"),
            Self::B4 => write!(f, "b4"),
            Self::C4 => write!(f, "c4"),
            Self::D4 => write!(f, "d4"),
            Self::E4 => write!(f, "e4"),
            Self::F4 => write!(f, "f4"),
            Self::G4 => write!(f, "g4"),
            Self::H4 => write!(f, "h4"),

            Self::A5 => write!(f, "a5"),
            Self::B5 => write!(f, "b5"),
            Self::C5 => write!(f, "c5"),
            Self::D5 => write!(f, "d5"),
            Self::E5 => write!(f, "e5"),
            Self::F5 => write!(f, "f5"),
            Self::G5 => write!(f, "g5"),
            Self::H5 => write!(f, "h5"),

            Self::A6 => write!(f, "a6"),
            Self::B6 => write!(f, "b6"),
            Self::C6 => write!(f, "c6"),
            Self::D6 => write!(f, "d6"),
            Self::E6 => write!(f, "e6"),
            Self::F6 => write!(f, "f6"),
            Self::G6 => write!(f, "g6"),
            Self::H6 => write!(f, "h6"),

            Self::A7 => write!(f, "a7"),
            Self::B7 => write!(f, "b7"),
            Self::C7 => write!(f, "c7"),
            Self::D7 => write!(f, "d7"),
            Self::E7 => write!(f, "e7"),
            Self::F7 => write!(f, "f7"),
            Self::G7 => write!(f, "g7"),
            Self::H7 => write!(f, "h7"),

            Self::A8 => write!(f, "a8"),
            Self::B8 => write!(f, "b8"),
            Self::C8 => write!(f, "c8"),
            Self::D8 => write!(f, "d8"),
            Self::E8 => write!(f, "e8"),
            Self::F8 => write!(f, "f8"),
            Self::G8 => write!(f, "g8"),
            Self::H8 => write!(f, "h8"),
        }
    }
}

impl const PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }
}

/// The error that occurs when attempting to create a square that wouldn't be valid for a standard chess board
#[allow(clippy::module_name_repetitions)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IllegalSquare;

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(Square::A1, 0)]
    #[test_case(Square::A4, 0)]
    #[test_case(Square::A8, 0)]
    #[test_case(Square::B8, 1)]
    #[test_case(Square::B2, 1)]
    #[test_case(Square::C5, 2)]
    #[test_case(Square::C7, 2)]
    #[test_case(Square::D3, 3)]
    #[test_case(Square::D1, 3)]
    #[test_case(Square::E8, 4)]
    #[test_case(Square::E2, 4)]
    #[test_case(Square::F7, 5)]
    #[test_case(Square::F4, 5)]
    #[test_case(Square::G6, 6)]
    #[test_case(Square::G2, 6)]
    #[test_case(Square::H3, 7)]
    #[test_case(Square::H8, 7)]
    fn file_works(square: Square, file_offset: usize) {
        assert_eq!(square.file(), file_offset);
    }

    #[test_case(Square::A1, 0)]
    #[test_case(Square::A4, 3)]
    #[test_case(Square::A8, 7)]
    #[test_case(Square::B8, 7)]
    #[test_case(Square::B2, 1)]
    #[test_case(Square::C5, 4)]
    #[test_case(Square::C7, 6)]
    #[test_case(Square::D3, 2)]
    #[test_case(Square::D1, 0)]
    #[test_case(Square::E8, 7)]
    #[test_case(Square::E2, 1)]
    #[test_case(Square::F7, 6)]
    #[test_case(Square::F4, 3)]
    #[test_case(Square::G6, 5)]
    #[test_case(Square::G2, 1)]
    #[test_case(Square::H3, 2)]
    #[test_case(Square::H8, 7)]
    fn rank_works(square: Square, rank_offset: usize) {
        assert_eq!(square.rank(), rank_offset);
    }

    #[test_case("a4", Some(Square::A4))]
    #[test_case("b7", Some(Square::B7))]
    #[test_case("c8", Some(Square::C8))]
    #[test_case("e3", Some(Square::E3))]
    #[test_case("f6", Some(Square::F6))]
    #[test_case("a1", Some(Square::A1))]
    #[test_case("a8", Some(Square::A8))]
    #[test_case("h8", Some(Square::H8))]
    #[test_case("h1", Some(Square::H1))]
    #[test_case("h9", None)]
    #[test_case("h0", None)]
    #[test_case("a9", None)]
    #[test_case("a0", None)]
    #[test_case("a-1", None)]
    #[test_case("i3", None)]
    #[test_case("l3", None)]
    #[test_case("  ", None; "space")]
    #[test_case("", None; "empty")]
    #[test_case("\n", None; "newline")]
    #[test_case("\0", None; "null char")]
    fn parse_works(input: &str, expected: Option<Square>) {
        assert_eq!(Square::parse(input).ok(), expected);
        assert_eq!(Square::parse(input.to_ascii_uppercase().as_str()).ok(), expected);
    }

    #[test_case(Square::A5, "a5")]
    #[test_case(Square::E2, "e2")]
    #[test_case(Square::E8, "e8")]
    #[test_case(Square::F4, "f4")]
    #[test_case(Square::A1, "a1")]
    #[test_case(Square::A8, "a8")]
    #[test_case(Square::H8, "h8")]
    #[test_case(Square::H1, "h1")]
    fn fmt_works(input: Square, expected: &str) {
        assert_eq!(format!("{input}").as_str(), expected);
    }

    #[test]
    fn squares_ordering_is_consistent() {
        assert_eq!(Square::SQUARES[0], Square::A1);
        assert_eq!(Square::SQUARES[NUM_SQUARES - 1], Square::H8);
        assert_eq!(Square::SQUARES[4 * NUM_FILES + 3], Square::D5);
        assert!(Square::SQUARES.is_sorted());
    }
}


#[cfg(test)]
mod bench {
    extern crate test;

    use test::{Bencher, black_box};
    use crate::square::Square;

    #[bench]
    fn parse_bench(bencher: &mut Bencher) {
        let input = black_box("a4");
        bencher.iter(|| Square::parse(input));
    }

    #[bench]
    fn rank_bench(bencher: &mut Bencher) {
        let input = black_box(Square::A3);
        bencher.iter(|| Square::rank(input));
    }

    #[bench]
    fn file_bench(bencher: &mut Bencher) {
        let input = black_box(Square::C8);
        bencher.iter(|| Square::file(input));
    }

    #[bench]
    fn to_mask_bench(bencher: &mut Bencher) {
        let input = black_box(Square::H7);
        bencher.iter(|| Square::to_mask(input));
    }
}