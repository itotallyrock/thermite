use crate::bitboard::BoardMask;
use crate::direction::Direction;
use crate::piece_count::PieceCount;
use crate::pieces::{NonPawnPieceType, SlidingPieceType};
use crate::player_color::PlayerColor;
use crate::square::Square;
use alloc::vec::Vec;
use enum_iterator::all;
use enum_map::EnumMap;
use once_cell::sync::Lazy;

/// Maximum number of blocker [square](Square)s (or the number of [piece](crate::pieces::PieceType)s that can be along the cardinals) for a [rook](crate::pieces::PieceType::Rook) on a given [square](Square)
///
/// For example: on [A1](Square::A1) count all the squares on the vertical file from [A2](Square::A2)-[A7](Square::A7) (6) and the horizontal rank from [B1](Square::B1)-[G1](Square::G1) (6) which total to 12
#[rustfmt::skip]
static ROOK_BLOCKER_COUNTS: EnumMap<Square, PieceCount> = EnumMap::from_array([
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
]);

/// Maximum number of blocker [square](Square)s (or the number of [piece](crate::pieces::PieceType)s that can be along the diagonals) for a [bishop](crate::pieces::PieceType::Bishop) on a given [square](Square)
#[rustfmt::skip]
static BISHOP_BLOCKER_COUNTS: EnumMap<Square, PieceCount> = EnumMap::from_array([
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
]);

/// [Mask](BoardMask) of relevant squares that could block a [bishop](crate::piece_type::PieceType::Bishop) on a given [square](Square)
#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
static BISHOP_OCCUPANCY_MASK: EnumMap<Square, BoardMask> = EnumMap::from_array([
    BoardMask(0x40201008040200), BoardMask(0x402010080400),   BoardMask(0x4020100A00),     BoardMask(0x40221400),       BoardMask(0x2442800),        BoardMask(0x204085000),      BoardMask(0x20408102000),    BoardMask(0x2040810204000),
    BoardMask(0x20100804020000), BoardMask(0x40201008040000), BoardMask(0x4020100A0000),   BoardMask(0x4022140000),     BoardMask(0x244280000),      BoardMask(0x20408500000),    BoardMask(0x2040810200000),  BoardMask(0x4081020400000),
    BoardMask(0x10080402000200), BoardMask(0x20100804000400), BoardMask(0x4020100A000A00), BoardMask(0x402214001400),   BoardMask(0x24428002800),    BoardMask(0x2040850005000),  BoardMask(0x4081020002000),  BoardMask(0x8102040004000),
    BoardMask(0x8040200020400),  BoardMask(0x10080400040800), BoardMask(0x20100A000A1000), BoardMask(0x40221400142200), BoardMask(0x2442800284400),  BoardMask(0x4085000500800),  BoardMask(0x8102000201000),  BoardMask(0x10204000402000),
    BoardMask(0x4020002040800),  BoardMask(0x8040004081000),  BoardMask(0x100A000A102000), BoardMask(0x22140014224000), BoardMask(0x44280028440200), BoardMask(0x8500050080400),  BoardMask(0x10200020100800), BoardMask(0x20400040201000),
    BoardMask(0x2000204081000),  BoardMask(0x4000408102000),  BoardMask(0xA000A10204000),  BoardMask(0x14001422400000), BoardMask(0x28002844020000), BoardMask(0x50005008040200), BoardMask(0x20002010080400), BoardMask(0x40004020100800),
    BoardMask(0x20408102000),    BoardMask(0x40810204000),    BoardMask(0xA1020400000),    BoardMask(0x142240000000),   BoardMask(0x284402000000),   BoardMask(0x500804020000),   BoardMask(0x201008040200),   BoardMask(0x402010080400),
    BoardMask(0x2040810204000),  BoardMask(0x4081020400000),  BoardMask(0xA102040000000),  BoardMask(0x14224000000000), BoardMask(0x28440200000000), BoardMask(0x50080402000000), BoardMask(0x20100804020000), BoardMask(0x40201008040200),
]);

