mod rights;

use crate::player::ByPlayer;
use crate::player::Player;
use crate::square::Square;
pub use rights::{CastleRights, IllegalCastleRights};

/// How many castle moves there are total.
/// 4 for white king side, white queen side, black king side, black queen side.
pub const NUM_CASTLES: usize = 4;

const STANDARD_KING_SQUARES: ByPlayer<Square> = ByPlayer::new_with(Square::E1, Square::E8);
const STANDARD_QUEEN_ROOK_SQUARES: ByPlayer<Square> = ByPlayer::new_with(Square::A1, Square::A8);
const STANDARD_KING_ROOK_SQUARES: ByPlayer<Square> = ByPlayer::new_with(Square::H1, Square::H8);

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
    #[cfg(feature = "chess_960")]
    is_chess_960: bool,
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
    /// Create the state management for a board's castle permissions, needs a starting set of rights, and each rook and king square needs to be included as well.
    #[cfg(feature = "chess_960")]
    #[must_use]
    pub const fn new(
        rights: CastleRights,
        white_king_square: Square,
        black_king_square: Square,
        white_queen_rook_square: Square,
        black_queen_rook_square: Square,
        white_king_rook_square: Square,
        black_king_rook_square: Square,
    ) -> Self {
        const DEFAULT_CASTLES: Castles = Castles::default();
        let king_square = ByPlayer::new_with(white_king_square, black_king_square);
        let queen_rook_square = ByPlayer::new_with(white_queen_rook_square, black_queen_rook_square);
        let king_rook_square = ByPlayer::new_with(white_king_rook_square, black_king_rook_square);
        // If a position doesnt have standard castle squares then assume chess_960 when constructing
        let is_chess_960 = DEFAULT_CASTLES.king_square != king_square
            || DEFAULT_CASTLES.king_rook_square != king_rook_square
            || DEFAULT_CASTLES.queen_rook_square != queen_rook_square;

        Self {
            rights,
            king_square,
            queen_rook_square,
            king_rook_square,
            is_chess_960,
        }
    }

    /// Create the state management for a board's castle permissions, needs a starting set of rights.
    #[cfg(not(feature = "chess_960"))]
    #[must_use]
    pub const fn new(
        rights: CastleRights,
    ) -> Self {
        Self {
            rights,
        }
    }

    /// Get the starting square one of the two castle rooks must be on in order to castle for a given side.
    /// Does not take into account rights, simply returns the expected square.
    /// ```
    #[must_use]
    pub const fn rook_square(&self, side: Player, king_side: bool) -> Square {
        #[cfg(not(feature = "chess_960"))]
        let rook_squares = if king_side { STANDARD_KING_ROOK_SQUARES } else { STANDARD_QUEEN_ROOK_SQUARES };
        #[cfg(feature = "chess_960")]
        let rook_squares = if king_side { self.king_rook_square } else { self.queen_rook_square };

        *rook_squares.get_side(side)
    }

    /// Get the starting square the king must be on to castle for a given side.
    /// Does not take into account rights, simply returns the expected square.
    #[must_use]
    pub const fn king_square(&self, side: Player) -> Square {
        #[cfg(not(feature = "chess_960"))]
        let king_squares = STANDARD_KING_SQUARES;
        #[cfg(feature = "chess_960")]
        let king_squares = self.king_square;

        *king_squares.get_side(side)
    }
}

impl const Default for Castles {
    fn default() -> Self {
        #[cfg(feature = "chess_960")]
        let default = Self {
            rights: CastleRights::None,
            king_square: STANDARD_KING_SQUARES,
            queen_rook_square: STANDARD_QUEEN_ROOK_SQUARES,
            king_rook_square: STANDARD_KING_ROOK_SQUARES,
            is_chess_960: false,
        };
        #[cfg(not(feature = "chess_960"))]
        let default = Self {
            rights: CastleRights::None,
        };

        default
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_castle_squares_are_correct() {
        assert_eq!(STANDARD_KING_SQUARES.get_side(Player::White), &Square::E1);
        assert_eq!(STANDARD_KING_SQUARES.get_side(Player::Black), &Square::E8);
        assert_eq!(STANDARD_KING_ROOK_SQUARES.get_side(Player::White), &Square::H1);
        assert_eq!(STANDARD_KING_ROOK_SQUARES.get_side(Player::Black), &Square::H8);
        assert_eq!(STANDARD_QUEEN_ROOK_SQUARES.get_side(Player::White), &Square::A1);
        assert_eq!(STANDARD_QUEEN_ROOK_SQUARES.get_side(Player::Black), &Square::A8);
    }
}