use crate::board_mask::BoardMask;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use enum_map::Enum;

/// How many rows on the board
pub const NUM_RANKS: usize = 8;
/// How many columns on the board
pub const NUM_FILES: usize = 8;

/// A single tile on a chess board
#[allow(missing_docs)]
#[rustfmt::skip]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
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

impl Square {
    /// Convert a square to a single bit set `BoardMask`
    ///
    /// ```
    /// use thermite::square::Square;
    /// use thermite::board_mask::BoardMask;
    ///
    /// assert_eq!(Square::A1.to_mask(), BoardMask::new(0b1));
    /// assert_eq!(Square::B1.to_mask(), BoardMask::new(0b10));
    /// assert_eq!(Square::C1.to_mask(), BoardMask::new(0b100));
    /// assert_eq!(Square::H1.to_mask(), BoardMask::new(0b10000000));
    /// assert_eq!(Square::H8.to_mask(), BoardMask::new(0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000));
    /// ```
    pub fn to_mask(self) -> BoardMask {
        BoardMask::new(1u64 << (self as u32))
    }

    /// Get the rank index offset 0-7 for ranks 1-8
    ///
    /// ```
    /// use thermite::square::Square;
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
    /// use thermite::square::Square;
    ///
    /// assert_eq!(Square::A3.file(), 0);
    /// assert_eq!(Square::H8.file(), 7);
    /// assert_eq!(Square::F5.file(), 5);
    /// ```
    #[must_use]
    pub const fn file(self) -> usize {
        (self as usize) % NUM_FILES
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
    use crate::square::{Square, Square::*, NUM_FILES, NUM_RANKS};
    use alloc::format;
    use core::str::FromStr;

    use test_case::test_case;

    #[test]
    fn display_works() {
        const FILES: [char; NUM_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; NUM_RANKS] = ['1', '2', '3', '4', '5', '6', '7', '8'];
        for (column, file) in FILES.into_iter().enumerate() {
            for (row, rank) in RANKS.into_iter().enumerate() {
                let input = Square::try_from((row * NUM_FILES + column) as u8).unwrap();
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
        const FILES: [char; NUM_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; NUM_RANKS] = ['1', '2', '3', '4', '5', '6', '7', '8'];
        for (column, file) in FILES.into_iter().enumerate() {
            for file in [file.to_ascii_lowercase(), file.to_ascii_uppercase()] {
                for (row, rank) in RANKS.into_iter().enumerate() {
                    let expected = Square::try_from((row * NUM_FILES + column) as u8).unwrap();
                    let input = [file as u8, rank as u8];
                    let input = core::str::from_utf8(&input).unwrap();
                    assert_eq!(Square::from_str(input).unwrap(), expected);
                }
            }
        }
    }
}