/// [Mask](BoardMask) of relevant squares that could block a [rook](crate::piece_type::PieceType::Rook) on a given [square](Square)
#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
static ROOK_OCCUPANCY_MASK: EnumMap<Square, BoardMask> = EnumMap::from_array([
    BoardMask(0x101010101017E),    BoardMask(0x202020202027C),    BoardMask(0x404040404047A),    BoardMask(0x8080808080876),    BoardMask(0x1010101010106E),   BoardMask(0x2020202020205E),   BoardMask(0x4040404040403E),   BoardMask(0x8080808080807E),
    BoardMask(0x1010101017E00),    BoardMask(0x2020202027C00),    BoardMask(0x4040404047A00),    BoardMask(0x8080808087600),    BoardMask(0x10101010106E00),   BoardMask(0x20202020205E00),   BoardMask(0x40404040403E00),   BoardMask(0x80808080807E00),
    BoardMask(0x10101017E0100),    BoardMask(0x20202027C0200),    BoardMask(0x40404047A0400),    BoardMask(0x8080808760800),    BoardMask(0x101010106E1000),   BoardMask(0x202020205E2000),   BoardMask(0x404040403E4000),   BoardMask(0x808080807E8000),
    BoardMask(0x101017E010100),    BoardMask(0x202027C020200),    BoardMask(0x404047A040400),    BoardMask(0x8080876080800),    BoardMask(0x1010106E101000),   BoardMask(0x2020205E202000),   BoardMask(0x4040403E404000),   BoardMask(0x8080807E808000),
    BoardMask(0x1017E01010100),    BoardMask(0x2027C02020200),    BoardMask(0x4047A04040400),    BoardMask(0x8087608080800),    BoardMask(0x10106E10101000),   BoardMask(0x20205E20202000),   BoardMask(0x40403E40404000),   BoardMask(0x80807E80808000),
    BoardMask(0x17E0101010100),    BoardMask(0x27C0202020200),    BoardMask(0x47A0404040400),    BoardMask(0x8760808080800),    BoardMask(0x106E1010101000),   BoardMask(0x205E2020202000),   BoardMask(0x403E4040404000),   BoardMask(0x807E8080808000),
    BoardMask(0x7E010101010100),   BoardMask(0x7C020202020200),   BoardMask(0x7A040404040400),   BoardMask(0x76080808080800),   BoardMask(0x6E101010101000),   BoardMask(0x5E202020202000),   BoardMask(0x3E404040404000),   BoardMask(0x7E808080808000),
    BoardMask(0x7E01010101010100), BoardMask(0x7C02020202020200), BoardMask(0x7A04040404040400), BoardMask(0x7608080808080800), BoardMask(0x6E10101010101000), BoardMask(0x5E20202020202000), BoardMask(0x3E40404040404000), BoardMask(0x7E80808080808000),
]);

impl BoardMask {
    fn pdep(self, occupancy_mask: Self) -> Self {
        // Self(Pdep::pdep(self.0, occupancy_mask.0))
        todo!("pdep")
    }
}

impl BoardMask {
    fn pext(self, occupancy_mask: Self) -> Self {
        // Self(Pext::pext(self.0, occupancy_mask.0))
        todo!("pext")
    }
}

/// Get the squares that could block a sliding piece on a square
fn get_occupancy_mask<const IS_ROOK: bool>(square: Square) -> BoardMask {
    let occupancy_masks = if IS_ROOK {
        ROOK_OCCUPANCY_MASK
    } else {
        BISHOP_OCCUPANCY_MASK
    };

    occupancy_masks[square]
}

/// Get the number of potential squares that could block a specific sliding piece on a given square
fn get_blocker_count<const IS_ROOK: bool>(square: Square) -> PieceCount {
    let blocker_counts = if IS_ROOK {
        &ROOK_BLOCKER_COUNTS
    } else {
        &BISHOP_BLOCKER_COUNTS
    };

    blocker_counts[square]
}

/// Get the sliding attack mask for a rook or bishop on a square given a blocker index
fn get_sliding_attack<const IS_ROOK: bool>(square: Square, blocker_index: usize) -> BoardMask {
    let square_mask = square.to_mask();
    let occupancy_mask = get_occupancy_mask::<IS_ROOK>(square);
    let occupied_mask = BoardMask::new(blocker_index as u64).pdep(occupancy_mask);
    let occupied_mask = square_mask | occupied_mask;

    if IS_ROOK {
        BoardMask::cardinal_sliding_attacks(square_mask, occupied_mask)
    } else {
        BoardMask::ordinal_sliding_attacks(square_mask, occupied_mask)
    }
}

