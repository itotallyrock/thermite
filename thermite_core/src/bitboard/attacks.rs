use crate::bitboard::{Bitboard, BitboardInner};
use crate::bitboard::direction::Direction;
use crate::piece_type::{ByPieceType, PieceType};
use crate::player::{ByPlayer, Player};
use crate::square::{BySquare, NUM_FILES, NUM_RANKS, NUM_SQUARES, Square};

/// Maximum number of variations of individual blockers for a rook
const ROOK_OCCUPANCY_LIMIT: u8 = 14;
/// Maximum number of variations of blockers for a bishop
const BISHOP_OCCUPANCY_LIMIT: u8 = 13;

#[derive(Copy, Clone, Debug)]
struct MagicLookup<const OCCUPANCY_LIMIT: u32> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
    occupancy_mask: Bitboard,
    attacks: [Bitboard; 1 << OCCUPANCY_LIMIT],
}

impl<const OCCUPANCY_LIMIT: u32> const Default for MagicLookup<OCCUPANCY_LIMIT> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
    fn default() -> Self {
        Self {
            occupancy_mask: Bitboard::default(),
            attacks: [Bitboard::default(); 1 << OCCUPANCY_LIMIT],
        }
    }
}

/// Bit deposit, take an index and use that value to essentially count in binary to fill a give occupancy mask
/// Index is generally valid for `0..(1 << N)` (or `2 ^ N`) where `N = occupancy_mask.count_ones()`
const fn board_mask_from_index(index: usize, occupancy_mask: Bitboard) -> Bitboard {
    let mut mask = occupancy_mask.0;
    let mut res = 0;
    let mut bb = Bitboard::A1.0;
    loop {
        if mask == 0 {
            break;
        }
        if (index as BitboardInner & bb) != 0 {
            res |= mask & mask.wrapping_neg();
        }
        mask &= mask - 1;
        bb = bb.wrapping_add(bb);
    }

    Bitboard(res)
}

/// Bit extract, pull bits from a set of occupied squares masked by an occupancy (or mask of relevant squares that when occupied will block a sliding piece)
const fn index_from_board_mask(occupied_mask: Bitboard, occupancy_mask: Bitboard) -> usize {
    let mut mask = occupancy_mask.0;
    let mut res = 0;
    let mut bb = Bitboard::A1.0;
    loop {
        if mask == 0 {
            break;
        }
        if occupied_mask.0 & mask & (mask.wrapping_neg()) != 0 {
            res |= bb;
        }
        mask &= mask - 1;
        bb = bb.wrapping_add(bb);
    }

    #[allow(clippy::cast_possible_truncation)]
    { res as usize }
}

impl<const OCCUPANCY_LIMIT: u32> MagicLookup<OCCUPANCY_LIMIT> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
    const fn get_for(&self, occupied: Bitboard) -> Bitboard {
        self.attacks[index_from_board_mask(occupied, self.occupancy_mask)]
    }
}

const fn create_sliding_attacks_occluded_masks<const OCCUPANCY_LIMIT: u32, const CARDINAL: bool>() -> [MagicLookup<OCCUPANCY_LIMIT>; NUM_SQUARES] where [(); 1 << OCCUPANCY_LIMIT]: Sized {
    const NOT_EDGES: Bitboard = !(Bitboard::RANKS[0] | Bitboard::RANKS[NUM_RANKS - 1] | Bitboard::FILES[0] | Bitboard::FILES[NUM_FILES - 1]);
    let mut attacks_by_square = [MagicLookup::<{ OCCUPANCY_LIMIT }>::default(); NUM_SQUARES];

    let mut square_index = 0;
    while square_index < NUM_SQUARES {
        #[allow(clippy::cast_possible_truncation)]
        let piece_square = Square::try_from(square_index as u8).ok().unwrap();
        let piece_mask = piece_square.to_mask();
        let occupancy_mask = if CARDINAL { piece_mask.cardinal_sliding_attacks(piece_mask) } else { piece_mask.ordinal_sliding_attacks(piece_mask) } & NOT_EDGES;
        attacks_by_square[square_index].occupancy_mask = occupancy_mask;
        let mut blocker_index = 0;
        while blocker_index < (1 << OCCUPANCY_LIMIT) {
            let blocker_mask = board_mask_from_index(blocker_index, occupancy_mask);
            let attacks = if CARDINAL { piece_mask.cardinal_sliding_attacks(blocker_mask) } else { piece_mask.ordinal_sliding_attacks(blocker_mask) };

            attacks_by_square[square_index].attacks[blocker_index] = attacks;

            blocker_index += 1;
        }
        square_index += 1;
    }

    attacks_by_square
}

