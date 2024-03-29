use crate::bitboard::BoardMask;
use crate::chess_move::castle::Castle;
use crate::player_color::PlayerColor;
use crate::square::Square;
use bitmask_enum::bitmask;
use core::str::FromStr;
use enum_iterator::Sequence;
use enum_map::{Enum, EnumMap};

/// The square that the king for a given side moves from when castling in a given direction
pub const KING_FROM_SQUARES: EnumMap<PlayerColor, Square> =
    EnumMap::from_array([Square::E1, Square::E8]);

/// The square that the king for a given side moves to when castling in a given direction
pub const KING_TO_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::G1, Square::G8]),
        EnumMap::from_array([Square::C1, Square::C8]),
    ]);

/// The square that the rook for a given side moves from when castling in a given direction
pub const ROOK_FROM_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::H1, Square::H8]),
        EnumMap::from_array([Square::A1, Square::A8]),
    ]);

/// The square that the rook for a given side moves to when castling in a given direction
pub const ROOK_TO_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::F1, Square::F8]),
        EnumMap::from_array([Square::D1, Square::D8]),
    ]);

/// A mask of the squares that must be empty to castle
pub const UNOCCUPIED_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, BoardMask>> =
    EnumMap::from_array([
        EnumMap::from_array([BoardMask(0x60), BoardMask(0x6000_0000_0000_0000)]),
        EnumMap::from_array([BoardMask(0xe), BoardMask(0x0e00_0000_0000_0000)]),
    ]);

/// A mask of the squares that cannot be attacked to castle
pub const UNATTACKED_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, BoardMask>> =
    EnumMap::from_array([
        EnumMap::from_array([BoardMask(0x60), BoardMask(0x6000_0000_0000_0000)]),
        EnumMap::from_array([BoardMask(0xc), BoardMask(0x0c00_0000_0000_0000)]),
    ]);

/// The direction to castle in for either side
#[derive(Enum, Sequence, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CastleDirection {
    /// Castle with the rook on the same side as the king
    KingSide,
    /// Castle with the rook on the same side as the queen
    QueenSide,
}

/// Keeps track of available castle abilities (king-side or queen-side castle) for both sides.
#[bitmask(u8)]
pub enum CastleRights {
    /// No castle abilities for either side
    None = 0,
    /// White's king-side castle ability
    WhiteKing = 1 << 0,
    /// White's queen-side castle ability
    WhiteQueen = 1 << 1,
    /// White can castle on both king and queen side
    WhiteBoth = Self::WhiteKing.bits | Self::WhiteQueen.bits,
    /// Black's king-side castle ability
    BlackKing = 1 << 2,
    /// Both sides' king-side castle abilities
    BothKings = Self::WhiteKing.bits | Self::BlackKing.bits,
    /// White queen-side and black king-side castle abilities
    WhiteQueenBlackKing = Self::WhiteQueen.bits | Self::BlackKing.bits,
    /// White queen & king-side as well as black's king-side castle abilities
    WhiteBothBlackKing = Self::WhiteBoth.bits | Self::BlackKing.bits,
    /// Black's queen-side castle ability
    BlackQueen = 1 << 3,
    /// White's king-side and black's queen-side castle abilities
    WhiteKingBlackQueen = Self::WhiteKing.bits | Self::BlackQueen.bits,
    /// Both sides' queen-side castle abilities
    BothQueens = Self::WhiteQueen.bits | Self::BlackQueen.bits,
    /// White's king & queen-side as well as black's queen-side castle abilities
    WhiteBothBlackQueen = Self::WhiteBoth.bits | Self::BlackQueen.bits,
    /// Black can castle on both king and queen side
    BlackBoth = Self::BlackKing.bits | Self::BlackQueen.bits,
    /// White's king-side and both black's king & queen-side castle abilities
    WhiteKingBlackBoth = Self::WhiteKing.bits | Self::BlackBoth.bits,
    /// White's queen-side and both black's king & queen-side castle abilities
    WhiteQueenBlackBoth = Self::WhiteQueen.bits | Self::BlackBoth.bits,
    /// If both players can castle in all directions
    All = Self::WhiteBoth.bits | Self::BlackBoth.bits,
}