/// Get a mask of sliding attacks (for rook or bishop) for all possible blocker combinations on a given square
fn get_sliding_attacks<const IS_ROOK: bool>() -> EnumMap<Square, Vec<BoardMask>> {
    all::<Square>().fold(EnumMap::default(), |mut square_map, square| {
        let blocker_count = get_blocker_count::<IS_ROOK>(square);
        // There are 2^blocker_count possible arrangements of blockers for this square
        let max_blocker_combinations = 1 << blocker_count;
        square_map[square] = (0..max_blocker_combinations)
            .map(|blocker_index| get_sliding_attack::<{ IS_ROOK }>(square, blocker_index))
            .collect();
        square_map
    })
}

/// Precomputed attack mask lookup for a [Rook](crate::pieces::PieceType::Rook) on a [square](Square) on an [occupied board](BoardMask)
/// Occupancy is indexed by PEXT to determine an offset using a masked extraction for relevant occupancy squares (squares that can block a rook).
static ROOK_ATTACKS: Lazy<EnumMap<Square, Vec<BoardMask>>> = Lazy::new(get_sliding_attacks::<true>);

/// Precomputed attack mask lookup for a [Bishop](crate::pieces::PieceType::Bishop) on a [square](Square) on an [occupied board](BoardMask)
/// Occupancy is indexed by PEXT to determine an offset using a masked extraction for relevant occupancy squares (squares that can block a bishop).
static BISHOP_ATTACKS: Lazy<EnumMap<Square, Vec<BoardMask>>> =
    Lazy::new(get_sliding_attacks::<false>);

/// Precomputed attack mask lookup for a piece on a square on an empty board
static PSEUDO_ATTACKS: Lazy<EnumMap<NonPawnPieceType, EnumMap<Square, BoardMask>>> =
    Lazy::new(|| {
        let mut piece_mask_map: EnumMap<NonPawnPieceType, EnumMap<Square, BoardMask>> =
            EnumMap::default();
        for sq in all::<Square>() {
            let mask = sq.to_mask();
            let ordinal_attacks = mask.ordinal_sliding_attacks(mask);
            let cardinal_attacks = mask.cardinal_sliding_attacks(mask);

            piece_mask_map[NonPawnPieceType::Knight][sq] = mask.knight_attacks();
            piece_mask_map[NonPawnPieceType::Bishop][sq] = ordinal_attacks;
            piece_mask_map[NonPawnPieceType::Rook][sq] = cardinal_attacks;
            piece_mask_map[NonPawnPieceType::Queen][sq] = ordinal_attacks | cardinal_attacks;
            piece_mask_map[NonPawnPieceType::King][sq] = mask.king_attacks();
        }
        piece_mask_map
    });

impl BoardMask {
    /// Get the attack [mask](Self) for a [sliding piece](SlidingPieceType) on a [`Square`] on an [occupied board](BoardMask)
    pub fn sliding_attacks_for(piece: SlidingPieceType, square: Square, occupied: Self) -> Self {
        match piece {
            SlidingPieceType::Bishop => {
                BISHOP_ATTACKS[square][occupied.pext(BISHOP_OCCUPANCY_MASK[square]).0 as usize]
            }
            SlidingPieceType::Rook => {
                ROOK_ATTACKS[square][occupied.pext(ROOK_OCCUPANCY_MASK[square]).0 as usize]
            }
            SlidingPieceType::Queen => {
                Self::sliding_attacks_for(SlidingPieceType::Rook, square, occupied)
                    | Self::sliding_attacks_for(SlidingPieceType::Bishop, square, occupied)
            }
        }
    }

    /// Get the attack [mask](Self) for a [non-sliding piece](NonPawnPieceType) on a [`Square`] on an empty board
    pub fn pseudo_attacks_for(piece: NonPawnPieceType, square: Square) -> Self {
        PSEUDO_ATTACKS[piece][square]
    }

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
            BoardMask(0xFFFF_FFFF_FFFF_FF00),
            BoardMask(0x00FF_FFFF_FFFF_FFFF),
            BoardMask(0xFEFE_FEFE_FEFE_FEFE),
            BoardMask(0x7F7F_7F7F_7F7F_7F7F),
            BoardMask(0xFEFE_FEFE_FEFE_FE00),
            BoardMask(0x7F7F_7F7F_7F7F_7F00),
            BoardMask(0x00FE_FEFE_FEFE_FEFE),
            BoardMask(0x007F_7F7F_7F7F_7F7F),
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
    use crate::square::Square::*;
    use crate::square::{File, Rank};
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

