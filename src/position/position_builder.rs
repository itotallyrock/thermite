use crate::castles::CastleRights;
use crate::half_move_clock::HalfMoveClock;
use crate::pieces::{OwnedPiece, PlacedPiece};
use crate::player_color::PlayerColor;
use crate::ply_count::PlyCount;
use crate::square::{EnPassantSquare, Square};
use enum_map::EnumMap;

/// Allows setting up a board and performing pseudo-legal moves without checking legality.
/// With the end goal being to [`convert`](std::convert) this into a [`LegalPosition`]
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PositionBuilder {
    pub(super) halfmove_clock: HalfMoveClock,
    pub(super) halfmove_count: PlyCount,
    pub(super) squares: EnumMap<Square, Option<OwnedPiece>>,
    pub(super) starting_player: PlayerColor,
    pub(super) castle_rights: CastleRights,
    pub(super) en_passant_square: Option<EnPassantSquare>,
}

impl PositionBuilder {
    /// Place a [`piece`](crate::pieces::PieceType) for a [`side`](PlayerColor) on a [`square`](Square)
    pub fn with_piece(mut self, piece: PlacedPiece) -> Self {
        let PlacedPiece {
            owned_piece,
            square,
        } = piece;
        self.squares[square] = Some(owned_piece);
        self
    }

    /// Set the starting [`player`](PlayerColor)
    pub const fn with_starting_player(mut self, starting_player: PlayerColor) -> Self {
        self.starting_player = starting_player;
        self
    }

    /// Set who has rights to [`castle`](CastleRights)
    pub const fn with_castle_rights(mut self, castle_rights: CastleRights) -> Self {
        self.castle_rights = castle_rights;
        self
    }

    /// Set the halfmove clock
    pub const fn with_halfmove_clock(mut self, halfmove_clock: HalfMoveClock) -> Self {
        self.halfmove_clock = halfmove_clock;
        self
    }

    /// Set how many full moves into the game we are
    pub fn with_fullmove_count(mut self, fullmove_count: PlyCount) -> Self {
        self.halfmove_count = PlyCount::new(*fullmove_count.as_ref() / 2);
        self
    }

    /// Set the [`en-passant square`](EnPassantSquare)
    pub const fn with_en_passant_square(mut self, en_passant_square: EnPassantSquare) -> Self {
        self.en_passant_square = Some(en_passant_square);
        self
    }
}

impl Default for PositionBuilder {
    fn default() -> Self {
        Self {
            halfmove_clock: HalfMoveClock::default(),
            halfmove_count: PlyCount::default(),
            squares: EnumMap::default(),
            starting_player: PlayerColor::White,
            castle_rights: CastleRights::None,
            en_passant_square: None,
        }
    }
}
