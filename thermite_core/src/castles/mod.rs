mod rights;

use crate::player::Player;
#[cfg(feature = "chess_960")]
use crate::player::ByPlayer;
use crate::square::Square;
pub use rights::{CastleRights, IllegalCastleRights};

/// How many castle moves there are total.
/// 4 for white king side, white queen side, black king side, black queen side.
pub const NUM_CASTLES: usize = 4;

/// The state management for a game of chess's castle permissions
/// Keep track of starting rights, the squares to monitor for invalidating those rights, and masks for checking attacked squares.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Castles {
    rights: CastleRights,

    #[cfg(feature = "chess_960")]
    king_square: ByPlayer<Square>,
    #[cfg(feature = "chess_960")]
    queen_rook_square: ByPlayer<Square>,
    #[cfg(feature = "chess_960")]
    king_rook_square: ByPlayer<Square>,

    // #[cfg(feature = "move_generation")]
    // TODO: Store board masks of squares that cannot be attacked for each side's castles
}

impl const AsRef<CastleRights> for Castles {
    fn as_ref(&self) -> &CastleRights {
        &self.rights
    }
}

impl const AsMut<CastleRights> for Castles {
    fn as_mut(&mut self) -> &mut CastleRights {
        &mut self.rights
    }
}

impl Castles {
    /// Create the state management for a board's castle permissions, needs a starting set of rights.
    /// If `chess_960` then, then each rook and king square needs to be included as well.
    #[must_use]
    pub const fn new(
        rights: CastleRights,
        #[cfg(feature = "chess_960")] white_king_square: Square,
        #[cfg(feature = "chess_960")] black_king_square: Square,
        #[cfg(feature = "chess_960")] white_queen_rook_square: Square,
        #[cfg(feature = "chess_960")] black_queen_rook_square: Square,
        #[cfg(feature = "chess_960")] white_king_rook_square: Square,
        #[cfg(feature = "chess_960")] black_king_rook_square: Square,
    ) -> Self {
        Self {
            rights,
            #[cfg(feature = "chess_960")]
            king_square: ByPlayer::new_with(white_king_square, black_king_square),
            #[cfg(feature = "chess_960")]
            queen_rook_square: ByPlayer::new_with(white_queen_rook_square, black_queen_rook_square),
            #[cfg(feature = "chess_960")]
            king_rook_square: ByPlayer::new_with(white_king_rook_square, black_king_rook_square),
        }
    }

    /// Get the starting square one of the two castle rooks must be on in order to castle for a given side.
    /// Does not take into account rights, simply returns the expected square.
    /// ```
    #[must_use]
    pub const fn rook_square(&self, side: Player, king_side: bool) -> Square {
        #[cfg(not(feature = "chess_960"))]
        match (side, king_side) {
            (Player::White, true) => Square::H1,
            (Player::White, false) => Square::A1,
            (Player::Black, true) => Square::H8,
            (Player::Black, false) => Square::A8,
        }
        #[cfg(feature = "chess_960")]
        if king_side {
            *self.king_rook_square.get_side(side)
        } else {
            *self.queen_rook_square.get_side(side)
        }
    }

    /// Get the starting square the king must be on to castle for a given side.
    /// Does not take into account rights, simply returns the expected square.
    #[must_use]
    pub const fn king_square(&self, side: Player) -> Square {
        #[cfg(not(feature = "chess_960"))]
        match side {
            Player::White => Square::E1,
            Player::Black => Square::E8,
        }
        #[cfg(feature = "chess_960")]
        *self.king_square.get_side(side)
    }
}
