use crate::board_mask::BoardMask;
use crate::castles::CastleRights;
use crate::half_move_clock::{HalfMoveClock, HALF_MOVE_LIMIT_USIZE};
use crate::pieces::{NonKingPieceType, OwnedPiece, PieceType};
use crate::player_color::PlayerColor;
use crate::position::position_builder::PositionBuilder;
use crate::square::Square;
use crate::zobrist::{HistoryHash, ZobristHash};
use alloc::boxed::Box;
use arrayvec::ArrayVec;
use derive_more::{AsMut, AsRef, Display, Error};
use enum_map::EnumMap;

/// Invalid standard chess position (violates rules)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash, Error, Display)]
pub enum IllegalPosition {}

/// The hard to compute or irrecoverable/irreversible state
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    // Irrecoverable state
    pub(super) halfmove_clock: HalfMoveClock,
    pub(super) en_passant_square: Option<Square>,
    pub(super) castles: CastleRights,
    // Move generation state
    pub(super) checkers: BoardMask,
    pub(super) pinners_for: EnumMap<PlayerColor, BoardMask>,
    pub(super) blockers_for: EnumMap<PlayerColor, BoardMask>,
    pub(super) check_squares: EnumMap<PieceType, BoardMask>,
}

/// A position known to be valid and legal in standard chess.
/// Keeps track of [`state`](State) to maintain legality as the board is mutated.
#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct LegalPosition {
    hash: ZobristHash,
    player_to_move: PlayerColor,
    pieces_masks: EnumMap<NonKingPieceType, BoardMask>,
    side_masks: EnumMap<PlayerColor, BoardMask>,
    king_squares: EnumMap<PlayerColor, Square>,
    state: State,
    hash_history: Box<ArrayVec<HistoryHash, { HALF_MOVE_LIMIT_USIZE }>>,
}

impl TryFrom<PositionBuilder> for LegalPosition {
    type Error = IllegalPosition;

    fn try_from(position: PositionBuilder) -> Result<Self, Self::Error> {
        let PositionBuilder {
            halfmove_clock,
            halfmove_count,
            squares,
            starting_player: player_to_move,
            castle_rights: castles,
            en_passant_square,
        } = position;
        let (pieces_masks, side_masks, king_squares, hash) =
            squares.iter().filter_map(|(s, p)| p.map(|p| (s, p))).fold(
                (
                    EnumMap::<NonKingPieceType, BoardMask>::default(),
                    EnumMap::<PlayerColor, BoardMask>::default(),
                    EnumMap::from_array([Square::E1, Square::E8]),
                    ZobristHash::default(),
                ),
                |(mut pieces_masks, mut side_masks, mut king_squares, hash),
                 (square, OwnedPiece { piece, player })| {
                    let square_offset = square as u8;
                    let square_mask = BoardMask::new(1) << square_offset;
                    // TODO: update hash using zobrist key lookup
                    side_masks[player] |= square_mask;
                    // If the piece is a king add it to the king squares, otherwise add it to the piece masks
                    NonKingPieceType::try_from(piece).map_or_else(
                        |_| {
                            king_squares[player] = square;
                        },
                        |non_king_piece| {
                            pieces_masks[non_king_piece] |= square_mask;
                        },
                    );

                    (pieces_masks, side_masks, king_squares, hash)
                },
            );
        let checkers = BoardMask::default();
        let pinners_for = EnumMap::default();
        let blockers_for = EnumMap::default();
        let check_squares = EnumMap::default();
        let en_passant_square = en_passant_square.map(Into::into);
        let state = State {
            halfmove_clock,
            en_passant_square,
            castles,
            checkers,
            pinners_for,
            blockers_for,
            check_squares,
        };

        let hash_history = Box::default(); // TODO: Get this from builder (when we have starting moves implemented)

        let pseudo_legal_position = Self {
            hash,
            player_to_move,
            pieces_masks,
            side_masks,
            king_squares,
            state,
            hash_history,
        };

        // TODO: Update state, hash_history
        // TODO: Check legality

        Ok(pseudo_legal_position)
    }
}
