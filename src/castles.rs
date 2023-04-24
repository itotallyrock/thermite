use bitmask_enum::bitmask;
use enum_map::Enum;

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
    All = Self::WhiteBoth.or(Self::BlackBoth).bits
}
