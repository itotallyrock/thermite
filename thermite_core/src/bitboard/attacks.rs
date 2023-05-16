use crate::bitboard::Bitboard;
use crate::bitboard::direction::Direction;
use crate::piece_type::{ByPieceType, PieceType};
use crate::player::{ByPlayer, Player};
use crate::square::{BySquare, NUM_SQUARES, Square};
use lazy_static::lazy_static;

mod magics {
    use lazy_static::lazy_static;
    use crate::bitboard::{Bitboard, BitboardInner};
    use crate::PieceCount;
    use crate::square::{BySquare, Square};

    /// Find the occluded bishop attack mask in the lookup table for a given origin square and occupied mask
    pub fn lookup_bishop_occluded_attacks(square: Square, occupied: Bitboard) -> Bitboard {
        BISHOP_OCCLUDED_ATTACKS.get_square(square).get_for(occupied)
    }

    /// Find the occluded rook attack mask in the lookup table for a given origin square and occupied mask
    pub fn lookup_rook_occluded_attacks(square: Square, occupied: Bitboard) -> Bitboard {
        ROOK_OCCLUDED_ATTACKS.get_square(square).get_for(occupied)
    }

    /// Maximum number of variations of individual blockers for a rook
    const ROOK_OCCUPANCY_LIMIT: u32 = 12;
    /// Maximum number of variations of blockers for a bishop
    const BISHOP_OCCUPANCY_LIMIT: u32 = 9;

lazy_static! {
    /// The lookup table for [bishop](crate::piece_type::PieceType::Bishop) occluded [`MagicLookup`] attack [masks](Bitboard)
    static ref BISHOP_OCCLUDED_ATTACKS: Box<BySquare<MagicLookup<BISHOP_OCCUPANCY_LIMIT>>> = create_sliding_lookup::<BISHOP_OCCUPANCY_LIMIT>(false);
    /// The lookup table for [rook](crate::piece_type::PieceType::Rook) occluded [`MagicLookup`] attack [masks](Bitboard)
    static ref ROOK_OCCLUDED_ATTACKS: Box<BySquare<MagicLookup<ROOK_OCCUPANCY_LIMIT>>> = create_sliding_lookup::<ROOK_OCCUPANCY_LIMIT>(true);

    /// Maximum number of blocker [square](Square)s (or the number of [piece](crate::piece_type::PieceType)s that can be along the cardinals) for a [rook](crate::piece_type::PieceType::Rook) on a given [square](Square)
    ///
    /// For example: on [A1](Square::A1) count all the squares on the vertical file from [A2](Square::A2)-[A7](Square::A7) (6) and the horizontal rank from [B1](Square::B1)-[G1](Square::G1) (6) which total to 12
    #[rustfmt::skip]
    static ref ROOK_BLOCKER_COUNTS: BySquare<PieceCount> = BySquare::from([
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12,
    ]);

    /// Maximum number of blocker [square](Square)s (or the number of [piece](crate::piece_type::PieceType)s that can be along the diagonals) for a [bishop](crate::piece_type::PieceType::Bishop) on a given [square](Square)
    #[rustfmt::skip]
    static ref BISHOP_BLOCKER_COUNTS: BySquare<PieceCount> = BySquare::from([
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6,
    ]);

    /// [Mask](Bitboard) of relevant squares that could block a [bishop](crate::piece_type::PieceType::Bishop) on a given [square](Square)
    #[rustfmt::skip]
    #[allow(clippy::unreadable_literal)]
    static ref BISHOP_OCCUPANCY_MASK: BySquare<Bitboard> = BySquare::from([
        Bitboard(0x40201008040200), Bitboard(0x402010080400),   Bitboard(0x4020100A00),     Bitboard(0x40221400),       Bitboard(0x2442800),        Bitboard(0x204085000),      Bitboard(0x20408102000),    Bitboard(0x2040810204000),
        Bitboard(0x20100804020000), Bitboard(0x40201008040000), Bitboard(0x4020100A0000),   Bitboard(0x4022140000),     Bitboard(0x244280000),      Bitboard(0x20408500000),    Bitboard(0x2040810200000),  Bitboard(0x4081020400000),
        Bitboard(0x10080402000200), Bitboard(0x20100804000400), Bitboard(0x4020100A000A00), Bitboard(0x402214001400),   Bitboard(0x24428002800),    Bitboard(0x2040850005000),  Bitboard(0x4081020002000),  Bitboard(0x8102040004000),
        Bitboard(0x8040200020400),  Bitboard(0x10080400040800), Bitboard(0x20100A000A1000), Bitboard(0x40221400142200), Bitboard(0x2442800284400),  Bitboard(0x4085000500800),  Bitboard(0x8102000201000),  Bitboard(0x10204000402000),
        Bitboard(0x4020002040800),  Bitboard(0x8040004081000),  Bitboard(0x100A000A102000), Bitboard(0x22140014224000), Bitboard(0x44280028440200), Bitboard(0x8500050080400),  Bitboard(0x10200020100800), Bitboard(0x20400040201000),
        Bitboard(0x2000204081000),  Bitboard(0x4000408102000),  Bitboard(0xA000A10204000),  Bitboard(0x14001422400000), Bitboard(0x28002844020000), Bitboard(0x50005008040200), Bitboard(0x20002010080400), Bitboard(0x40004020100800),
        Bitboard(0x20408102000),    Bitboard(0x40810204000),    Bitboard(0xA1020400000),    Bitboard(0x142240000000),   Bitboard(0x284402000000),   Bitboard(0x500804020000),   Bitboard(0x201008040200),   Bitboard(0x402010080400),
        Bitboard(0x2040810204000),  Bitboard(0x4081020400000),  Bitboard(0xA102040000000),  Bitboard(0x14224000000000), Bitboard(0x28440200000000), Bitboard(0x50080402000000), Bitboard(0x20100804020000), Bitboard(0x40201008040200),
    ]);

    /// [Mask](Bitboard) of relevant squares that could block a [rook](crate::piece_type::PieceType::Rook) on a given [square](Square)
    #[rustfmt::skip]
    #[allow(clippy::unreadable_literal)]
    static ref ROOK_OCCUPANCY_MASK: BySquare<Bitboard> = BySquare::from([
        Bitboard(0x101010101017E),    Bitboard(0x202020202027C),    Bitboard(0x404040404047A),    Bitboard(0x8080808080876),    Bitboard(0x1010101010106E),   Bitboard(0x2020202020205E),   Bitboard(0x4040404040403E),   Bitboard(0x8080808080807E),
        Bitboard(0x1010101017E00),    Bitboard(0x2020202027C00),    Bitboard(0x4040404047A00),    Bitboard(0x8080808087600),    Bitboard(0x10101010106E00),   Bitboard(0x20202020205E00),   Bitboard(0x40404040403E00),   Bitboard(0x80808080807E00),
        Bitboard(0x10101017E0100),    Bitboard(0x20202027C0200),    Bitboard(0x40404047A0400),    Bitboard(0x8080808760800),    Bitboard(0x101010106E1000),   Bitboard(0x202020205E2000),   Bitboard(0x404040403E4000),   Bitboard(0x808080807E8000),
        Bitboard(0x101017E010100),    Bitboard(0x202027C020200),    Bitboard(0x404047A040400),    Bitboard(0x8080876080800),    Bitboard(0x1010106E101000),   Bitboard(0x2020205E202000),   Bitboard(0x4040403E404000),   Bitboard(0x8080807E808000),
        Bitboard(0x1017E01010100),    Bitboard(0x2027C02020200),    Bitboard(0x4047A04040400),    Bitboard(0x8087608080800),    Bitboard(0x10106E10101000),   Bitboard(0x20205E20202000),   Bitboard(0x40403E40404000),   Bitboard(0x80807E80808000),
        Bitboard(0x17E0101010100),    Bitboard(0x27C0202020200),    Bitboard(0x47A0404040400),    Bitboard(0x8760808080800),    Bitboard(0x106E1010101000),   Bitboard(0x205E2020202000),   Bitboard(0x403E4040404000),   Bitboard(0x807E8080808000),
        Bitboard(0x7E010101010100),   Bitboard(0x7C020202020200),   Bitboard(0x7A040404040400),   Bitboard(0x76080808080800),   Bitboard(0x6E101010101000),   Bitboard(0x5E202020202000),   Bitboard(0x3E404040404000),   Bitboard(0x7E808080808000),
        Bitboard(0x7E01010101010100), Bitboard(0x7C02020202020200), Bitboard(0x7A04040404040400), Bitboard(0x7608080808080800), Bitboard(0x6E10101010101000), Bitboard(0x5E20202020202000), Bitboard(0x3E40404040404000), Bitboard(0x7E80808080808000),
    ]);
}

