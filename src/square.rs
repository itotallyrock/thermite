use crate::bitboard::BoardMask;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use enum_map::Enum;
use subenum::subenum;

/// A single tile on a chess board
#[allow(missing_docs)]
#[rustfmt::skip]
#[subenum(EnPassantSquare, WhiteEnPassantSquare, BlackEnPassantSquare)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    #[subenum(EnPassantSquare, WhiteEnPassantSquare)] A3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] B3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] C3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] D3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] E3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] F3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] G3, #[subenum(EnPassantSquare, WhiteEnPassantSquare)] H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    #[subenum(EnPassantSquare, BlackEnPassantSquare)] A6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] B6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] C6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] D6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] E6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] F6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] G6, #[subenum(EnPassantSquare, BlackEnPassantSquare)] H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// A single row on the board
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum Rank {
    /// The first rank, the lower most rank from white's pov, white's back rank
    First,
    /// The second rank, white's pawn rank
    Second,
    /// The third rank
    Third,
    /// The fourth rank
    Fourth,
    /// The fifth rank
    Fifth,
    /// The sixth rank
    Sixth,
    /// The seventh rank, black's pawn rank
    Seventh,
    /// The eighth and upper-most rank from whites pov, black's back rank
    Eighth,
}

/// A single column on the board
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
#[repr(u8)]
pub enum File {
    /// The A or first file
    A,
    /// The B or second file
    B,
    /// The C or third file
    C,
    /// The D or fourth file
    D,
    /// The E or fifth file
    E,
    /// The F or sixth file
    F,
    /// The G or seventh file
    G,
    /// The H or eighth file
    H,
}

impl Square {
    /// Convert a square to a single bit set `BoardMask`
    ///
    /// ```
    /// use thermite::bitboard::BoardMask;
    /// use thermite::square::Square;
    ///
    /// assert_eq!(Square::A1.to_mask(), BoardMask::new(0b1));
    /// assert_eq!(Square::B1.to_mask(), BoardMask::new(0b10));
    /// assert_eq!(Square::C1.to_mask(), BoardMask::new(0b100));
    /// assert_eq!(Square::H1.to_mask(), BoardMask::new(0b10000000));
    /// assert_eq!(Square::H8.to_mask(), BoardMask::new(0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000));
    /// ```
    pub fn to_mask(self) -> BoardMask {
        BoardMask::A1 << self as u32
    }

    /// Get the [`Rank`] for a [`Square`]
    ///
    /// ```
    /// use thermite::square::{Rank, Square};
    ///
    /// assert_eq!(Square::A1.rank(), Rank::First);
    /// assert_eq!(Square::B3.rank(), Rank::Third);
    /// assert_eq!(Square::F5.rank(), Rank::Fifth);
    /// assert_eq!(Square::E8.rank(), Rank::Eighth);
    /// ```
    #[must_use]
    pub const fn rank(self) -> Rank {
        // SAFETY: Because this value is at most 63usize divided by 8 (Rank::LENGTH) gives a maximum
        // of 7 for our `Rank`. Which is valid for our discriminant as there are 8 variants.
        // ALLOW: truncation is not possible when Rank::LENGTH is under 255 and 8 is well under
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, Rank>((self as u8) / Rank::LENGTH as u8)
        }
    }

    /// Get the [`File`] for a [`Square`]
    ///
    /// ```
    /// use thermite::square::{File, Square};
    ///
    /// assert_eq!(Square::A3.file(), File::A);
    /// assert_eq!(Square::H8.file(), File::H);
    /// assert_eq!(Square::F5.file(), File::F);
    /// ```
    #[must_use]
    pub const fn file(self) -> File {
        // SAFETY: Because this value is mod File::LENGTH it will always be a valid discriminant for it
        // ALLOW: truncation is not possible when File::LENGTH is under 255 and 8 is well under
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            std::mem::transmute::<u8, File>((self as u8) % File::LENGTH as u8)
        }
    }

    /// Try to add an offset to a square
    ///
    /// ```
    /// use thermite::square::Square;
    ///
    /// assert_eq!(Square::A1.checked_add(8), Some(Square::A2));
    /// assert_eq!(Square::A1.checked_add(1), Some(Square::B1));
    /// assert_eq!(Square::B1.checked_add(6), Some(Square::H1));
    /// assert_eq!(Square::A8.checked_add(7), Some(Square::H8));
    /// assert_eq!(Square::H8.checked_add(1), None);
    #[must_use]
    pub fn checked_add(self, rhs: u8) -> Option<Self> {
        Self::try_from((self as u8).saturating_add(rhs)).ok()
    }

    /// Try to subtract an offset from a square
    ///
    /// ```
    ///
    /// use thermite::square::Square;
    /// assert_eq!(Square::A1.checked_sub(0), Some(Square::A1));
    /// assert_eq!(Square::A1.checked_sub(1), None);
    /// assert_eq!(Square::H1.checked_sub(7), Some(Square::A1));
    /// assert_eq!(Square::A8.checked_sub(8), Some(Square::A7));
    /// assert_eq!(Square::H8.checked_sub(63), Some(Square::A1));
    /// assert_eq!(Square::G2.checked_sub(2), Some(Square::E2));
    /// ```
    #[must_use]
    pub fn checked_sub(self, rhs: u8) -> Option<Self> {
        Self::try_from((self as u8).wrapping_sub(rhs)).ok()
    }
}

