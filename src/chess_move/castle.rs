use crate::castles::{CastleDirection, KING_FROM_SQUARES, KING_TO_SQUARES};
use crate::player_color::PlayerColor;
use crate::square::Square;
use enum_iterator::all;

/// A valid castle move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct Castle {
    /// The player doing the castling
    pub(crate) player: PlayerColor,
    /// The direction the king is castling towards
    pub(crate) direction: CastleDirection,
}

impl Castle {
    /// Get the king's original [`Square`]
    #[must_use]
    pub fn king_from(&self) -> Square {
        KING_FROM_SQUARES[self.player]
    }

    /// Get the king's destination [`Square`]
    #[must_use]
    pub fn king_to(&self) -> Square {
        KING_TO_SQUARES[self.direction][self.player]
    }

    /// Get the [player](PlayerColor) doing the castling
    #[must_use]
    pub const fn player(&self) -> PlayerColor {
        self.player
    }

    /// Get the [`CastleDirection`]
    #[must_use]
    pub const fn direction(&self) -> CastleDirection {
        self.direction
    }

    /// Get all possible castle moves
    pub(crate) fn all() -> impl Iterator<Item = Self> {
        all::<PlayerColor>().flat_map(|player| {
            all::<CastleDirection>().map(move |direction| Self { player, direction })
        })
    }
}
