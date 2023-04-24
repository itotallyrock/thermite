use enum_map::EnumMap;
use crate::pieces::{NonKingPieceType, PieceType};
use crate::player_color::PlayerColor;
use crate::square::Square;
use crate::{BoardMask, ZobristHash};
use crate::castles::CastleRights;

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