    #[test_case(A1.to_mask(), BoardMask::EMPTY, Direction::NorthEast, BoardMask(0x8040_2010_0804_0200))]
    #[test_case(A1.to_mask(), BoardMask(0xffff), Direction::North, BoardMask(0x100))]
    #[test_case(H1.to_mask(), BoardMask(0xffff), Direction::North, BoardMask(0x8000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::North, BoardMask(0x1010_1010_0000_0000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::South, BoardMask(0x0010_1010))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::East, BoardMask(0xE000_0000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::West, BoardMask(0x0F00_0000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::NorthEast, BoardMask(0x0080_4020_0000_0000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::SouthWest, BoardMask(0x80402))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::NorthWest, BoardMask(0x0102_0408_0000_0000))]
    #[test_case(E4.to_mask(), E4.to_mask(), Direction::SouthEast, BoardMask(0x0020_4080))]
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

    #[test_case(
        SlidingPieceType::Rook,
        Square::A1,
        BoardMask::EMPTY,
        BoardMask(0x0101_0101_0101_01FE)
    )]
    #[test_case(
        SlidingPieceType::Rook,
        Square::H1,
        BoardMask::EMPTY,
        BoardMask(0x8080_8080_8080_807F)
    )]
    #[test_case(SlidingPieceType::Rook, Square::A1, BoardMask(0xFFFF), BoardMask(0x102); "startpos white queenside Rook")]
    #[test_case(SlidingPieceType::Rook, Square::H1, BoardMask(0xFFFF), BoardMask(0x8040); "startpos white kingside Rook")]
    #[test_case(
        SlidingPieceType::Bishop,
        Square::A1,
        BoardMask::EMPTY,
        BoardMask(0x8040_2010_0804_0200)
    )]
    #[test_case(
        SlidingPieceType::Rook,
        Square::B4,
        BoardMask(0x2200_3300_0802),
        BoardMask(0x0202_1D02_0202)
    )]
    #[test_case(SlidingPieceType::Bishop, Square::D4, BoardMask::FULL, BoardMask(0x0014_0014_0000); "1 Bishop, all blocking")]
    #[test_case(SlidingPieceType::Bishop, Square::F6, BoardMask(0x2000_0000_0000), BoardMask(0x8850_0050_8804_0201); "1 Bishop, no blocking a")]
    #[test_case(SlidingPieceType::Bishop, Square::H1, BoardMask(0x80), BoardMask(0x0102_0408_1020_4000); "1 Bishop, no blocking b")]
    #[test_case(SlidingPieceType::Bishop, Square::C4, BoardMask(0x0020_0140_0402_4004), BoardMask(0x0020_110A_000A_1020); "1 attacker multiple blocking")]
    #[test_case(
        SlidingPieceType::Bishop,
        Square::G6,
        BoardMask(0x4000_F800_0000),
        BoardMask(0x10A0_00A0_1000_0000)
    )]
    #[test_case(SlidingPieceType::Rook, Square::D4, BoardMask::FULL, BoardMask(0x0008_1408_0000); "1 Rook, all blocking")]
    #[test_case(SlidingPieceType::Rook, Square::F6, BoardMask(0x2000_0000_0000), BoardMask(0x2020_DF20_2020_2020); "1 Rook, no blocking a")]
    #[test_case(SlidingPieceType::Rook, Square::H1, BoardMask(0x80), BoardMask(0x8080_8080_8080_807F); "1 Rook, no blocking b")]
    #[test_case(SlidingPieceType::Rook, Square::C4, BoardMask(0x0004_2500_1000), BoardMask(0x0004_3B04_0404); "1 attacker multiple blocking a")]
    #[test_case(SlidingPieceType::Rook, Square::G6, BoardMask(0x4000_F800_0000), BoardMask(0x4040_BF40_4000_0000); "1 Rook, multiple blocking b")]
    #[test_case(SlidingPieceType::Queen, Square::D4, BoardMask::FULL, BoardMask(0x001C_141C_0000); "1 Queen, all blocking")]
    #[test_case(SlidingPieceType::Queen, Square::D4, BoardMask(0x2000_0000_0000), BoardMask(0x0809_2A1C_F71C_2A49); "1 Queen, no blocking a")]
    #[test_case(SlidingPieceType::Queen, Square::H1, BoardMask(0x80), BoardMask(0x8182_8488_90A0_C07F); "1 Queen, no blocking b")]
    #[test_case(SlidingPieceType::Queen, Square::F3, BoardMask(0x0038_0062_2000), BoardMask(0x00A8_705E_7088); "1 attacker multiple blocking c")]
    #[test_case(SlidingPieceType::Queen, Square::G6, BoardMask(0x4000_F800_0000), BoardMask(0x50E0_BFE0_5000_0000); "1 Queen, multiple blocking b")]
    fn occluded_attacks_mask_works(
        piece: SlidingPieceType,
        square: Square,
        occupied: BoardMask,
        expected: BoardMask,
    ) {
        assert_eq!(
            BoardMask::sliding_attacks_for(piece, square, occupied),
            expected
        );
    }

    #[test_case(Square::A1, true, BoardMask(0x0001_0101_0101_017E))]
    #[test_case(Square::A1, false, BoardMask(0x0040_2010_0804_0200))]
    #[test_case(Square::C4, true, BoardMask(0x0004_0404_7a04_0400))]
    #[test_case(Square::C4, false, BoardMask(0x0020_100A_000A_1000))]
    #[test_case(Square::B4, true, BoardMask(0x0002_0202_7C02_0200))]
    #[test_case(Square::B4, false, BoardMask(0x0010_0804_0004_0800))]
    #[test_case(Square::H1, true, BoardMask(0x0080_8080_8080_807E))]
    #[test_case(Square::H1, false, BoardMask(0x0002_0408_1020_4000))]
    #[test_case(Square::C7, true, BoardMask(0x007A_0404_0404_0400))]
    #[test_case(Square::C7, false, BoardMask(0x0A10_2040_0000))]
    fn get_occupancy_mask_works(square: Square, cardinal: bool, expected: BoardMask) {
        let occupancy_mask = if cardinal {
            get_occupancy_mask::<true>(square)
        } else {
            get_occupancy_mask::<false>(square)
        };
        assert_eq!(occupancy_mask, expected);
    }

    #[test_case(&BISHOP_OCCUPANCY_MASK, &BISHOP_BLOCKER_COUNTS)]
    #[test_case(&ROOK_OCCUPANCY_MASK, &ROOK_BLOCKER_COUNTS)]
    fn board_mask_index_commutative_for_all_square_indices(
        occupancy_masks: &EnumMap<Square, BoardMask>,
        blocker_counts: &EnumMap<Square, PieceCount>,
    ) {
        for square in all::<Square>() {
            let occupancy_mask: BoardMask = occupancy_masks[square];
            let max_blockers = 1 << blocker_counts[square];
            for index in 0..max_blockers {
                assert_eq!(
                    BoardMask::new(index)
                        .pdep(occupancy_mask)
                        .pext(occupancy_mask)
                        .0,
                    index,
                    "{square} ({occupancy_mask:#?}) index: {index}"
                );
            }
        }
    }

    #[test]
    fn bishop_occupancy_mask_contains_no_edges() {
        const EDGES: BoardMask = BoardMask(0xFF81_8181_8181_81FF);
        assert_eq!(
            EDGES,
            BoardMask::RANKS[Rank::First]
                | BoardMask::RANKS[Rank::Eighth]
                | BoardMask::FILES[File::A]
                | BoardMask::FILES[File::H]
        );

        for square in all::<Square>() {
            let occupancy_mask = BISHOP_OCCUPANCY_MASK[square];
            assert_eq!(
                occupancy_mask & EDGES,
                BoardMask::EMPTY,
                "{square} ({occupancy_mask:#?}) contains edges"
            );
        }
    }

    #[test_case(true)]
    #[test_case(false)]
    fn get_blocker_count_matches_num_squares_in_occupancy_mask_for_all_squares(is_rook: bool) {
        for square in all::<Square>() {
            let blocker_count = if is_rook {
                get_blocker_count::<true>(square)
            } else {
                get_blocker_count::<false>(square)
            };
            let num_squares_blocker_count = if is_rook {
                get_occupancy_mask::<true>(square)
            } else {
                get_occupancy_mask::<false>(square)
            }
            .num_squares();
            assert_eq!(
                blocker_count, num_squares_blocker_count,
                "{square} doesn't have the correct number of blockers per occupancy_mask"
            );
        }
    }
}