impl CastleRights {
    /// Get the rights for a specific side
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    /// use thermite_core::player_color::PlayerColor;
    ///
    /// assert_eq!(CastleRights::for_player(PlayerColor::White), CastleRights::WhiteBoth);
    /// assert_eq!(CastleRights::for_player(PlayerColor::Black), CastleRights::BlackBoth);
    /// ```
    #[must_use]
    pub const fn for_player(side: PlayerColor) -> Self {
        match side {
            PlayerColor::White => Self::WhiteBoth,
            PlayerColor::Black => Self::BlackBoth,
        }
    }

    /// If the castle rights specify the ability for a side to castle in a given direction.
    ///
    /// Combination `CastleRights` such as `None` or `All` are supported as well.
    #[must_use]
    pub fn can_castle(&self, castle: Castle) -> bool {
        self.has_rights(castle.required_rights())
    }

    /// If one set of castle rights intersects with another
    #[must_use]
    pub fn has_rights(&self, other: Self) -> bool {
        other & *self != Self::None
    }

    /// Clear the available rights (if any) for a given [player](PlayerColor)
    ///
    /// ```
    /// use thermite_core::castles::CastleRights;
    /// use thermite_core::player_color::PlayerColor;
    ///
    /// let mut rights = CastleRights::All;
    ///
    /// rights.remove_rights_for(PlayerColor::White);
    /// assert_eq!(rights, CastleRights::BlackBoth);
    ///
    /// rights.remove_rights_for(PlayerColor::Black);
    /// assert_eq!(rights, CastleRights::None);
    /// ```
    pub fn remove_rights_for(&mut self, player: PlayerColor) {
        self.remove_rights(Self::for_player(player));
    }

    /// Remove a set of [rights](CastleRights) from another
    pub fn remove_rights(&mut self, other: Self) {
        *self &= !other;
    }
}

/// Invalid string input value was provided to `CastleRights::parse`.
/// Wasn't a valid combination of the 4 castle abilities or none.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IllegalCastleRights;

impl FromStr for CastleRights {
    type Err = IllegalCastleRights;