impl FromStr for Square {
    type Err = IllegalSquare;

    /// 2 character, case-insensitive, file-rank string parse into a `Square`
    ///
    /// ```
    /// use std::str::FromStr;
    /// use thermite::square::{IllegalSquare, Square};
    ///
    /// assert_eq!(Square::from_str("A1"), Ok(Square::A1));
    /// assert_eq!(Square::from_str("C8"), Ok(Square::C8));
    /// assert_eq!(Square::from_str("A9"), Err(IllegalSquare));
    /// assert_eq!(Square::from_str("L1"), Err(IllegalSquare));
    /// ```
    ///
    /// # Errors
    /// Will error if input is not a valid square on a typical 8x8 board.
    /// - Files A-H
    /// - Ranks 1-8
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "a1" | "A1" => Self::A1,
            "b1" | "B1" => Self::B1,
            "c1" | "C1" => Self::C1,
            "d1" | "D1" => Self::D1,
            "e1" | "E1" => Self::E1,
            "f1" | "F1" => Self::F1,
            "g1" | "G1" => Self::G1,
            "h1" | "H1" => Self::H1,
            "a2" | "A2" => Self::A2,
            "b2" | "B2" => Self::B2,
            "c2" | "C2" => Self::C2,
            "d2" | "D2" => Self::D2,
            "e2" | "E2" => Self::E2,
            "f2" | "F2" => Self::F2,
            "g2" | "G2" => Self::G2,
            "h2" | "H2" => Self::H2,
            "a3" | "A3" => Self::A3,
            "b3" | "B3" => Self::B3,
            "c3" | "C3" => Self::C3,
            "d3" | "D3" => Self::D3,
            "e3" | "E3" => Self::E3,
            "f3" | "F3" => Self::F3,
            "g3" | "G3" => Self::G3,
            "h3" | "H3" => Self::H3,
            "a4" | "A4" => Self::A4,
            "b4" | "B4" => Self::B4,
            "c4" | "C4" => Self::C4,
            "d4" | "D4" => Self::D4,
            "e4" | "E4" => Self::E4,
            "f4" | "F4" => Self::F4,
            "g4" | "G4" => Self::G4,
            "h4" | "H4" => Self::H4,
            "a5" | "A5" => Self::A5,
            "b5" | "B5" => Self::B5,
            "c5" | "C5" => Self::C5,
            "d5" | "D5" => Self::D5,
            "e5" | "E5" => Self::E5,
            "f5" | "F5" => Self::F5,
            "g5" | "G5" => Self::G5,
            "h5" | "H5" => Self::H5,
            "a6" | "A6" => Self::A6,
            "b6" | "B6" => Self::B6,
            "c6" | "C6" => Self::C6,
            "d6" | "D6" => Self::D6,
            "e6" | "E6" => Self::E6,
            "f6" | "F6" => Self::F6,
            "g6" | "G6" => Self::G6,
            "h6" | "H6" => Self::H6,
            "a7" | "A7" => Self::A7,
            "b7" | "B7" => Self::B7,
            "c7" | "C7" => Self::C7,
            "d7" | "D7" => Self::D7,
            "e7" | "E7" => Self::E7,
            "f7" | "F7" => Self::F7,
            "g7" | "G7" => Self::G7,
            "h7" | "H7" => Self::H7,
            "a8" | "A8" => Self::A8,
            "b8" | "B8" => Self::B8,
            "c8" | "C8" => Self::C8,
            "d8" | "D8" => Self::D8,
            "e8" | "E8" => Self::E8,
            "f8" | "F8" => Self::F8,
            "g8" | "G8" => Self::G8,
            "h8" | "H8" => Self::H8,
            _ => return Err(IllegalSquare),
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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

/// The error that occurs when attempting to create a square that wouldn't be valid for a standard chess board
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IllegalSquare;

impl TryFrom<u8> for Square {
    type Error = IllegalSquare;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::H8 as u8 {
            Ok(<Self as Enum>::from_usize(value as usize))
        } else {
            Err(IllegalSquare)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::square::{File, File::*, Rank, Rank::*, Square, Square::*};
    use alloc::format;
    use core::str::FromStr;
    use enum_map::Enum;

    use test_case::test_case;

    #[test]
    fn display_works() {
        const FILES: [char; File::LENGTH] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; Rank::LENGTH] = ['1', '2', '3', '4', '5', '6', '7', '8'];
        for (column, file) in FILES.into_iter().enumerate() {
            for (row, rank) in RANKS.into_iter().enumerate() {
                let input = Square::try_from((row * File::LENGTH + column) as u8).unwrap();
                let expected = [file as u8, rank as u8];
                let expected = core::str::from_utf8(&expected).unwrap();
                assert_eq!(format!("{input}").as_str(), expected);
            }
        }
    }

