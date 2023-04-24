use crate::player_color::PlayerColor;
use crate::square::Square;
use bitmask_enum::bitmask;
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

/// The direction to castle in for either side
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CastleDirection {
    /// Castle with the rook on the same side as the king
    KingSide,
    /// Castle with the rook on the same side as the queen
    QueenSide,
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_castle_squares_are_correct() {
        assert_eq!(KING_FROM_SQUARES[PlayerColor::White], Square::E1);
        assert_eq!(KING_FROM_SQUARES[PlayerColor::Black], Square::E8);
        assert_eq!(
            ROOK_FROM_SQUARES[CastleDirection::KingSide][PlayerColor::White],
            Square::H1
        );
        assert_eq!(
            ROOK_FROM_SQUARES[CastleDirection::KingSide][PlayerColor::Black],
            Square::H8
        );
        assert_eq!(
            ROOK_FROM_SQUARES[CastleDirection::QueenSide][PlayerColor::White],
            Square::A1
        );
        assert_eq!(
            ROOK_FROM_SQUARES[CastleDirection::QueenSide][PlayerColor::Black],
            Square::A8
        );
        assert_eq!(
            ROOK_TO_SQUARES[CastleDirection::KingSide][PlayerColor::White],
            Square::F1
        );
        assert_eq!(
            ROOK_TO_SQUARES[CastleDirection::KingSide][PlayerColor::Black],
            Square::F8
        );
        assert_eq!(
            ROOK_TO_SQUARES[CastleDirection::QueenSide][PlayerColor::White],
            Square::D1
        );
        assert_eq!(
            ROOK_TO_SQUARES[CastleDirection::QueenSide][PlayerColor::Black],
            Square::D8
        );
    }
}
