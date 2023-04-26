use crate::board_mask::BoardMask;
use crate::castles::CastleRights;
use crate::pieces::{NonKingPieceType, PieceType};
use crate::player_color::PlayerColor;
use crate::square::Square;
use crate::zobrist::ZobristHash;
use enum_map::EnumMap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    checkers: BoardMask,
    pinners_for: EnumMap<PlayerColor, BoardMask>,
    blockers_for: EnumMap<PlayerColor, BoardMask>,
    check_squares: EnumMap<PieceType, BoardMask>,
    en_passant_square: Option<Square>,
    castles: CastleRights,
}

impl Default for State {
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
    pub(crate) state: State,
}

impl Default for RawPosition {
    fn default() -> Self {
        Self {
            hash: ZobristHash::default(),
            player_to_move: PlayerColor::White,
            squares: EnumMap::default(),
            pieces_masks: EnumMap::default(),
            side_masks: EnumMap::default(),
            king_squares: EnumMap::from_array([Square::E1, Square::E8]),
            state: State::default(),
        }
    }
}
