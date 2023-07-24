use crate::chess_move::quiet::Quiet;
use crate::pieces::{Piece, PieceType};
use crate::player_color::PlayerColor;
use crate::square::{DoublePawnToSquare, EnPassantSquare, File, PromotableSquare};
use enum_map::EnumMap;

/// A valid double-pawn push, or a special starting rank unobstructed two square pawn push
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DoublePawnPush {
    /// The player doing the pawn pushing, the owner of the piece
    player: PlayerColor,
    /// The file the pawn is pushing along
    file: File,
}

impl DoublePawnPush {
    /// Create a new [`DoublePawnPush`] for a given [player](PlayerColor) moving a [`Pawn`](pieces::PieceType::Pawn) on a [`File`]
    #[must_use]
    pub(crate) const fn new(player: PlayerColor, file: File) -> Self {
        Self { player, file }
    }
    /// The starting [`square`](crate::square::Square) the pawn is moving `from`
    #[must_use]
    pub fn from(&self) -> PromotableSquare {
        #[allow(clippy::enum_glob_use)]
        use crate::square::PromotableSquare::*;
        const FROM_SQUARES: EnumMap<PlayerColor, EnumMap<File, PromotableSquare>> =
            EnumMap::from_array([
                EnumMap::from_array([A2, B2, C2, D2, E2, F2, G2, H2]),
                EnumMap::from_array([A7, B7, C7, D7, E7, F7, G7, H7]),
            ]);

        FROM_SQUARES[self.player][self.file]
    }

    /// The ending [`square`](DoublePawnToSquare) the pawn jumped `to`
    #[must_use]
    pub fn to(&self) -> DoublePawnToSquare {
        #[allow(clippy::enum_glob_use)]
        use crate::square::DoublePawnToSquare::*;
        const TO_SQUARES: EnumMap<PlayerColor, EnumMap<File, DoublePawnToSquare>> =
            EnumMap::from_array([
                EnumMap::from_array([A4, B4, C4, D4, E4, F4, G4, H4]),
                EnumMap::from_array([A5, B5, C5, D5, E5, F5, G5, H5]),
            ]);

        TO_SQUARES[self.player][self.file]
    }

    /// The square being jumped over
    #[must_use]
    pub fn en_passant_square(&self) -> EnPassantSquare {
        #[allow(clippy::enum_glob_use)]
        use crate::square::EnPassantSquare::*;
        const EP_SQUARES: EnumMap<PlayerColor, EnumMap<File, EnPassantSquare>> =
            EnumMap::from_array([
                EnumMap::from_array([A3, B3, C3, D3, E3, F3, G3, H3]),
                EnumMap::from_array([A6, B6, C6, D6, E6, F6, G6, H6]),
            ]);

        EP_SQUARES[self.player][self.file]
    }
}

impl From<DoublePawnPush> for Quiet {
    fn from(value: DoublePawnPush) -> Self {
        Self::new(
            value.from().into(),
            value.to().into(),
            PieceType::Pawn.owned_by(value.player),
        )
        .expect("DoublePawnPush shouldn't have same from and to squares")
    }
}
