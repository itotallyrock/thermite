#![warn(missing_docs, clippy::pedantic, rustdoc::missing_doc_code_examples, clippy::nursery, clippy::cargo, clippy::style)]

use std::hash::Hasher;
use arrayvec::ArrayVec;
use nutype::nutype;
use enum_map::{Enum, EnumMap};
use derive_more::{AsMut, AsRef};
use subenum::subenum;
use bitmask_enum::bitmask;
use player_color::PlayerColor;
use square::Square;

mod player_color;
mod square;

#[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum PieceType {
    #[subenum(NonKingPieceType)]
    Pawn,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Knight,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Bishop,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Rook,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Queen,
    #[subenum(NonPawnPieceType)]
    King,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OwnedPiece {
    piece: PieceType,
    player: PlayerColor,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PlacedPiece {
    owned_piece: OwnedPiece,
    square: Square,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct BoardMask(u64);

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RawPositionState {
    checkers: BoardMask,
    pinners_for: EnumMap<PlayerColor, BoardMask>,
    blockers_for: EnumMap<PlayerColor, BoardMask>,
    check_squares: EnumMap<PieceType, BoardMask>,
    en_passant_square: Option<Square>,
    castles: CastleRights,
}

impl Default for RawPositionState {
    fn default() -> Self {
        Self {
            checkers: BoardMask::default(),
            pinners_for: EnumMap::default(),
            blockers_for: EnumMap::default(),
            check_squares: EnumMap::default(),
            en_passant_square: None,
            castles: CastleRights::None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RawPosition {
    hash: ZobristHash,
    player_to_move: PlayerColor,
    squares: EnumMap<Square, Option<PieceType>>,
    pieces_masks: EnumMap<NonKingPieceType, BoardMask>,
    side_masks: EnumMap<PlayerColor, BoardMask>,
    king_squares: EnumMap<PlayerColor, Square>,
    state: RawPositionState,
}

impl Default for RawPosition {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            player_to_move: PlayerColor::White,
            squares: Default::default(),
            pieces_masks: Default::default(),
            side_masks: Default::default(),
            king_squares: EnumMap::from_array([Square::E1, Square::E8]),
            state: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum IllegalPosition {

}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct LegalPositionState {
    raw_state: RawPositionState,
    halfmove_clock: HalfMoveClock,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, AsRef, AsMut)]
pub struct LegalPosition {
    #[as_ref()]
    #[as_mut()]
    position: RawPosition,
    state: LegalPositionState,
    hash_history: Box<ArrayVec<HistoryHash, { HALF_MOVE_LIMIT_USIZE }>>,
}

impl TryFrom<RawPosition> for LegalPosition {
    type Error = IllegalPosition;

    fn try_from(position: RawPosition) -> Result<Self, Self::Error> {
        let state = LegalPositionState {
            raw_state: position.state,
            halfmove_clock: Default::default(),
        };

        Ok(Self {
            position,
            state,
            hash_history: Box::new(Default::default()),
        })
    }
}

#[nutype]
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct HistoryHash(u8);

#[nutype]
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct ZobristHash(u64);

impl Hasher for ZobristHash {
    fn finish(&self) -> u64 {
        self.into_inner()
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes.chunks_exact(u64::BITS as usize / 8)
            .map(|bits| u64::from_be_bytes(bits.try_into().unwrap()))
            .for_each(|chunk| self.write_u64(chunk));
    }

    fn write_u64(&mut self, i: u64) {
        *self = Self::new(self.into_inner() ^ i);
    }
}

impl Default for ZobristHash {
    fn default() -> Self {
        Self::new(0xF1DC_4349_4EA4_76CE)
    }
}

impl From<ZobristHash> for HistoryHash {
    fn from(value: ZobristHash) -> Self {
        // Intentional truncation for a smaller memory footprint with still enough bits to avoid a hash collision
        #[allow(clippy::cast_possible_truncation)]
        Self::new(*value.as_ref() as u8)
    }
}

impl PartialEq<HistoryHash> for ZobristHash {
    fn eq(&self, other: &HistoryHash) -> bool {
        HistoryHash::from(*self) == *other
    }
}

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;

#[nutype(validate(max = 50))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct HalfMoveClock(u8);

impl Default for HalfMoveClock {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl HalfMoveClock {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[nutype(validate(max = 255))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct PlyCount(u8);

impl PlyCount {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn decrement(&mut self) {
        *self = Self::new(self.into_inner().saturating_sub(1)).unwrap();
    }
}


#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Game {
    #[as_ref()]
    #[as_mut()]
    legal_position: LegalPosition,
    halfmove_count: PlyCount,
}

#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Searchable {
    #[as_ref()]
    #[as_mut()]
    game: Game,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