impl Bitboard {
    const PSEUDO_ATTACKS: ByPieceType<BySquare<Self>> = {
        let mut items: ByPieceType<BySquare<Self>> = ByPieceType::default();

        let mut square_offset = 0;
        while (square_offset as usize) < NUM_SQUARES {
            let square = Square::try_from(square_offset).ok().unwrap();
            let square_mask = square.to_mask();
            *items.mut_piece(PieceType::King).mut_square(square) = square_mask.king_attacks();
            *items.mut_piece(PieceType::Knight).mut_square(square) = square_mask.knight_attacks();
            let cardinal_attacks = square_mask.cardinal_sliding_attacks(square_mask);
            let ordinal_attacks = square_mask.ordinal_sliding_attacks(square_mask);
            *items.mut_piece(PieceType::Rook).mut_square(square) = cardinal_attacks;
            *items.mut_piece(PieceType::Bishop).mut_square(square) = ordinal_attacks;
            *items.mut_piece(PieceType::Queen).mut_square(square) = cardinal_attacks | ordinal_attacks;

            square_offset += 1;
        }

        items
    };

    const BISHOP_OCCLUDED_ATTACKS: BySquare<MagicLookup<{ BISHOP_OCCUPANCY_LIMIT as u32 }>> = BySquare::from(create_sliding_attacks_occluded_masks::<{ BISHOP_OCCUPANCY_LIMIT as u32 }, true>());

    const ROOK_OCCLUDED_ATTACKS: BySquare<MagicLookup<{ ROOK_OCCUPANCY_LIMIT as u32 }>> = BySquare::from(create_sliding_attacks_occluded_masks::<{ ROOK_OCCUPANCY_LIMIT as u32 }, false>());

    const PAWN_ATTACKS: ByPlayer<BySquare<Self>> = {
        let mut items: ByPlayer<BySquare<Self>> = ByPlayer::default();
        let mut square_offset = 0;
        while (square_offset as usize) < NUM_SQUARES {
            let square = Square::try_from(square_offset).ok().unwrap();
            let square_mask = square.to_mask();
            *items.mut_side(Player::White).mut_square(square) = square_mask.pawn_attacks(Player::White);
            *items.mut_side(Player::Black).mut_square(square) = square_mask.pawn_attacks(Player::Black);
            square_offset += 1;
        }

        items
    };

    /// Lookup for pawn attack masks
    pub const fn pawn_attacks_mask(square: Square, side: Player) -> Self {
        *Self::PAWN_ATTACKS.get_side(side).get_square(square)
    }

    /// Lookup for non-pawn un-occluded attack masks
    ///
    /// # Panics
    /// Does not support `piece: PieceType::Pawn`, use [`Bitboard::pawn_attacks_mask`](Self::pawn_attacks_mask)
    pub const fn attacks_mask(piece: PieceType, square: Square) -> Self {
        assert!(piece != PieceType::Pawn, "use pawn_attacks");
        *Self::PSEUDO_ATTACKS.get_piece(piece).get_square(square)
    }