    #[test_case(0, A1)]
    #[test_case(8, A2)]
    #[test_case(16, A3)]
    #[test_case(24, A4)]
    #[test_case(32, A5)]
    #[test_case(40, A6)]
    #[test_case(48, A7)]
    #[test_case(56, A8)]
    #[test_case(1, B1)]
    #[test_case(2, C1)]
    #[test_case(3, D1)]
    #[test_case(4, E1)]
    #[test_case(5, F1)]
    #[test_case(6, G1)]
    #[test_case(7, H1)]
    #[test_case(63, H8)]
    #[test_case(34, C5)]
    #[test_case(44, E6)]
    #[test_case(57, B8)]
    #[test_case(26, C4)]
    #[test_case(18, C3)]
    #[test_case(14, G2)]
    #[test_case(12, E2)]
    fn try_from_valid_works(input: u8, expected: Square) {
        assert_eq!(Square::try_from(input), Ok(expected));
    }

    #[test]
    fn try_from_error_works() {
        for valid_input in 0..=63 {
            assert!(Square::try_from(valid_input).is_ok());
        }
        for invalid_input in 64..255 {
            assert!(Square::try_from(invalid_input).is_err());
        }
    }

    #[test]
    fn from_str_works() {
        const FILES: [char; File::LENGTH] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; Rank::LENGTH] = ['1', '2', '3', '4', '5', '6', '7', '8'];
        for (column, file) in FILES.into_iter().enumerate() {
            for file in [file.to_ascii_lowercase(), file.to_ascii_uppercase()] {
                for (row, rank) in RANKS.into_iter().enumerate() {
                    let expected = Square::try_from((row * File::LENGTH + column) as u8).unwrap();
                    let input = [file as u8, rank as u8];
                    let input = core::str::from_utf8(&input).unwrap();
                    assert_eq!(Square::from_str(input).unwrap(), expected);
                }
            }
        }
    }

    #[test_case(E1, First)]
    #[test_case(E2, Second)]
    #[test_case(E3, Third)]
    #[test_case(E4, Fourth)]
    #[test_case(E6, Sixth)]
    #[test_case(E7, Seventh)]
    #[test_case(E8, Eighth)]
    #[test_case(A1, First)]
    #[test_case(A2, Second)]
    #[test_case(A3, Third)]
    #[test_case(A4, Fourth)]
    #[test_case(A5, Fifth)]
    #[test_case(A6, Sixth)]
    #[test_case(A7, Seventh)]
    #[test_case(A8, Eighth)]
    #[test_case(H1, First)]
    #[test_case(G2, Second)]
    #[test_case(F3, Third)]
    #[test_case(D4, Fourth)]
    #[test_case(E5, Fifth)]
    #[test_case(C6, Sixth)]
    #[test_case(B7, Seventh)]
    #[test_case(D8, Eighth)]
    fn rank_works(square: Square, rank: Rank) {
        assert_eq!(square.rank(), rank);
    }

    #[test_case(E1, E)]
    #[test_case(E2, E)]
    #[test_case(E3, E)]
    #[test_case(E4, E)]
    #[test_case(E5, E)]
    #[test_case(E6, E)]
    #[test_case(E7, E)]
    #[test_case(E8, E)]
    #[test_case(A1, A)]
    #[test_case(A2, A)]
    #[test_case(A3, A)]
    #[test_case(A4, A)]
    #[test_case(A5, A)]
    #[test_case(A6, A)]
    #[test_case(A7, A)]
    #[test_case(A8, A)]
    #[test_case(H1, H)]
    #[test_case(G2, G)]
    #[test_case(F3, F)]
    #[test_case(D4, D)]
    #[test_case(C6, C)]
    #[test_case(B7, B)]
    #[test_case(D8, D)]
    fn file_works(square: Square, file: File) {
        assert_eq!(square.file(), file);
    }

    #[test_case(A1, 0, Some(A1))]
    #[test_case(A1, 1, None)]
    #[test_case(H1, 7, Some(A1))]
    #[test_case(A8, 8, Some(A7))]
    #[test_case(H8, 63, Some(A1))]
    #[test_case(G2, 2, Some(E2))]
    fn checked_sub_works(input: Square, offset: u8, expected: Option<Square>) {
        assert_eq!(input.checked_sub(offset), expected);
    }

    #[test_case(A1, 8, Some(A2))]
    #[test_case(A1, 1, Some(B1))]
    #[test_case(B1, 6, Some(H1))]
    #[test_case(A8, 7, Some(H8))]
    #[test_case(H8, 1, None)]
    fn checked_add_works(input: Square, offset: u8, expected: Option<Square>) {
        assert_eq!(input.checked_add(offset), expected);
    }
}
