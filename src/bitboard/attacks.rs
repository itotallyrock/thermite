use enum_map::EnumMap;
use crate::bitboard::direction::Direction;
use crate::bitboard::BoardMask;
use crate::player_color::PlayerColor;

impl BoardMask {
    /// Calculate the knight attacks mask for a given mask of knight attacker(s)
    pub fn knight_attacks(self) -> Self {
        let l1 = Self(self.0 >> 1) & Self(0x7F7F_7F7F_7F7F_7F7F);
        let l2 = Self(self.0 >> 2) & Self(0x3F3F_3F3F_3F3F_3F3F);
        let r1 = Self(self.0 << 1) & Self(0xFEFE_FEFE_FEFE_FEFE);
        let r2 = Self(self.0 << 2) & Self(0xFCFC_FCFC_FCFC_FCFC);
        let h1 = l1 | r1;
        let h2 = l2 | r2;

        Self(h1.0 << 16) | Self(h1.0 >> 16) | Self(h2.0 << 8) | Self(h2.0 >> 8)
    }

    /// Calculate the king attacks mask for a given mask of king attacker(s)
    pub fn king_attacks(mut self) -> Self {
        let attacks = self.shift(Direction::East) | self.shift(Direction::West);
        self |= attacks;

        attacks | self.shift(Direction::North) | self.shift(Direction::South)
    }

    /// Calculate the pawn west attacks mask for a given mask of pawn attacker(s)
    pub fn pawn_west_attacks(self, player: PlayerColor) -> Self {
        let west_attack_direction = match player {
            PlayerColor::White => Direction::NorthWest,
            PlayerColor::Black => Direction::SouthWest,
        };

        self.shift(west_attack_direction)
    }

    /// Calculate the pawn east attacks mask for a given mask of pawn attacker(s)
    pub fn pawn_east_attacks(self, player: PlayerColor) -> Self {
        let west_attack_direction = match player {
            PlayerColor::White => Direction::NorthEast,
            PlayerColor::Black => Direction::SouthEast,
        };

        self.shift(west_attack_direction)
    }

    /// Calculate the pawn attacks mask for a given mask of pawn attacker(s)
    pub fn pawn_attacks(self, player: PlayerColor) -> Self {
        self.pawn_east_attacks(player) | self.pawn_west_attacks(player)
    }

    /// Calculate the single pawn push mask for a given mask of pawns
    pub fn pawn_push(self, player: PlayerColor) -> Self {
        let direction = match player {
            PlayerColor::White => Direction::North,
            PlayerColor::Black => Direction::South,
        };

        self.shift(direction)
    }

    fn occluded_fill(mut self, occupied: Self, direction: Direction) -> Self {
        const SLIDING_MASKS: EnumMap<Direction, BoardMask> = EnumMap::from_array([
            Self(0xFFFF_FFFF_FFFF_FF00),
            Self(0x00FF_FFFF_FFFF_FFFF),
            Self(0xFEFE_FEFE_FEFE_FEFE),
            Self(0x7F7F_7F7F_7F7F_7F7F),
            Self(0xFEFE_FEFE_FEFE_FE00),
            Self(0x7F7F_7F7F_7F7F_7F00),
            Self(0x00FE_FEFE_FEFE_FEFE),
            Self(0x007F_7F7F_7F7F_7F7F),
        ]);
        let mut empty = !occupied;
        let mut flood = Self::EMPTY;
        if self != Self::EMPTY {
            let direction_shift = direction as i32;
            empty &= SLIDING_MASKS[direction];
            loop {
                flood |= self;
                self = self.shift_raw(direction_shift) & empty;
                if self == Self::EMPTY {
                    break;
                }
            }
        }

        flood
    }

    fn sliding_attacks(self, occupied: Self, direction: Direction) -> Self {
        self.occluded_fill(occupied, direction).shift(direction)
    }