    /// A [`Bitboard`] lookup for a specific [`Square`] and board occupancy mask for sliding attack lookups
    #[derive(Copy, Clone, Debug)]
    struct MagicLookup<const OCCUPANCY_LIMIT: u32> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
        occupancy_mask: Bitboard,
        attacks: [Bitboard; 1 << OCCUPANCY_LIMIT],
    }

    impl<const OCCUPANCY_LIMIT: u32> Default for MagicLookup<OCCUPANCY_LIMIT> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
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
        /// Lookup the attacks [mask](Bitboard) for a given occupancy [mask](Bitboard)
        const fn get_for(&self, occupied: Bitboard) -> Bitboard {
            self.attacks[index_from_board_mask(occupied, self.occupancy_mask)]
        }
    }

    /// Get the maximum [number of blockers](PieceCount) for a piece on a [square](Square) that attacks in a given direction (ordinal or diagonal)
    fn get_max_blockers(is_cardinal: bool, piece_square: Square) -> PieceCount {
        let directional_blocker_count_table = if is_cardinal { &*ROOK_BLOCKER_COUNTS } else { &*BISHOP_BLOCKER_COUNTS };

        *directional_blocker_count_table.get_square(piece_square)
    }

    /// Get the occupancy [mask](Bitboard) for a piece on a [square](Square) that attacks in a given direction (ordinal or diagonal)
    fn get_occupancy_mask(is_cardinal: bool, piece_square: Square) -> Bitboard {
        let directional_occupancy_table = if is_cardinal { &*ROOK_OCCUPANCY_MASK } else { &*BISHOP_OCCUPANCY_MASK };

        *directional_occupancy_table.get_square(piece_square)
    }

    /// Generate a [magic lookup](MagicLookup) given an occupancy mask capacity [for each square](BySquare) given a direction.
    fn create_sliding_lookup<const OCCUPANCY_LIMIT: u32>(is_cardinal: bool) -> Box<BySquare<MagicLookup<OCCUPANCY_LIMIT>>> where [(); 1 << OCCUPANCY_LIMIT]: Sized {
        let mut attacks_by_square: Box<BySquare<MagicLookup<OCCUPANCY_LIMIT>>> = Box::default();

        for piece_square in Square::SQUARES {
            #[allow(clippy::cast_possible_truncation)]
            let piece_mask = piece_square.to_mask();
            let occupancy_mask = get_occupancy_mask(is_cardinal, piece_square);
            attacks_by_square.mut_square(piece_square).occupancy_mask = occupancy_mask;
            let max_blocker_combinations = 1 << get_max_blockers(is_cardinal, piece_square);
            for blocker_index in 0..max_blocker_combinations {
                let blocker_mask = board_mask_from_index(blocker_index, occupancy_mask) | piece_mask;
                let attacks = if is_cardinal {
                    piece_mask.cardinal_sliding_attacks(blocker_mask)
                } else {
                    piece_mask.ordinal_sliding_attacks(blocker_mask)
                };

                attacks_by_square.mut_square(piece_square).attacks[blocker_index] = attacks;
            }
        }

        attacks_by_square
    }

    #[cfg(test)]
    mod test {
        use test_case::test_case;

        use crate::square::{NUM_FILES, NUM_RANKS};

        use super::*;

        #[test_case(Square::A1, true, Bitboard(0x0001_0101_0101_017E))]
        #[test_case(Square::A1, false, Bitboard(0x0040_2010_0804_0200))]
        #[test_case(Square::C4, true, Bitboard(0x0004_0404_7a04_0400))]
        #[test_case(Square::C4, false, Bitboard(0x0020_100A_000A_1000))]
        #[test_case(Square::B4, true, Bitboard(0x0002_0202_7C02_0200))]
        #[test_case(Square::B4, false, Bitboard(0x0010_0804_0004_0800))]
        #[test_case(Square::H1, true, Bitboard(0x0080_8080_8080_807E))]
        #[test_case(Square::H1, false, Bitboard(0x0002_0408_1020_4000))]
        #[test_case(Square::C7, true, Bitboard(0x007A_0404_0404_0400))]
        #[test_case(Square::C7, false, Bitboard(0x0A10_2040_0000))]
        fn get_occupancy_mask_works(square: Square, cardinal: bool, expected: Bitboard) {
            let occupancy_mask = if cardinal { get_occupancy_mask(true, square) } else { get_occupancy_mask(false, square) };
            assert_eq!(occupancy_mask, expected);
        }

        #[test_case(&*BISHOP_OCCUPANCY_MASK, &*BISHOP_BLOCKER_COUNTS)]
        #[test_case(&*ROOK_OCCUPANCY_MASK, &*ROOK_BLOCKER_COUNTS)]
        fn board_mask_index_commutative_for_all_square_indices(occupancy_masks: &BySquare<Bitboard>, blocker_counts: &BySquare<PieceCount>) {
            for square in Square::SQUARES {
                let occupancy_mask: Bitboard = *occupancy_masks.get_square(square);
                let max_blockers = 1 << *blocker_counts.get_square(square);
                for index in 0..max_blockers {
                    assert_eq!(index_from_board_mask(board_mask_from_index(index, occupancy_mask), occupancy_mask), index, "{square} ({occupancy_mask:#?}) index: {index}");
                }
            }
        }

        #[test]
        fn bishop_occupancy_mask_contains_no_edges() {
            const EDGES: Bitboard = Bitboard(0xFF81_8181_8181_81FF);
            assert_eq!(EDGES, Bitboard::RANKS[0] | Bitboard::RANKS[NUM_RANKS - 1] | Bitboard::FILES[0] | Bitboard::FILES[NUM_FILES - 1]);

            for square in Square::SQUARES {
                let occupancy_mask = *BISHOP_OCCUPANCY_MASK.get_square(square);
                assert_eq!(occupancy_mask & EDGES, Bitboard::EMPTY, "{square} ({occupancy_mask:#?}) contains edges");
            }
        }
    }
}

