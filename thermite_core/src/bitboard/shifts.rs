use crate::bitboard::BoardMask;
use crate::direction::Direction;

impl BoardMask {
    #[inline(always)]
    const fn get_shift_mask(direction: Direction) -> Self {
        const NOT_A_FILE_MASK: BoardMask = BoardMask(0xFEFE_FEFE_FEFE_FEFE);
        const NOT_H_FILE_MASK: BoardMask = BoardMask(0x7F7F_7F7F_7F7F_7F7F);
        match direction {
            Direction::North | Direction::South => Self::FULL,
            Direction::East | Direction::NorthEast | Direction::SouthEast => NOT_H_FILE_MASK,
            Direction::West | Direction::NorthWest | Direction::SouthWest => NOT_A_FILE_MASK,
        }
    }

    pub(super) const fn shift_raw(self, direction_shift: i32) -> Self {
        debug_assert!(
            (direction_shift as i64) < (u64::BITS as i64),
            "shifting by value large enough that it would clear entire mask"
        );
        if direction_shift.is_positive() {
            Self(self.0 << direction_shift)
        } else {
            Self(self.0 >> direction_shift.abs())
        }
    }

    /// Shift all of the set bits in a bitboard in a certain direction
    pub fn shift(self, direction: Direction) -> Self {
        let direction_shift = direction as i32;
        let masked = self & Self::get_shift_mask(direction);

        masked.shift_raw(direction_shift)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shift_north_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::North),
            BoardMask(0x040A_1024_4088_0000)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::North),
            BoardMask(0xFFFF_FFFF_FFFF_FF00)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::North),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_south_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::South),
            BoardMask(0x0003_040A_1024_4088)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::South),
            BoardMask(0x00FF_FFFF_FFFF_FFFF)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::South),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_east_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::East),
            BoardMask(0x0608_1420_4880_1000)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::East),
            BoardMask(0xFEFE_FEFE_FEFE_FEFE)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::East),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_west_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::West),
            BoardMask(0x0102_0508_1220_4400)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::West),
            BoardMask(0x7F7F_7F7F_7F7F_7F7F)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::West),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_north_east_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::NorthEast),
            BoardMask(0x0814_2048_8010_0000)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::NorthEast),
            BoardMask(0xFEFE_FEFE_FEFE_FE00)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::NorthEast),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_north_west_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::NorthWest),
            BoardMask(0x0205_0812_2044_0000)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::NorthWest),
            BoardMask(0x7F7F_7F7F_7F7F_7F00)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::NorthWest),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_south_east_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::SouthEast),
            BoardMask(0x0006_0814_2048_8010)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::SouthEast),
            BoardMask(0x00FE_FEFE_FEFE_FEFE)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::SouthEast),
            BoardMask(0x0)
        );
    }

    #[test]
    fn shift_south_west_works() {
        assert_eq!(
            BoardMask::shift(BoardMask(0x0304_0A10_2440_8800), Direction::SouthWest),
            BoardMask(0x0001_0205_0812_2044)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0xFFFF_FFFF_FFFF_FFFF), Direction::SouthWest),
            BoardMask(0x007F_7F7F_7F7F_7F7F)
        );
        assert_eq!(
            BoardMask::shift(BoardMask(0x0), Direction::SouthWest),
            BoardMask(0x0)
        );
    }
}