    /// Get the cardinal (rook) ray/sliding attacks for a given bitboard of sliders and occupied squares mask
    pub fn cardinal_sliding_attacks(self, occupied: Self) -> Self {
        self.sliding_attacks(occupied, Direction::North)
            | self.sliding_attacks(occupied, Direction::South)
            | self.sliding_attacks(occupied, Direction::East)
            | self.sliding_attacks(occupied, Direction::West)
    }

    /// Get the diagonal (bishop) ray/sliding attacks for a given bitboard of sliders and occupied squares mask
    pub fn ordinal_sliding_attacks(self, occupied: Self) -> Self {
        self.sliding_attacks(occupied, Direction::NorthEast)
            | self.sliding_attacks(occupied, Direction::NorthWest)
            | self.sliding_attacks(occupied, Direction::SouthEast)
            | self.sliding_attacks(occupied, Direction::SouthWest)
    }
}

#[cfg(test)]
mod test {
    use crate::pieces::PieceType;
    use crate::square::Square;
    use test_case::test_case;

    use super::*;

    #[test_case(
        BoardMask(0x0020_0000_0000),
        BoardMask(0x0020_0000_0000),
        BoardMask(0x2020_2020)
    )]
    #[test_case(
        BoardMask(0x0400_2000_0000),
        BoardMask(0x0400_2000_0000),
        BoardMask(0x0004_0424_2424)
    )]
    #[test_case(
        BoardMask(0x0002_0004_0020_0000),
        BoardMask(0x0002_0204_0020_0420),
        BoardMask(0x0200_0404_2420)
    )]
    #[test_case(
        BoardMask(0x0002_0004_0020_0000),
        BoardMask(0x0006_0004_0421_0020),
        BoardMask(0x0202_0602_2222)
    )]
    fn south_sliding_attacks_works(sliders: BoardMask, occupied: BoardMask, expected: BoardMask) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::South),
            expected
        );
    }

    #[test_case(BoardMask(0x400), BoardMask(0x400), BoardMask(0x0404_0404_0404_0000))]
    #[test_case(
        BoardMask(0x42000),
        BoardMask(0x42000),
        BoardMask(0x2424_2424_2420_0000)
    )]
    #[test_case(
        BoardMask(0x0420_0100),
        BoardMask(0x0400_2000_0421_0100),
        BoardMask(0x0404_2424_2001_0000)
    )]
    fn north_sliding_attacks_works(sliders: BoardMask, occupied: BoardMask, expected: BoardMask) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::North),
            expected
        );
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::North),
            expected
        );
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::North),
            expected
        );
    }

    #[test_case(
        BoardMask(0x0010_0000_0000_0000),
        BoardMask(0x0010_0000_0000_0000),
        BoardMask(0x00E0_0000_0000_0000)
    )]
    #[test_case(
        BoardMask(0x0004_0000_0008_0000),
        BoardMask(0x0004_0000_0008_0000),
        BoardMask(0x00F8_0000_00F0_0000)
    )]
    #[test_case(
        BoardMask(0x0010_0800_0010_0000),
        BoardMask(0x0010_1800_0090_0000),
        BoardMask(0x00E0_1000_00E0_0000)
    )]
    #[test_case(
        BoardMask(0x0020_0008_0000_0400),
        BoardMask(0x2020_0048_0000_1404),
        BoardMask(0x00C0_0070_0000_1800)
    )]
    fn east_sliding_attacks_works(sliders: BoardMask, occupied: BoardMask, expected: BoardMask) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::East),
            expected
        );
    }

    #[test_case(
        BoardMask(0x2000_0000_0000),
        BoardMask(0x2000_0000_0000),
        BoardMask(0x1F00_0000_0000)
    )]
    #[test_case(
        BoardMask(0x0008_0000_0040_0000),
        BoardMask(0x0008_0000_0040_0000),
        BoardMask(0x0007_0000_003F_0000)
    )]
    #[test_case(
        BoardMask(0x0800_0000_0020_2000),
        BoardMask(0x0900_0000_0030_2400),
        BoardMask(0x0700_0000_0010_1C00)
    )]
    #[test_case(
        BoardMask(0x0020_0004_0020_0000),
        BoardMask(0x0020_1005_0028_0000),
        BoardMask(0x001F_0003_0018_0000)
    )]
    fn west_sliding_attacks_works(sliders: BoardMask, occupied: BoardMask, expected: BoardMask) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::West),
            expected
        );
    }

    #[test_case(
        BoardMask(0x1000_0000),
        BoardMask(0x1000_0000),
        BoardMask(0x0080_4020_0000_0000)
    )]
    #[test_case(
        BoardMask(0x0002_0020_0000),
        BoardMask(0x0002_0020_0000),
        BoardMask(0x1008_0480_4000_0000)
    )]
    #[test_case(
        BoardMask(0x0400_0004_2000),
        BoardMask(0x1000_0410_0004_2000),
        BoardMask(0x1008_0010_8840_0000)
    )]
    #[test_case(
        BoardMask(0x0200_0800_0010),
        BoardMask(0x2600_0840_0030),
        BoardMask(0x0804_2010_0040_2000)
    )]
    fn north_east_sliding_attacks_works(
        sliders: BoardMask,
        occupied: BoardMask,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::NorthEast),
            expected
        );
    }

    #[test_case(
        BoardMask(0x0010_0000_0000),
        BoardMask(0x0010_0000_0000),
        BoardMask(0x0804_0201)
    )]
    #[test_case(
        BoardMask(0x0008_0000_0010_0000),
        BoardMask(0x0008_0000_0010_0000),
        BoardMask(0x0402_0100_0804)
    )]
    #[test_case(
        BoardMask(0x0004_0020_0000_2000),
        BoardMask(0x0004_0020_0008_2010),
        BoardMask(0x0201_1008_0010)
    )]
    #[test_case(
        BoardMask(0x0004_0000_8800_0000),
        BoardMask(0x0004_0400_8880_2200),
        BoardMask(0x0201_0044_2200)
    )]
    fn south_west_sliding_attacks_works(
        sliders: BoardMask,
        occupied: BoardMask,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::SouthWest),
            expected
        );
    }

    #[test_case(
        BoardMask(0x0010_0000_0000),
        BoardMask(0x0010_0000_0000),
        BoardMask(0x2040_8000)
    )]
    #[test_case(
        BoardMask(0x1002_0000_0000),
        BoardMask(0x1002_0000_0000),
        BoardMask(0x0020_4488_1020)
    )]
    #[test_case(
        BoardMask(0x0020_0400_0400_0000),
        BoardMask(0x0020_0480_0420_0000),
        BoardMask(0x4088_1028_1020)
    )]
    #[test_case(
        BoardMask(0x2200_0002_0000),
        BoardMask(0x2220_0026_2000),
        BoardMask(0x0044_8810_2408)
    )]
    fn south_east_sliding_attacks_works(
        sliders: BoardMask,
        occupied: BoardMask,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::sliding_attacks(sliders, occupied, Direction::SouthEast),
            expected
        );
    }

    #[test_case(BoardMask(0x1), BoardMask(0x1), BoardMask(0x0101_0101_0101_01FE))]
    #[test_case(BoardMask(0x80), BoardMask(0x80), BoardMask(0x8080_8080_8080_807F))]
    #[test_case(
        BoardMask(0x2000_0000_0000),
        BoardMask(0x2000_0000_0000),
        BoardMask(0x2020_DF20_2020_2020)
    )]
    #[test_case(
        BoardMask(0x2000_0004_0000),
        BoardMask(0x2000_0004_0000),
        BoardMask(0x2424_DF24_24FB_2424)
    )]
    #[test_case(
        BoardMask(0x2002_0400_0000),
        BoardMask(0x0022_200a_1400_0400),
        BoardMask(0x0426_DF2D_3B26_2622)
    )]
    #[test_case(
        BoardMask(0x0040_0002_0010_0000),
        BoardMask(0x0048_400a_0130_0000),
        BoardMask(0x52BA_521D_122F_1212)
    )]
    fn cardinal_sliding_attacks_works(
        sliders: BoardMask,
        occupied: BoardMask,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::cardinal_sliding_attacks(sliders, occupied),
            expected
        );
    }

    #[test_case(
        BoardMask(0x0800_0000_0000),
        BoardMask(0x0800_0000_0000),
        BoardMask(0x2214_0014_2241_8000)
    )]
    #[test_case(
        BoardMask(0x0800_0040_0000),
        BoardMask(0x0800_0040_0000),
        BoardMask(0x2214_0814_A241_A010)
    )]
    #[test_case(
        BoardMask(0x0420_0000_2000),
        BoardMask(0x0010_0420_1100_2020),
        BoardMask(0x158B_520E_D9D0_0050)
    )]
    #[test_case(
        BoardMask(0x0010_0002_0000_0080),
        BoardMask(0x2010_0c06_01a8_0080),
        BoardMask(0x2800_2D40_8528_4000)
    )]
    fn ordinal_sliding_attacks(sliders: BoardMask, occupied: BoardMask, expected: BoardMask) {
        assert_eq!(
            BoardMask::ordinal_sliding_attacks(sliders, occupied),
            expected
        );
    }

    #[test_case(BoardMask(1), BoardMask(0x302u64))]
    #[test_case(BoardMask(0x0020_0000_0000_u64), BoardMask(0x7050_7000_0000_u64))]
    #[test_case(
        BoardMask(0x0080_0000_0000_0000_u64),
        BoardMask(0xC040_C000_0000_0000_u64)
    )]
    fn king_attacks_works(knights: BoardMask, expected: BoardMask) {
        assert_eq!(BoardMask::king_attacks(knights), expected);
    }

    #[test_case(BoardMask(0x0400_0000_0000_u64), BoardMask(0x0A11_0011_0A00_0000_u64))]
    #[test_case(BoardMask(0x0020_0000_0000_u64), BoardMask(0x0050_8800_8850_0000_u64))]
    #[test_case(BoardMask(0x80u64), BoardMask(0x0040_2000_u64))]
    fn knight_attacks_works(knights: BoardMask, expected: BoardMask) {
        assert_eq!(BoardMask::knight_attacks(knights), expected);
    }

    #[test_case(BoardMask::EMPTY, PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask::EMPTY, PlayerColor::Black, BoardMask::EMPTY)]
    #[test_case(BoardMask(0x0010_0000), PlayerColor::White, BoardMask(0x0800_0000))]
    #[test_case(BoardMask(0x20000), PlayerColor::White, BoardMask(0x0100_0000))]
    #[test_case(BoardMask(0x0100_0000), PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask(0x0010_0000), PlayerColor::Black, BoardMask(0x800))]
    #[test_case(BoardMask(0x20000), PlayerColor::Black, BoardMask(0x100))]
    #[test_case(BoardMask(0x0100_0000), PlayerColor::Black, BoardMask::EMPTY)]
    #[test_case(BoardMask(0xFF00), PlayerColor::White, BoardMask(0x007F_0000))]
    #[test_case(
        BoardMask(0x00FF_0000_0000_0000),
        PlayerColor::Black,
        BoardMask(0x7F00_0000_0000)
    )]
    fn pawn_west_attacks_works(pawns: BoardMask, player: PlayerColor, expected: BoardMask) {
        assert_eq!(BoardMask::pawn_west_attacks(pawns, player), expected);
    }

    #[test_case(BoardMask::EMPTY, PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask::EMPTY, PlayerColor::Black, BoardMask::EMPTY)]
    #[test_case(BoardMask(0x0010_0000), PlayerColor::White, BoardMask(0x2000_0000))]
    #[test_case(
        BoardMask(0x2000_0000),
        PlayerColor::White,
        BoardMask(0x0040_0000_0000)
    )]
    #[test_case(BoardMask(0x8000_0000_0000), PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask(0x0010_0000), PlayerColor::Black, BoardMask(0x2000))]
    #[test_case(BoardMask(0x2000_0000), PlayerColor::Black, BoardMask(0x0040_0000))]
    #[test_case(BoardMask(0x8000_0000_0000), PlayerColor::Black, BoardMask::EMPTY)]
    fn pawn_east_attacks_works(pawns: BoardMask, player: PlayerColor, expected: BoardMask) {
        assert_eq!(BoardMask::pawn_east_attacks(pawns, player), expected);
    }

    #[test_case(BoardMask::EMPTY, PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask::EMPTY, PlayerColor::Black, BoardMask::EMPTY)]
    #[test_case(
        BoardMask(0x0800_0000),
        PlayerColor::White,
        BoardMask(0x0014_0000_0000)
    )]
    #[test_case(BoardMask(0x0800_0000), PlayerColor::Black, BoardMask(0x0014_0000))]
    #[test_case(
        BoardMask(0x2010_0440_0000),
        PlayerColor::White,
        BoardMask(0x0050_280A_A000_0000)
    )]
    #[test_case(
        BoardMask(0x2010_0440_0000),
        PlayerColor::Black,
        BoardMask(0x0050_280A_A000)
    )]
    #[test_case(BoardMask(0xFF00), PlayerColor::White, BoardMask(0x00FF_0000))]
    fn pawn_attacks_works(pawns: BoardMask, player: PlayerColor, expected: BoardMask) {
        assert_eq!(BoardMask::pawn_attacks(pawns, player), expected);
    }

    #[test_case(
        BoardMask(0x00FF_0000_0000_0000),
        PlayerColor::Black,
        BoardMask(0xFF00_0000_0000)
    )]
    #[test_case(BoardMask::EMPTY, PlayerColor::White, BoardMask::EMPTY)]
    #[test_case(BoardMask::EMPTY, PlayerColor::Black, BoardMask::EMPTY)]
    #[test_case(
        BoardMask(0x0800_0000),
        PlayerColor::White,
        BoardMask(0x0008_0000_0000)
    )]
    #[test_case(BoardMask(0x0800_0000), PlayerColor::Black, BoardMask(0x80000))]
    #[test_case(
        BoardMask(0x2010_0440_0000),
        PlayerColor::White,
        BoardMask(0x0020_1004_4000_0000)
    )]
    #[test_case(
        BoardMask(0x2010_0440_0000),
        PlayerColor::Black,
        BoardMask(0x0020_1004_4000)
    )]
    fn pawn_pushes_works(pawns: BoardMask, player: PlayerColor, expected: BoardMask) {
        assert_eq!(BoardMask::pawn_push(pawns, player), expected);
    }

    #[test_case(Square::A1.to_mask(), BoardMask::EMPTY, Direction::NorthEast, BoardMask(0x8040_2010_0804_0200))]
    #[test_case(Square::A1.to_mask(), BoardMask(0xffff), Direction::North, BoardMask(0x100))]
    #[test_case(Square::H1.to_mask(), BoardMask(0xffff), Direction::North, BoardMask(0x8000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::North, BoardMask(0x1010_1010_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::South, BoardMask(0x0010_1010))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::East, BoardMask(0xE000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::West, BoardMask(0x0F00_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::NorthEast, BoardMask(0x0080_4020_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::SouthWest, BoardMask(0x80402))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::NorthWest, BoardMask(0x0102_0408_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::SouthEast, BoardMask(0x0020_4080))]
    fn sliding_attacks_works(
        mask: BoardMask,
        occluded: BoardMask,
        direction: Direction,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::sliding_attacks(mask, occluded, direction),
            expected
        );
    }
}