    /// Attempt to parse a UCI string into a [`CastleRight`](CastleRights).
    ///
    /// ```
    /// use std::str::FromStr;
    /// use thermite_core::castles::{CastleRights, IllegalCastleRights};
    ///
    /// assert_eq!(CastleRights::from_str("KQkq"), Ok(CastleRights::All));
    /// assert_eq!(CastleRights::from_str("KQ"), Ok(CastleRights::WhiteBoth));
    /// assert_eq!(CastleRights::from_str("q"), Ok(CastleRights::BlackQueen));
    /// assert_eq!(CastleRights::from_str("-"), Ok(CastleRights::None));
    /// assert_eq!(CastleRights::from_str("32"), Err(IllegalCastleRights));
    /// ```
    ///
    /// # Errors
    /// Will error if input is not a valid UCI castle right.
    /// Must be a combination of `'K'`, `'Q'`, `'k'`, and `'q'` or `'-'`.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "-" => Self::None,
            "K" => Self::WhiteKing,
            "Q" => Self::WhiteQueen,
            "KQ" => Self::WhiteBoth,
            "k" => Self::BlackKing,
            "Kk" => Self::BothKings,
            "Qk" => Self::WhiteQueenBlackKing,
            "KQk" => Self::WhiteBothBlackKing,
            "q" => Self::BlackQueen,
            "Kq" => Self::WhiteKingBlackQueen,
            "Qq" => Self::BothQueens,
            "KQq" => Self::WhiteBothBlackQueen,
            "kq" => Self::BlackBoth,
            "Kkq" => Self::WhiteKingBlackBoth,
            "Qkq" => Self::WhiteQueenBlackBoth,
            "KQkq" => Self::All,
            _ => return Err(IllegalCastleRights),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(PlayerColor::White, Square::E1)]
    #[test_case(PlayerColor::Black, Square::E8)]
    fn king_from_squares_are_correct(player: PlayerColor, expected: Square) {
        assert_eq!(KING_FROM_SQUARES[player], expected);
    }

    #[test_case(CastleDirection::KingSide, PlayerColor::White, Square::G1)]
    #[test_case(CastleDirection::KingSide, PlayerColor::Black, Square::G8)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::White, Square::C1)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::Black, Square::C8)]
    fn king_to_squares_are_correct(
        direction: CastleDirection,
        player: PlayerColor,
        expected: Square,
    ) {
        assert_eq!(KING_TO_SQUARES[direction][player], expected);
    }

    #[test_case(CastleDirection::KingSide, PlayerColor::White, Square::H1)]
    #[test_case(CastleDirection::KingSide, PlayerColor::Black, Square::H8)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::White, Square::A1)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::Black, Square::A8)]
    fn rook_from_squares_are_correct(
        direction: CastleDirection,
        player: PlayerColor,
        expected: Square,
    ) {
        assert_eq!(ROOK_FROM_SQUARES[direction][player], expected);
    }

    #[test_case(CastleDirection::KingSide, PlayerColor::White, Square::F1)]
    #[test_case(CastleDirection::KingSide, PlayerColor::Black, Square::F8)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::White, Square::D1)]
    #[test_case(CastleDirection::QueenSide, PlayerColor::Black, Square::D8)]
    fn rook_to_squares_are_correct(
        direction: CastleDirection,
        player: PlayerColor,
        expected: Square,
    ) {
        assert_eq!(ROOK_TO_SQUARES[direction][player], expected);
    }

    #[test]
    fn castles_bitor_works() {
        assert_eq!(
            CastleRights::WhiteQueen.or(CastleRights::WhiteKing),
            CastleRights::WhiteBoth
        );
        assert_eq!(
            CastleRights::WhiteBoth.or(CastleRights::BlackBoth),
            CastleRights::All
        );
        assert_eq!(
            CastleRights::None.or(CastleRights::BlackBoth),
            CastleRights::BlackBoth
        );
        assert_eq!(CastleRights::None.or(CastleRights::All), CastleRights::All);
        assert_eq!(
            CastleRights::None.or(CastleRights::None),
            CastleRights::None
        );
    }

    #[test_case(CastleRights::All, CastleRights::None)]
    #[test_case(CastleRights::WhiteBoth, CastleRights::BlackBoth)]
    #[test_case(CastleRights::BothKings, CastleRights::BothQueens)]
    #[test_case(CastleRights::WhiteBothBlackQueen, CastleRights::BlackKing)]
    #[test_case(CastleRights::WhiteKingBlackQueen, CastleRights::WhiteQueenBlackKing)]
    fn castles_not_works(input: CastleRights, expected: CastleRights) {
        assert_eq!(input.not().and(CastleRights::All), expected);
        assert_eq!(expected.not().and(CastleRights::All), input);
    }

    #[test_case(CastleRights::None, 0b0000)]
    #[test_case(CastleRights::WhiteKing, 0b0001)]
    #[test_case(CastleRights::WhiteQueen, 0b0010)]
    #[test_case(CastleRights::WhiteBoth, 0b0011)]
    #[test_case(CastleRights::BlackKing, 0b0100)]
    #[test_case(CastleRights::BothKings, 0b0101)]
    #[test_case(CastleRights::WhiteQueenBlackKing, 0b0110)]
    #[test_case(CastleRights::WhiteBothBlackKing, 0b0111)]
    #[test_case(CastleRights::BlackQueen, 0b1000)]
    #[test_case(CastleRights::WhiteKingBlackQueen, 0b1001)]
    #[test_case(CastleRights::BothQueens, 0b1010)]
    #[test_case(CastleRights::WhiteBothBlackQueen, 0b1011)]
    #[test_case(CastleRights::BlackBoth, 0b1100)]
    #[test_case(CastleRights::WhiteKingBlackBoth, 0b1101)]
    #[test_case(CastleRights::WhiteQueenBlackBoth, 0b1110)]
    #[test_case(CastleRights::All, 0b1111)]
    fn castles_are_expected_u8(castle_rights: CastleRights, expected_repr: u8) {
        assert_eq!(castle_rights.bits, expected_repr);
    }

    #[test_case(
        CastleRights::All,
        Castle::new(PlayerColor::White, CastleDirection::KingSide),
        true
    )]
    #[test_case(
        CastleRights::All,
        Castle::new(PlayerColor::White, CastleDirection::QueenSide),
        true
    )]
    #[test_case(
        CastleRights::All,
        Castle::new(PlayerColor::Black, CastleDirection::KingSide),
        true
    )]
    #[test_case(
        CastleRights::All,
        Castle::new(PlayerColor::Black, CastleDirection::QueenSide),
        true
    )]
    #[test_case(
        CastleRights::WhiteBoth,
        Castle::new(PlayerColor::White, CastleDirection::KingSide),
        true
    )]
    #[test_case(
        CastleRights::WhiteBoth,
        Castle::new(PlayerColor::White, CastleDirection::QueenSide),
        true
    )]
    #[test_case(
        CastleRights::WhiteBoth,
        Castle::new(PlayerColor::Black, CastleDirection::KingSide),
        false
    )]
    #[test_case(
        CastleRights::WhiteBoth,
        Castle::new(PlayerColor::Black, CastleDirection::QueenSide),
        false
    )]
    #[test_case(
        CastleRights::BlackBoth,
        Castle::new(PlayerColor::White, CastleDirection::KingSide),
        false
    )]
    #[test_case(
        CastleRights::BlackBoth,
        Castle::new(PlayerColor::White, CastleDirection::QueenSide),
        false
    )]
    #[test_case(
        CastleRights::BlackBoth,
        Castle::new(PlayerColor::Black, CastleDirection::KingSide),
        true
    )]
    #[test_case(
        CastleRights::BlackBoth,
        Castle::new(PlayerColor::Black, CastleDirection::QueenSide),
        true
    )]
    #[test_case(
        CastleRights::None,
        Castle::new(PlayerColor::White, CastleDirection::KingSide),
        false
    )]
    #[test_case(
        CastleRights::None,
        Castle::new(PlayerColor::White, CastleDirection::QueenSide),
        false
    )]
    #[test_case(
        CastleRights::None,
        Castle::new(PlayerColor::Black, CastleDirection::KingSide),
        false
    )]
    #[test_case(
        CastleRights::None,
        Castle::new(PlayerColor::Black, CastleDirection::QueenSide),
        false
    )]
    fn can_castle_works(rights: CastleRights, castle: Castle, expected: bool) {
        assert_eq!(rights.can_castle(castle), expected);
    }

    #[test_case("-", Ok(CastleRights::None))]
    #[test_case("K", Ok(CastleRights::WhiteKing))]
    #[test_case("Q", Ok(CastleRights::WhiteQueen))]
    #[test_case("KQ", Ok(CastleRights::WhiteBoth))]
    #[test_case("k", Ok(CastleRights::BlackKing))]
    #[test_case("Kk", Ok(CastleRights::BothKings))]
    #[test_case("Qk", Ok(CastleRights::WhiteQueenBlackKing))]
    #[test_case("KQk", Ok(CastleRights::WhiteBothBlackKing))]
    #[test_case("q", Ok(CastleRights::BlackQueen))]
    #[test_case("Kq", Ok(CastleRights::WhiteKingBlackQueen))]
    #[test_case("Qq", Ok(CastleRights::BothQueens))]
    #[test_case("KQq", Ok(CastleRights::WhiteBothBlackQueen))]
    #[test_case("kq", Ok(CastleRights::BlackBoth))]
    #[test_case("Kkq", Ok(CastleRights::WhiteKingBlackBoth))]
    #[test_case("Qkq", Ok(CastleRights::WhiteQueenBlackBoth))]
    #[test_case("KQkq", Ok(CastleRights::All))]
    #[test_case("KQkqa", Err(IllegalCastleRights))]
    #[test_case("lKQkq", Err(IllegalCastleRights))]
    #[test_case("9430", Err(IllegalCastleRights))]
    #[test_case("____", Err(IllegalCastleRights))]
    fn from_str_works(input: &str, expected: Result<CastleRights, IllegalCastleRights>) {
        assert_eq!(CastleRights::from_str(input), expected);
    }

    #[test_case(PlayerColor::White, CastleRights::WhiteBoth)]
    #[test_case(PlayerColor::Black, CastleRights::BlackBoth)]
    fn for_side_works(player: PlayerColor, expected: CastleRights) {
        assert_eq!(CastleRights::for_player(player), expected);
    }
}