    /// Lookup for non-pawn occluded attack masks
    ///
    /// # Panics
    /// Does not support `piece: PieceType::Pawn`, use [`Bitboard::pawn_attacks_mask`](Self::pawn_attacks_mask)
    pub const fn occluded_attacks_mask(piece: PieceType, square: Square, occupied: Self) -> Self {
        match piece {
            PieceType::Pawn => panic!("use pawn_attacks_mask"),
            PieceType::Knight | PieceType::King => Self::attacks_mask(piece, square),
            PieceType::Bishop => Self::BISHOP_OCCLUDED_ATTACKS.get_square(square).get_for(occupied),
            PieceType::Rook => Self::ROOK_OCCLUDED_ATTACKS.get_square(square).get_for(occupied),
            PieceType::Queen => Self::occluded_attacks_mask(PieceType::Rook, square, occupied) | Self::occluded_attacks_mask(PieceType::Bishop, square, occupied),
        }
    }

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
    use test_case::test_case;

    #[test_case(Bitboard(0x0020_0000_0000), Bitboard(0x0020_0000_0000), Bitboard(0x2020_2020))]
    #[test_case(Bitboard(0x0400_2000_0000), Bitboard(0x0400_2000_0000), Bitboard(0x0004_0424_2424))]
    #[test_case(Bitboard(0x0002_0004_0020_0000), Bitboard(0x0002_0204_0020_0420), Bitboard(0x0200_0404_2420))]
    #[test_case(Bitboard(0x0002_0004_0020_0000), Bitboard(0x0006_0004_0421_0020), Bitboard(0x0202_0602_2222))]
    fn south_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::South), expected);
    }

    #[test_case(Bitboard(0x400), Bitboard(0x400), Bitboard(0x0404_0404_0404_0000))]
    #[test_case(Bitboard(0x42000), Bitboard(0x42000), Bitboard(0x2424_2424_2420_0000))]
    #[test_case(Bitboard(0x0420_0100), Bitboard(0x0400_2000_0421_0100), Bitboard(0x0404_2424_2001_0000))]
    fn north_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::North), expected);
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::North), expected);
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::North), expected);
    }

    #[test_case(Bitboard(0x0010_0000_0000_0000), Bitboard(0x0010_0000_0000_0000), Bitboard(0x00E0_0000_0000_0000))]
    #[test_case(Bitboard(0x0004_0000_0008_0000), Bitboard(0x0004_0000_0008_0000), Bitboard(0x00F8_0000_00F0_0000))]
    #[test_case(Bitboard(0x0010_0800_0010_0000), Bitboard(0x0010_1800_0090_0000), Bitboard(0x00E0_1000_00E0_0000))]
    #[test_case(Bitboard(0x0020_0008_0000_0400), Bitboard(0x2020_0048_0000_1404), Bitboard(0x00C0_0070_0000_1800))]
    fn east_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::East), expected);
    }

    #[test_case(Bitboard(0x2000_0000_0000), Bitboard(0x2000_0000_0000), Bitboard(0x1F00_0000_0000))]
    #[test_case(Bitboard(0x0008_0000_0040_0000), Bitboard(0x0008_0000_0040_0000), Bitboard(0x0007_0000_003F_0000))]
    #[test_case(Bitboard(0x0800_0000_0020_2000), Bitboard(0x0900_0000_0030_2400), Bitboard(0x0700_0000_0010_1C00))]
    #[test_case(Bitboard(0x0020_0004_0020_0000), Bitboard(0x0020_1005_0028_0000), Bitboard(0x001F_0003_0018_0000))]
    fn west_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::West), expected);
    }

    #[test_case(Bitboard(0x1000_0000), Bitboard(0x1000_0000), Bitboard(0x0080_4020_0000_0000))]
    #[test_case(Bitboard(0x0002_0020_0000), Bitboard(0x0002_0020_0000), Bitboard(0x1008_0480_4000_0000))]
    #[test_case(Bitboard(0x0400_0004_2000), Bitboard(0x1000_0410_0004_2000), Bitboard(0x1008_0010_8840_0000))]
    #[test_case(Bitboard(0x0200_0800_0010), Bitboard(0x2600_0840_0030), Bitboard(0x0804_2010_0040_2000))]
    fn north_east_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::NorthEast), expected);
    }

    #[test_case(Bitboard(0x0010_0000_0000), Bitboard(0x0010_0000_0000), Bitboard(0x0804_0201))]
    #[test_case(Bitboard(0x0008_0000_0010_0000), Bitboard(0x0008_0000_0010_0000), Bitboard(0x0402_0100_0804))]
    #[test_case(Bitboard(0x0004_0020_0000_2000), Bitboard(0x0004_0020_0008_2010), Bitboard(0x0201_1008_0010))]
    #[test_case(Bitboard(0x0004_0000_8800_0000), Bitboard(0x0004_0400_8880_2200), Bitboard(0x0201_0044_2200))]
    fn south_west_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::SouthWest), expected);
    }

    #[test_case(Bitboard(0x0010_0000_0000), Bitboard(0x0010_0000_0000), Bitboard(0x2040_8000))]
    #[test_case(Bitboard(0x1002_0000_0000), Bitboard(0x1002_0000_0000), Bitboard(0x0020_4488_1020))]
    #[test_case(Bitboard(0x0020_0400_0400_0000), Bitboard(0x0020_0480_0420_0000), Bitboard(0x4088_1028_1020))]
    #[test_case(Bitboard(0x2200_0002_0000), Bitboard(0x2220_0026_2000), Bitboard(0x0044_8810_2408))]
    fn south_east_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(sliders, occupied, Direction::SouthEast), expected);
    }

    #[test_case(Bitboard(0x2000_0000_0000), Bitboard(0x2000_0000_0000), Bitboard(0x2020_DF20_2020_2020))]
    #[test_case(Bitboard(0x2000_0004_0000), Bitboard(0x2000_0004_0000), Bitboard(0x2424_DF24_24FB_2424))]
    #[test_case(Bitboard(0x2002_0400_0000), Bitboard(0x0022_200a_1400_0400), Bitboard(0x0426_DF2D_3B26_2622))]
    #[test_case(Bitboard(0x0040_0002_0010_0000), Bitboard(0x0048_400a_0130_0000), Bitboard(0x52BA_521D_122F_1212))]
    fn cardinal_sliding_attacks_works(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::cardinal_sliding_attacks(sliders, occupied), expected);
    }

    #[test_case(Bitboard(0x0800_0000_0000), Bitboard(0x0800_0000_0000), Bitboard(0x2214_0014_2241_8000))]
    #[test_case(Bitboard(0x0800_0040_0000), Bitboard(0x0800_0040_0000), Bitboard(0x2214_0814_A241_A010))]
    #[test_case(Bitboard(0x0420_0000_2000), Bitboard(0x0010_0420_1100_2020), Bitboard(0x158B_520E_D9D0_0050))]
    #[test_case(Bitboard(0x0010_0002_0000_0080), Bitboard(0x2010_0c06_01a8_0080), Bitboard(0x2800_2D40_8528_4000))]
    fn ordinal_sliding_attacks(sliders: Bitboard, occupied: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::ordinal_sliding_attacks(sliders, occupied), expected);
    }

    #[test_case(Bitboard(1), Bitboard(0x302u64))]
    #[test_case(Bitboard(0x0020_0000_0000_u64), Bitboard(0x7050_7000_0000_u64))]
    #[test_case(Bitboard(0x0080_0000_0000_0000_u64), Bitboard(0xC040_C000_0000_0000_u64))]
    fn king_attacks_works(knights: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::king_attacks(knights), expected);
    }

    #[test_case(Bitboard(0x0400_0000_0000_u64), Bitboard(0x0A11_0011_0A00_0000_u64))]
    #[test_case(Bitboard(0x0020_0000_0000_u64), Bitboard(0x0050_8800_8850_0000_u64))]
    #[test_case(Bitboard(0x80u64), Bitboard(0x0040_2000_u64))]
    fn knight_attacks_works(knights: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::knight_attacks(knights), expected);
    }

    #[test_case(Bitboard::EMPTY, Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard::EMPTY, Player::Black, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0010_0000), Player::White, Bitboard(0x0800_0000))]
    #[test_case(Bitboard(0x20000), Player::White, Bitboard(0x0100_0000))]
    #[test_case(Bitboard(0x0100_0000), Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0010_0000), Player::Black, Bitboard(0x800))]
    #[test_case(Bitboard(0x20000), Player::Black, Bitboard(0x100))]
    #[test_case(Bitboard(0x0100_0000), Player::Black, Bitboard::EMPTY)]
    #[test_case(Bitboard(0xFF00), Player::White, Bitboard(0x007F_0000))]
    fn pawn_west_attacks_works(pawns: Bitboard, side: Player, expected: Bitboard) {
        assert_eq!(Bitboard::pawn_west_attacks(pawns, side), expected);
    }

    #[test_case(Bitboard(0x00FF_0000_0000_0000), Player::Black, Bitboard(0x7F00_0000_0000))]
    #[test_case(Bitboard::EMPTY, Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard::EMPTY, Player::Black, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0010_0000), Player::White, Bitboard(0x2000_0000))]
    #[test_case(Bitboard(0x2000_0000), Player::White, Bitboard(0x0040_0000_0000))]
    #[test_case(Bitboard(0x8000_0000_0000), Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0010_0000), Player::Black, Bitboard(0x2000))]
    #[test_case(Bitboard(0x2000_0000), Player::Black, Bitboard(0x0040_0000))]
    #[test_case(Bitboard(0x8000_0000_0000), Player::Black, Bitboard::EMPTY)]
    fn pawn_east_attacks_works(pawns: Bitboard, side: Player, expected: Bitboard) {
        assert_eq!(Bitboard::pawn_east_attacks(pawns, side), expected);
    }

    #[test_case(Bitboard::EMPTY, Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard::EMPTY, Player::Black, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0800_0000), Player::White, Bitboard(0x0014_0000_0000))]
    #[test_case(Bitboard(0x0800_0000), Player::Black, Bitboard(0x0014_0000))]
    #[test_case(Bitboard(0x2010_0440_0000), Player::White, Bitboard(0x0050_280A_A000_0000))]
    #[test_case(Bitboard(0x2010_0440_0000), Player::Black, Bitboard(0x0050_280A_A000))]
    #[test_case(Bitboard(0xFF00), Player::White, Bitboard(0x00FF_0000))]
    fn pawn_attacks_works(pawns: Bitboard, side: Player, expected: Bitboard) {
        assert_eq!(Bitboard::pawn_attacks(pawns, side),expected);
    }

    #[test_case(Bitboard(0x00FF_0000_0000_0000), Player::Black, Bitboard(0xFF00_0000_0000))]
    #[test_case(Bitboard::EMPTY, Player::White, Bitboard::EMPTY)]
    #[test_case(Bitboard::EMPTY, Player::Black, Bitboard::EMPTY)]
    #[test_case(Bitboard(0x0800_0000), Player::White, Bitboard(0x0008_0000_0000))]
    #[test_case(Bitboard(0x0800_0000), Player::Black, Bitboard(0x80000))]
    #[test_case(Bitboard(0x2010_0440_0000), Player::White, Bitboard(0x0020_1004_4000_0000))]
    #[test_case(Bitboard(0x2010_0440_0000), Player::Black, Bitboard(0x0020_1004_4000))]
    fn pawn_pushes_works(pawns: Bitboard, side: Player, expected: Bitboard) {
        assert_eq!(Bitboard::pawn_push(pawns, side), expected);
    }
}