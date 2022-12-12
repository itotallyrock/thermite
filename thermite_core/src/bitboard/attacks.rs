use crate::bitboard::Bitboard;
use crate::bitboard::direction::Direction;
use crate::player::Player;

impl Bitboard {
    /// Calculate the knight attacks mask for a given mask of knight attacker(s)
    pub const fn knight_attacks(self) -> Self {
        let l1 = Self(self.0 >> 1) & Self(0x7F7F_7F7F_7F7F_7F7F);
        let l2 = Self(self.0 >> 2) & Self(0x3F3F_3F3F_3F3F_3F3F);
        let r1 = Self(self.0 << 1) & Self(0xFEFE_FEFE_FEFE_FEFE);
        let r2 = Self(self.0 << 2) & Self(0xFCFC_FCFC_FCFC_FCFC);
        let h1 = l1 | r1;
        let h2 = l2 | r2;

        Self(h1.0 << 16) | Self(h1.0 >> 16) | Self(h2.0 << 8) | Self(h2.0 >> 8)
    }

    /// Calculate the king attacks mask for a given mask of king attacker(s)
    pub const fn king_attacks(mut self) -> Self {
        let attacks = self.shift(Direction::East) | self.shift(Direction::West);
        self |= attacks;

        attacks | self.shift(Direction::North) | self.shift(Direction::South)
    }

    const fn pawn_west_attacks(self, side: Player) -> Self {
        let west_attack_direction = match side {
            Player::White => Direction::NorthWest,
            Player::Black => Direction::SouthWest,
        };

        self.shift(west_attack_direction)
    }

    const fn pawn_east_attacks(self, side: Player) -> Self {
        let west_attack_direction = match side {
            Player::White => Direction::NorthEast,
            Player::Black => Direction::SouthEast,
        };

        self.shift(west_attack_direction)
    }

    /// Calculate the pawn attacks mask for a given mask of pawn attacker(s)
    pub const fn pawn_attacks(self, side: Player) -> Self {
        self.pawn_east_attacks(side) | self.pawn_west_attacks(side)
    }

    /// Calculate the single pawn push mask for a given mask of pawns
    pub const fn pawn_push(self, side: Player) -> Self {
        let direction = match side {
            Player::White => Direction::North,
            Player::Black => Direction::South,
        };

        self.shift(direction)
    }

    const fn get_sliding_mask(direction: Direction) -> Self {
        match direction {
            Direction::North => Self(0xFFFF_FFFF_FFFF_FF00),
            Direction::South => Self(0x00FF_FFFF_FFFF_FFFF),
            Direction::East => Self(0xFEFE_FEFE_FEFE_FEFE),
            Direction::West => Self(0x7F7F_7F7F_7F7F_7F7F),
            Direction::NorthEast => Self(0xFEFE_FEFE_FEFE_FE00),
            Direction::NorthWest => Self(0x7F7F_7F7F_7F7F_7F00),
            Direction::SouthEast => Self(0x00FE_FEFE_FEFE_FEFE),
            Direction::SouthWest => Self(0x007F_7F7F_7F7F_7F7F),
        }
    }

