use crate::player_color::PlayerColor;
use crate::square::Square;
use bitmask_enum::bitmask;
use enum_map::{Enum, EnumMap};

pub const STANDARD_KING_FROM_SQUARES: EnumMap<PlayerColor, Square> =
    EnumMap::from_array([Square::E1, Square::E8]);

pub const STANDARD_KING_TO_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::G1, Square::G8]),
        EnumMap::from_array([Square::C1, Square::C8]),
    ]);

pub const STANDARD_ROOK_FROM_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::H1, Square::H8]),
        EnumMap::from_array([Square::A1, Square::A8]),
    ]);

pub const STANDARD_ROOK_TO_SQUARES: EnumMap<CastleDirection, EnumMap<PlayerColor, Square>> =
    EnumMap::from_array([
        EnumMap::from_array([Square::F1, Square::F8]),
        EnumMap::from_array([Square::D1, Square::D8]),
    ]);

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum CastleDirection {
    KingSide,
    QueenSide,
}

#[bitmask(u8)]
pub enum CastleRights {
    None = 0,
    WhiteKing = 0b0001,
    WhiteQueen = 0b0010,
    WhiteBoth = Self::WhiteKing.or(Self::WhiteQueen).bits,
    BlackKing = 0b0100,
    BlackQueen = 0b1000,
    BlackBoth = Self::WhiteKing.or(Self::WhiteQueen).bits,
    All = Self::WhiteBoth.or(Self::BlackBoth).bits,
}