lazy_static! {
    static ref PSEUDO_ATTACKS: ByPieceType<BySquare<Bitboard>> = {
        let mut items: ByPieceType<BySquare<Bitboard>> = ByPieceType::new(BySquare::new(Bitboard::EMPTY));

        let mut square_offset = 0;
        while square_offset < NUM_SQUARES {
            let square = Square::SQUARES[square_offset];
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

    static ref PAWN_ATTACKS: ByPlayer<BySquare<Bitboard>> = {
        let mut items: ByPlayer<BySquare<Bitboard>> = ByPlayer::new(BySquare::new(Bitboard::EMPTY));
        let mut square_offset = 0;
        while square_offset < NUM_SQUARES {
            let square = Square::SQUARES[square_offset];
            let square_mask = square.to_mask();
            *items.mut_side(Player::White).mut_square(square) = square_mask.pawn_attacks(Player::White);
            *items.mut_side(Player::Black).mut_square(square) = square_mask.pawn_attacks(Player::Black);
            square_offset += 1;
        }

        items
    };
}

impl Bitboard {
    /// Lookup for pawn attack masks
    pub fn pawn_attacks_mask(square: Square, side: Player) -> Self {
        *PAWN_ATTACKS.get_side(side).get_square(square)
    }

    /// Lookup for non-pawn un-occluded attack masks
    ///
    /// # Panics
    /// Does not support `piece: PieceType::Pawn`, use [`Bitboard::pawn_attacks_mask`](Self::pawn_attacks_mask)
    pub fn attacks_mask(piece: PieceType, square: Square) -> Self {
        assert_ne!(piece, PieceType::Pawn, "use pawn_attacks");
        *PSEUDO_ATTACKS.get_piece(piece).get_square(square)
    }

    /// Lookup for non-pawn occluded attack masks
    ///
    /// # Panics
    /// Does not support `piece: PieceType::Pawn`, use [`Bitboard::pawn_attacks_mask`](Self::pawn_attacks_mask)
    pub fn occluded_attacks_mask(piece: PieceType, square: Square, occupied: Self) -> Self {
        match piece {
            PieceType::Pawn => panic!("use pawn_attacks_mask"),
            PieceType::Knight | PieceType::King => Self::attacks_mask(piece, square),
            PieceType::Bishop => magics::lookup_bishop_occluded_attacks(square, occupied),
            PieceType::Rook => magics::lookup_rook_occluded_attacks(square, occupied),
            PieceType::Queen => Self::occluded_attacks_mask(PieceType::Rook, square, occupied) | Self::occluded_attacks_mask(PieceType::Bishop, square, occupied),
        }
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
    pub fn pawn_west_attacks(self, side: Player) -> Self {
        let west_attack_direction = match side {
            Player::White => Direction::NorthWest,
            Player::Black => Direction::SouthWest,
        };

        self.shift(west_attack_direction)
    }

    /// Calculate the pawn east attacks mask for a given mask of pawn attacker(s)
    pub fn pawn_east_attacks(self, side: Player) -> Self {
        let west_attack_direction = match side {
            Player::White => Direction::NorthEast,
            Player::Black => Direction::SouthEast,
        };

        self.shift(west_attack_direction)
    }

    /// Calculate the pawn attacks mask for a given mask of pawn attacker(s)
    pub fn pawn_attacks(self, side: Player) -> Self {
        self.pawn_east_attacks(side) | self.pawn_west_attacks(side)
    }

    /// Calculate the single pawn push mask for a given mask of pawns
    pub fn pawn_push(self, side: Player) -> Self {
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

    fn occluded_fill(mut self, occupied: Self, direction: Direction) -> Self {
        let mut empty = !occupied;
        let mut flood = Self::EMPTY;
        if self != Self::EMPTY {
            let direction_shift = direction as i32;
            empty &= Self::get_sliding_mask(direction);
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
    use test_case::test_case;

    use super::*;

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

    #[test_case(Bitboard(0x1), Bitboard(0x1), Bitboard(0x0101_0101_0101_01FE))]
    #[test_case(Bitboard(0x80), Bitboard(0x80), Bitboard(0x8080_8080_8080_807F))]
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
    #[test_case(Bitboard(0x00FF_0000_0000_0000), Player::Black, Bitboard(0x7F00_0000_0000))]
    fn pawn_west_attacks_works(pawns: Bitboard, side: Player, expected: Bitboard) {
        assert_eq!(Bitboard::pawn_west_attacks(pawns, side), expected);
    }

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

    #[test_case(PieceType::Bishop, Square::A1, Bitboard(0x8040_2010_0804_0200))]
    #[test_case(PieceType::Bishop, Square::H8, Bitboard(0x0040_2010_0804_0201))]
    #[test_case(PieceType::Rook, Square::A8, Bitboard(0xFE01_0101_0101_0101))]
    #[test_case(PieceType::Rook, Square::H8, Bitboard(0x7F80_8080_8080_8080))]
    #[test_case(PieceType::Rook, Square::A1, Bitboard(0x0101_0101_0101_01FE))]
    #[test_case(PieceType::Queen, Square::A1, Bitboard(0x8141_2111_0905_03FE))]
    #[test_case(PieceType::Queen, Square::H1, Bitboard(0x8182_8488_90A0_C07F))]
    #[test_case(PieceType::Rook, Square::H1, Bitboard(0x8080_8080_8080_807F))]
    #[test_case(PieceType::Rook, Square::F4, Bitboard(0x2020_2020_DF20_2020))]
    #[test_case(PieceType::Bishop, Square::D6, Bitboard(0x2214_0014_2241_8000))]
    #[test_case(PieceType::Knight, Square::E4, Bitboard(0x2844_0044_2800))]
    #[test_case(PieceType::Knight, Square::F3, Bitboard(0x0050_8800_8850))]
    #[test_case(PieceType::Knight, Square::C8, Bitboard(0x0011_0A00_0000_0000))]
    #[test_case(PieceType::Knight, Square::E6, Bitboard(0x2844_0044_2800_0000))]
    #[test_case(PieceType::King, Square::D3, Bitboard(0x1C14_1C00))]
    #[test_case(PieceType::King, Square::H1, Bitboard(0xC040))]
    #[test_case(PieceType::King, Square::D6, Bitboard(0x001C_141C_0000_0000))]
    #[test_case(PieceType::King, Square::A5, Bitboard(0x0302_0300_0000))]
    #[test_case(PieceType::Queen, Square::C4, Bitboard(0x4424_150E_FB0E_1524))]
    #[test_case(PieceType::Queen, Square::H3, Bitboard(0x8488_90A0_C07F_C0A0))]
    #[test_case(PieceType::Queen, Square::B5, Bitboard(0x120A_07FD_070A_1222))]
    fn attacks_mask_works(piece: PieceType, square: Square, expected: Bitboard) {
        assert_eq!(Bitboard::attacks_mask(piece, square), expected);
    }

    #[test_case(PieceType::Rook, Square::A1, Bitboard::EMPTY, Bitboard(0x0101_0101_0101_01FE))]
    #[test_case(PieceType::Rook, Square::H1, Bitboard::EMPTY, Bitboard(0x8080_8080_8080_807F))]
    #[test_case(PieceType::Rook, Square::A1, Bitboard(0xFFFF), Bitboard(0x102); "startpos white queenside Rook")]
    #[test_case(PieceType::Rook, Square::H1, Bitboard(0xFFFF), Bitboard(0x8040); "startpos white kingside Rook")]
    #[test_case(PieceType::Bishop, Square::A1, Bitboard::EMPTY, Bitboard(0x8040_2010_0804_0200))]
    #[test_case(PieceType::Rook, Square::B4, Bitboard(0x2200_3300_0802), Bitboard(0x0202_1D02_0202))]
    #[test_case(PieceType::Bishop, Square::D4, Bitboard::FULL, Bitboard(0x0014_0014_0000); "1 Bishop, all blocking")]
    #[test_case(PieceType::Bishop, Square::F6, Bitboard(0x2000_0000_0000), Bitboard(0x8850_0050_8804_0201); "1 Bishop, no blocking a")]
    #[test_case(PieceType::Bishop, Square::H1, Bitboard(0x80), Bitboard(0x0102_0408_1020_4000); "1 Bishop, no blocking b")]
    #[test_case(PieceType::Bishop, Square::C4, Bitboard(0x0020_0140_0402_4004), Bitboard(0x0020_110A_000A_1020); "1 attacker multiple blocking")]
    #[test_case(PieceType::Bishop, Square::G6, Bitboard(0x4000_F800_0000), Bitboard(0x10A0_00A0_1000_0000))]
    #[test_case(PieceType::Rook, Square::D4, Bitboard::FULL, Bitboard(0x0008_1408_0000); "1 Rook, all blocking")]
    #[test_case(PieceType::Rook, Square::F6, Bitboard(0x2000_0000_0000), Bitboard(0x2020_DF20_2020_2020); "1 Rook, no blocking a")]
    #[test_case(PieceType::Rook, Square::H1, Bitboard(0x80), Bitboard(0x8080_8080_8080_807F); "1 Rook, no blocking b")]
    #[test_case(PieceType::Rook, Square::C4, Bitboard(0x0004_2500_1000), Bitboard(0x0004_3B04_0404); "1 attacker multiple blocking a")]
    #[test_case(PieceType::Rook, Square::G6, Bitboard(0x4000_F800_0000), Bitboard(0x4040_BF40_4000_0000); "1 Rook, multiple blocking b")]
    #[test_case(PieceType::Queen, Square::D4, Bitboard::FULL, Bitboard(0x001C_141C_0000); "1 Queen, all blocking")]
    #[test_case(PieceType::Queen, Square::D4, Bitboard(0x2000_0000_0000), Bitboard(0x0809_2A1C_F71C_2A49); "1 Queen, no blocking a")]
    #[test_case(PieceType::Queen, Square::H1, Bitboard(0x80), Bitboard(0x8182_8488_90A0_C07F); "1 Queen, no blocking b")]
    #[test_case(PieceType::Queen, Square::F3, Bitboard(0x0038_0062_2000), Bitboard(0x00A8_705E_7088); "1 attacker multiple blocking c")]
    #[test_case(PieceType::Queen, Square::G6, Bitboard(0x4000_F800_0000), Bitboard(0x50E0_BFE0_5000_0000); "1 Queen, multiple blocking b")]
    fn occluded_attacks_mask_works(piece: PieceType, square: Square, occluded: Bitboard, expected: Bitboard) {
        assert_eq!(Bitboard::occluded_attacks_mask(piece, square, occluded), expected);
    }

    #[test_case(Square::A1.to_mask(), Bitboard::EMPTY, Direction::NorthEast, Bitboard(0x8040_2010_0804_0200))]
    #[test_case(Square::A1.to_mask(), Bitboard(0xffff), Direction::North, Bitboard(0x100))]
    #[test_case(Square::H1.to_mask(), Bitboard(0xffff), Direction::North, Bitboard(0x8000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::North, Bitboard(0x1010_1010_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::South, Bitboard(0x0010_1010))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::East, Bitboard(0xE000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::West, Bitboard(0x0F00_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::NorthEast, Bitboard(0x0080_4020_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::SouthWest, Bitboard(0x80402))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::NorthWest, Bitboard(0x0102_0408_0000_0000))]
    #[test_case(Square::E4.to_mask(), Square::E4.to_mask(), Direction::SouthEast, Bitboard(0x0020_4080))]
    fn sliding_attacks_works(mask: Bitboard, occluded: Bitboard, direction: Direction, expected: Bitboard) {
        assert_eq!(Bitboard::sliding_attacks(mask, occluded, direction), expected);
    }
}