    const fn occluded_fill(mut self, occupied: Self, direction: Direction) -> Self {
        let mut flood = Self::EMPTY;
        if self != Self::EMPTY {
            let direction_shift = direction as i32;
            let empty = !occupied & Self::get_sliding_mask(direction);
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

    const fn sliding_attacks(self, occupied: Self, direction: Direction) -> Self {
        self.occluded_fill(occupied, direction).shift(direction)
    }

    /// Get the cardinal (rook) ray/sliding attacks for a given bitboard of sliders and occupied squares mask
    pub const fn cardinal_sliding_attacks(self, occupied: Self) -> Self {
        self.sliding_attacks(occupied, Direction::North)
            | self.sliding_attacks(occupied, Direction::South)
            | self.sliding_attacks(occupied, Direction::East)
            | self.sliding_attacks(occupied, Direction::West)
    }

    /// Get the diagonal (bishop) ray/sliding attacks for a given bitboard of sliders and occupied squares mask
    pub const fn ordinal_sliding_attacks(self, occupied: Self) -> Self {
        self.sliding_attacks(occupied, Direction::NorthEast)
            | self.sliding_attacks(occupied, Direction::NorthWest)
            | self.sliding_attacks(occupied, Direction::SouthEast)
            | self.sliding_attacks(occupied, Direction::SouthWest)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn south_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0000_0000), Bitboard(0x0020_0000_0000), Direction::South), Bitboard(0x2020_2020));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0400_2000_0000), Bitboard(0x0400_2000_0000), Direction::South), Bitboard(0x0004_0424_2424));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0002_0004_0020_0000), Bitboard(0x0002_0204_0020_0420), Direction::South), Bitboard(0x0200_0404_2420));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0002_0004_0020_0000), Bitboard(0x0006_0004_0421_0020), Direction::South), Bitboard(0x0202_0602_2222));
    }

    #[test]
    fn north_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x400), Bitboard(0x400), Direction::North), Bitboard(0x0404_0404_0404_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x42000), Bitboard(0x42000), Direction::North), Bitboard(0x2424_2424_2420_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0420_0100), Bitboard(0x0400_2000_0421_0100), Direction::North), Bitboard(0x0404_2424_2001_0000));
    }

    #[test]
    fn east_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0010_0000_0000_0000), Bitboard(0x0010_0000_0000_0000), Direction::East), Bitboard(0x00E0_0000_0000_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0004_0000_0008_0000), Bitboard(0x0004_0000_0008_0000), Direction::East), Bitboard(0x00F8_0000_00F0_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0010_0800_0010_0000), Bitboard(0x0010_1800_0090_0000), Direction::East), Bitboard(0x00E0_1000_00E0_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0008_0000_0400), Bitboard(0x2020_0048_0000_1404), Direction::East), Bitboard(0x00C0_0070_0000_1800));
    }

    #[test]
    fn west_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x2000_0000_0000), Bitboard(0x2000_0000_0000), Direction::West), Bitboard(0x1F00_0000_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0008_0000_0040_0000), Bitboard(0x0008_0000_0040_0000), Direction::West), Bitboard(0x0007_0000_003F_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0800_0000_0020_2000), Bitboard(0x0900_0000_0030_2400), Direction::West), Bitboard(0x0700_0000_0010_1C00));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0004_0020_0000), Bitboard(0x0020_1005_0028_0000), Direction::West), Bitboard(0x001F_0003_0018_0000));
    }

    #[test]
    fn north_west_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0000_0000), Bitboard(0x0020_0000_0000), Direction::NorthWest), Bitboard(0x0408_1000_0000_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0008_0000), Bitboard(0x0020_0008_0000), Direction::NorthWest), Bitboard(0x0408_1102_0400_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0040_1000_0000_1000), Bitboard(0x2040_1002_0000_1000), Direction::NorthWest), Bitboard(0x2408_0002_0408_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0040_1000_0000_1000), Bitboard(0x4040_1020_0400_1800), Direction::NorthWest), Bitboard(0x2408_0000_0408_0000));
    }

    #[test]
    fn north_east_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x1000_0000), Bitboard(0x1000_0000), Direction::NorthEast), Bitboard(0x0080_4020_0000_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0002_0020_0000), Bitboard(0x0002_0020_0000), Direction::NorthEast), Bitboard(0x1008_0480_4000_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0400_0004_2000), Bitboard(0x1000_0410_0004_2000), Direction::NorthEast), Bitboard(0x1008_0010_8840_0000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0200_0800_0010), Bitboard(0x2600_0840_0030), Direction::NorthEast), Bitboard(0x0804_2010_0040_2000));
    }

    #[test]
    fn south_west_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0010_0000_0000), Bitboard(0x0010_0000_0000), Direction::SouthWest), Bitboard(0x0804_0201));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0008_0000_0010_0000), Bitboard(0x0008_0000_0010_0000), Direction::SouthWest), Bitboard(0x0402_0100_0804));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0004_0020_0000_2000), Bitboard(0x0004_0020_0008_2010), Direction::SouthWest), Bitboard(0x0201_1008_0010));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0004_0000_8800_0000), Bitboard(0x0004_0400_8880_2200), Direction::SouthWest), Bitboard(0x0201_0044_2200));
    }

    #[test]
    fn south_east_sliding_attacks_works() {
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0010_0000_0000), Bitboard(0x0010_0000_0000), Direction::SouthEast), Bitboard(0x2040_8000));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x1002_0000_0000), Bitboard(0x1002_0000_0000), Direction::SouthEast), Bitboard(0x0020_4488_1020));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x0020_0400_0400_0000), Bitboard(0x0020_0480_0420_0000), Direction::SouthEast), Bitboard(0x4088_1028_1020));
        assert_eq!(Bitboard::sliding_attacks(Bitboard(0x2200_0002_0000), Bitboard(0x2220_0026_2000), Direction::SouthEast), Bitboard(0x0044_8810_2408));
    }

    #[test]
    fn cardinal_sliding_attacks_works() {
        assert_eq!(Bitboard::cardinal_sliding_attacks(Bitboard(0x2000_0000_0000), Bitboard(0x2000_0000_0000)), Bitboard(0x2020_DF20_2020_2020));
        assert_eq!(Bitboard::cardinal_sliding_attacks(Bitboard(0x2000_0004_0000), Bitboard(0x2000_0004_0000)), Bitboard(0x2424_DF24_24FB_2424));
        assert_eq!(Bitboard::cardinal_sliding_attacks(Bitboard(0x2002_0400_0000), Bitboard(0x0022_200a_1400_0400)), Bitboard(0x0426_DF2D_3B26_2622));
        assert_eq!(Bitboard::cardinal_sliding_attacks(Bitboard(0x0040_0002_0010_0000), Bitboard(0x0048_400a_0130_0000)), Bitboard(0x52BA_521D_122F_1212));
    }

    #[test]
    fn ordinal_sliding_attacks() {
        assert_eq!(Bitboard::ordinal_sliding_attacks(Bitboard(0x0800_0000_0000), Bitboard(0x0800_0000_0000)), Bitboard(0x2214_0014_2241_8000));
        assert_eq!(Bitboard::ordinal_sliding_attacks(Bitboard(0x0800_0040_0000), Bitboard(0x0800_0040_0000)), Bitboard(0x2214_0814_A241_A010));
        assert_eq!(Bitboard::ordinal_sliding_attacks(Bitboard(0x0420_0000_2000), Bitboard(0x0010_0420_1100_2020)), Bitboard(0x158B_520E_D9D0_0050));
        assert_eq!(Bitboard::ordinal_sliding_attacks(Bitboard(0x0010_0002_0000_0080), Bitboard(0x2010_0c06_01a8_0080)), Bitboard(0x2800_2D40_8528_4000));
    }

    #[test]
    fn king_attacks_works() {
        assert_eq!(Bitboard::king_attacks(Bitboard(1)), Bitboard(0x302u64));
        assert_eq!(Bitboard::king_attacks(Bitboard(0x0020_0000_0000_u64)), Bitboard(0x7050_7000_0000_u64));
        assert_eq!(Bitboard::king_attacks(Bitboard(0x0080_0000_0000_0000_u64)), Bitboard(0xC040_C000_0000_0000_u64));
    }

    #[test]
    fn knight_attacks_works() {
        assert_eq!(Bitboard::knight_attacks(Bitboard(0x0400_0000_0000_u64)), Bitboard(0x0A11_0011_0A00_0000_u64));
        assert_eq!(Bitboard::knight_attacks(Bitboard(0x0020_0000_0000_u64)), Bitboard(0x0050_8800_8850_0000_u64));
        assert_eq!(Bitboard::knight_attacks(Bitboard(0x80u64)), Bitboard(0x0040_2000_u64));
    }

    #[test]
    fn pawn_west_attacks_works() {
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard::EMPTY, Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard::EMPTY, Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x0010_0000), Player::White), Bitboard(0x0800_0000));
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x20000), Player::White), Bitboard(0x0100_0000));
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x0100_0000), Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x0010_0000), Player::Black), Bitboard(0x800));
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x20000), Player::Black), Bitboard(0x100));
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x0100_0000), Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0xFF00), Player::White), Bitboard(0x007F_0000));
        assert_eq!(Bitboard::pawn_west_attacks(Bitboard(0x00FF_0000_0000_0000), Player::Black), Bitboard(0x7F00_0000_0000));
    }

    #[test]
    fn pawn_east_attacks_works() {
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard::EMPTY, Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard::EMPTY, Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x0010_0000), Player::White), Bitboard(0x2000_0000));
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x2000_0000), Player::White), Bitboard(0x0040_0000_0000));
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x8000_0000_0000), Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x0010_0000), Player::Black), Bitboard(0x2000));
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x2000_0000), Player::Black), Bitboard(0x0040_0000));
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x8000_0000_0000), Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0xFF00), Player::White), Bitboard(0x00FE_0000));
        assert_eq!(Bitboard::pawn_east_attacks(Bitboard(0x00FF_0000_0000_0000), Player::Black), Bitboard(0xFE00_0000_0000));
    }

    #[test]
    fn pawn_attacks_works() {
        assert_eq!(Bitboard::pawn_attacks(Bitboard::EMPTY, Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_attacks(Bitboard::EMPTY, Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0x0800_0000), Player::White), Bitboard(0x0014_0000_0000));
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0x0800_0000), Player::Black), Bitboard(0x0014_0000));
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0x2010_0440_0000), Player::White), Bitboard(0x0050_280A_A000_0000));
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0x2010_0440_0000), Player::Black), Bitboard(0x0050_280A_A000));
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0xFF00), Player::White), Bitboard(0x00FF_0000));
        assert_eq!(Bitboard::pawn_attacks(Bitboard(0x00FF_0000_0000_0000), Player::Black), Bitboard(0xFF00_0000_0000));
    }

    #[test]
    fn pawn_pushes_works() {
        assert_eq!(Bitboard::pawn_push(Bitboard::EMPTY, Player::White), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_push(Bitboard::EMPTY, Player::Black), Bitboard::EMPTY);
        assert_eq!(Bitboard::pawn_push(Bitboard(0x0800_0000), Player::White), Bitboard(0x0008_0000_0000));
        assert_eq!(Bitboard::pawn_push(Bitboard(0x0800_0000), Player::Black), Bitboard(0x80000));
        assert_eq!(Bitboard::pawn_push(Bitboard(0x2010_0440_0000), Player::White), Bitboard(0x0020_1004_4000_0000));
        assert_eq!(Bitboard::pawn_push(Bitboard(0x2010_0440_0000), Player::Black), Bitboard(0x0020_1004_4000));
        assert_eq!(Bitboard::pawn_push(Bitboard(0xFF00), Player::White), Bitboard(0x00FF_0000));
        assert_eq!(Bitboard::pawn_push(Bitboard(0x00FF_0000_0000_0000), Player::Black), Bitboard(0xFF00_0000_0000));
    }
}