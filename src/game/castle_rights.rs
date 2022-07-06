use crate::game::Side;
use num_enum::TryFromPrimitive;

/// Flag enum for keeping track of the 4 individual castle rights.
///
/// [White](Side::White) and [Black](Side::Black) both support two directions for castling.
/// - King-side - Castling with the rook on the same side of the board as the [king](PieceType::King)
/// - Queen-side - Castling with the rook on the same side of the board as the [queen](PieceType::Queen)
///
/// This gives a total of 4 flags.  This enum helps to manipulate and read individual flags.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, TryFromPrimitive)]
#[repr(u8)]
pub enum CastleRights {
    /// No side has any castling abilities
    #[default]
    None = 0b0000,
    /// White can king-side castle, but black can't do either
    WhiteKingBlackNone = 0b0001,
    /// White can queen-side castle, but black can't do either
    WhiteQueenBlackNone = 0b0010,
    /// White can do either, but black can do neither
    WhiteBothBlackNone = 0b0011,
    /// White cannot castle, but black can king-side
    WhiteNoneBlackKing = 0b0100,
    /// Both sides can king-side castle
    WhiteKingBlackKing = 0b0101,
    /// White can queen-side, and black can king-side castle
    WhiteQueenBlackKing = 0b0110,
    /// White can do either, and black can king-side castle
    WhiteBothBlackKing = 0b0111,
    /// White can't do either, and black can queen-side castle
    WhiteNoneBlackQueen = 0b1000,
    /// White can king-side, and black can queen-side castle
    WhiteKingBlackQueen = 0b1001,
    /// Both sides can queen-side castle
    WhiteQueenBlackQueen = 0b1010,
    /// White can do either, and black can queen-side castle
    WhiteBothBlackQueen = 0b1011,
    /// White cannot castle, but black can do either
    WhiteNoneBlackBoth = 0b1100,
    /// White can king-side castle, and black can do either
    WhiteKingBlackBoth = 0b1101,
    /// White can queen-side castle, and black can do either
    WhiteQueenBlackBoth = 0b1110,
    /// Both sides have full castling abilities
    All = 0b1111,
}

impl CastleRights {
    /// The number of individual castle abilities (2 for each [Side], king-side/queen-side)
    pub const COUNT: usize = 4;

    const ANY_SIDE_MASKS: [u8; Side::COUNT] = [Self::WhiteBothBlackNone as u8, Self::WhiteNoneBlackBoth as u8];
    const KING_SIDE_MASKS: [u8; Side::COUNT] = [Self::WhiteKingBlackNone as u8, Self::WhiteNoneBlackKing as u8];
    const QUEEN_SIDE_MASKS: [u8; Side::COUNT] = [Self::WhiteQueenBlackNone as u8, Self::WhiteNoneBlackQueen as u8];

    /// Get full castle rights (king-side and queen-side) for a given side (the other side won't be included)
    pub fn for_side(side: Side) -> Self {
        const CASTLE_SIDE_MAP: [CastleRights; Side::COUNT] = [CastleRights::WhiteBothBlackNone, CastleRights::WhiteNoneBlackBoth];
        CASTLE_SIDE_MAP[side as usize]
    }

    /// Remove a subset of rights from an instance
    pub fn remove_rights(&mut self, rights: CastleRights) {
        *self = (*self as u8 & !(rights as u8)).try_into().unwrap();
    }

    /// If a given side has any castle rights (king-side and queen-side)
    pub fn can_castle(&self, side: Side) -> bool {
        *self as u8 & Self::ANY_SIDE_MASKS[side as usize] != 0
    }
    /// If a given side has the rights to castle king-side
    pub fn can_king_castle(&self, side: Side) -> bool {
        *self as u8 & Self::KING_SIDE_MASKS[side as usize] != 0
    }
    /// If a given side has the rights to castle queen-side
    pub fn can_queen_castle(&self, side: Side) -> bool {
        *self as u8 & Self::QUEEN_SIDE_MASKS[side as usize] != 0
    }
}
