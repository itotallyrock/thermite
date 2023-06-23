use crate::bitboard::BoardMask;
use crate::castles::{
    CastleDirection, CastleRights, KING_FROM_SQUARES, KING_TO_SQUARES, ROOK_FROM_SQUARES,
    ROOK_TO_SQUARES, UNATTACKED_SQUARES,
};
use crate::chess_move::quiet::Quiet;
use crate::pieces::{Piece, PieceType};
use crate::player_color::PlayerColor;
use crate::square::Square;
use enum_iterator::all;

/// A valid castle move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct Castle {
    /// The player doing the castling
    player: PlayerColor,
    /// The direction the king is castling towards
    direction: CastleDirection,
}

impl Castle {
    /// Create a new castle inner move for a [player](PlayerColor) in a given [`CastleDirection`]
    pub(crate) const fn new(player: PlayerColor, direction: CastleDirection) -> Self {
        Self { player, direction }
    }
    /// Get the [king](PieceType::King)'s original [`Square`]
    #[must_use]
    pub fn king_from(&self) -> Square {
        KING_FROM_SQUARES[self.player]
    }

    /// Get the [king](PieceType::King)'s destination [`Square`]
    #[must_use]
    pub fn king_to(&self) -> Square {
        KING_TO_SQUARES[self.direction][self.player]
    }

    /// Get the [rook](PieceType::Rook)'s original [`Square`]
    #[must_use]
    pub fn rook_from(&self) -> Square {
        ROOK_FROM_SQUARES[self.direction][self.player]
    }

    /// Get the [rook](PieceType::Rook)'s destination [`Square`]
    #[must_use]
    pub fn rook_to(&self) -> Square {
        ROOK_TO_SQUARES[self.direction][self.player]
    }

    /// The [rook](PieceType::Rook)'s [move](Quiet)
    #[must_use]
    pub(crate) fn rook_quiet(self) -> Quiet {
        Quiet::new(
            self.rook_from(),
            self.rook_to(),
            PieceType::Rook.owned_by(self.player()),
        )
        .expect("rook quiet from and to squares are different")
    }

    /// The [king](PieceType::King)'s [move](Quiet)
    #[must_use]
    pub(crate) fn king_quiet(self) -> Quiet {
        Quiet::new(
            self.king_from(),
            self.king_to(),
            PieceType::King.owned_by(self.player()),
        )
        .expect("king quiet from and to squares are different")
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

    /// Get the [mask](BoardMask) for [squares](Square) that cannot be attacked in order to [castle](Castle)
    pub fn unattacked_mask(&self) -> BoardMask {
        UNATTACKED_SQUARES[self.direction()][self.player()]
    }

    /// Get the [rights](CastleRights) required to do this castle move
    #[must_use]
    pub const fn required_rights(&self) -> CastleRights {
        match self {
            Self {
                player: PlayerColor::White,
                direction: CastleDirection::KingSide,
            } => CastleRights::WhiteKing,
            Self {
                player: PlayerColor::White,
                direction: CastleDirection::QueenSide,
            } => CastleRights::WhiteQueen,
            Self {
                player: PlayerColor::Black,
                direction: CastleDirection::KingSide,
            } => CastleRights::BlackKing,
            Self {
                player: PlayerColor::Black,
                direction: CastleDirection::QueenSide,
            } => CastleRights::BlackQueen,
        }
    }

    /// Get all possible castle moves
    pub(crate) fn all() -> impl Iterator<Item = Self> {
        all::<PlayerColor>().flat_map(Self::all_for_player)
    }

    /// Get all possible castle moves for a [player](PlayerColor)
    pub(crate) fn all_for_player(player: PlayerColor) -> impl Iterator<Item = Self> {
        all::<CastleDirection>().map(move |direction| Self { player, direction })
    }
}

/// Container for the pair of quiet moves (king and rook) required to castle
pub(crate) struct CastleQuietMoves {
    pub(crate) rook_quiet: Quiet,
    pub(crate) king_quiet: Quiet,
}

impl CastleQuietMoves {
    /// Get the quiet moves for a [`Castle`] move
    #[must_use]
    pub fn new(castle: Castle) -> Self {
        Self {
            rook_quiet: castle.rook_quiet(),
            king_quiet: castle.king_quiet(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::BoardMask;
    use crate::castles::CastleDirection::{KingSide, QueenSide};
    use crate::chess_move::castle::Castle;
    use crate::player_color::PlayerColor::{Black, White};

    use test_case::test_case;

    #[test_case(Castle::new(White, KingSide), BoardMask::new(0x6))]
    #[test_case(Castle::new(White, QueenSide), BoardMask::new(0x70))]
    #[test_case(Castle::new(Black, KingSide), BoardMask::new(0x0600_0000_0000_0000))]
    #[test_case(Castle::new(Black, QueenSide), BoardMask::new(0x7000_0000_0000_0000))]
    fn unattacked_mask_works(castle: Castle, expected_mask: BoardMask) {
        assert_eq!(castle.unattacked_mask(), expected_mask);
    }
}
