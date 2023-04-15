pub use rights::{CastleRights, IllegalCastleRights};

use crate::bitboard::Bitboard;
pub use crate::castles::by_castle_direction::ByCastleDirection;
pub use crate::castles::direction::CastleDirection;
use crate::castles::squares::{STANDARD_KING_FROM_SQUARES, STANDARD_KING_TO_SQUARES, STANDARD_ROOK_FROM_SQUARES, STANDARD_ROOK_TO_SQUARES};
#[cfg(feature = "chess_960")]
use crate::player::ByPlayer;
use crate::player::Player;
use crate::square::Square;

mod rights;
mod by_castle_direction;
mod direction;
mod squares;

/// How many castle moves there are total.
/// 4 for white king side, white queen side, black king side, black queen side.
pub const NUM_CASTLES: usize = 4;

/// The state management for a game of chess's castle permissions
/// Keep track of starting rights, the squares to monitor for invalidating those rights, and masks for checking attacked squares.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Castles {
    rights: CastleRights,
    #[cfg(feature = "chess_960")]
    king_starting_squares: ByPlayer<Square>,
    #[cfg(feature = "chess_960")]
    rook_starting_squares: ByCastleDirection<ByPlayer<Square>>,
    #[cfg(feature = "chess_960")]
    is_chess_960: bool,
}

const fn get_unoccupied_path(king_square: Square, rook_square: Square) -> Bitboard {
    Bitboard::line_between(king_square, rook_square) ^ rook_square.to_mask()
}

const fn get_unattacked_path(king_square: Square, king_to_square: Square) -> Bitboard {
    Bitboard::line_between(king_square, king_to_square)
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
        let king_starting_squares = ByPlayer::new_with(white_king_square, black_king_square);
        let rook_starting_squares = ByCastleDirection::new_with(ByPlayer::new_with(white_king_rook_square, black_king_rook_square), ByPlayer::new_with(white_queen_rook_square, black_queen_rook_square));
        // If a position doesnt have standard castle squares then assume chess_960 when constructing
        let is_chess_960 = DEFAULT_CASTLES.king_starting_squares != king_starting_squares
            || *DEFAULT_CASTLES.rook_starting_squares.get_direction(CastleDirection::KingSide) != *rook_starting_squares.get_direction(CastleDirection::KingSide)
            || *DEFAULT_CASTLES.rook_starting_squares.get_direction(CastleDirection::QueenSide) != *rook_starting_squares.get_direction(CastleDirection::QueenSide);

        Self {
            rights,
            king_starting_squares,
            rook_starting_squares,
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
    pub const fn rook_from_square(&self, side: Player, direction: CastleDirection) -> Square {
        #[cfg(not(feature = "chess_960"))]
        let rook_starting_squares = STANDARD_ROOK_FROM_SQUARES;
        #[cfg(feature = "chess_960")]
        let rook_starting_squares = self.rook_starting_squares;

        *rook_starting_squares.get_direction(direction).get_side(side)
    }

    /// Get the starting square the king must be on to castle for a given side.
    /// Does not take into account rights, simply returns the expected square.
    #[must_use]
    pub const fn king_from_square(&self, side: Player) -> Square {
        #[cfg(not(feature = "chess_960"))]
        let king_squares = STANDARD_KING_FROM_SQUARES;
        #[cfg(feature = "chess_960")]
        let king_squares = self.king_starting_squares;

        *king_squares.get_side(side)
    }

    /// Compare the starting squares for the kings and rooks for both sides
    #[cfg(feature = "chess_960")]
    #[must_use]
    pub const fn eq_starting_squares(&self, other: Self) -> bool {
        self.king_starting_squares == other.king_starting_squares
            && self.rook_starting_squares == other.rook_starting_squares
    }

    /// The square the king will end on after castling for a given [player](Player) in a given [direction](CastleDirection)
    #[must_use]
    pub const fn king_to_square(&self, side: Player, direction: CastleDirection) -> Square {
        *STANDARD_KING_TO_SQUARES.get_direction(direction).get_side(side)
    }

    /// The square the rook will end on after castling for a given [player](Player) in a given [direction](CastleDirection)
    #[must_use]
    pub const fn rook_to_square(&self, side: Player, direction: CastleDirection) -> Square {
        *STANDARD_ROOK_TO_SQUARES.get_direction(direction).get_side(side)
    }

    /// The mask that must not contain any pieces in order to castle for a [player](Player) in a given [direction](CastleDirection)
    pub const fn get_unoccupied_path(&self, side: Player, direction: CastleDirection) -> Bitboard {
        get_unoccupied_path(self.king_from_square(side), self.rook_from_square(side, direction))
    }

    /// The mask that must not be attacked for the king to pass through in order to castle for a [player](Player) in a given [direction](CastleDirection)
    pub const fn get_unattacked_path(&self, side: Player, direction: CastleDirection) -> Bitboard {
        get_unattacked_path(self.king_from_square(side), self.king_to_square(side, direction))
    }

    /// Set the `is_chess_960` flag
    #[cfg(feature = "chess_960")]
    pub(crate) const fn set_chess_960(&mut self, is_chess_960: bool) {
        self.is_chess_960 = is_chess_960;
    }
}

impl const Default for Castles {
    fn default() -> Self {
        #[cfg(feature = "chess_960")]
        let default = Self {
            rights: CastleRights::None,
            king_starting_squares: STANDARD_KING_FROM_SQUARES,
            rook_starting_squares: STANDARD_ROOK_FROM_SQUARES,
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
    use test_case::test_case;

    use super::*;

    #[test]
    #[cfg(feature = "chess_960")]
    fn castles_size_remains_consistent() {
        assert_eq!(std::mem::size_of::<Castles>(), 8);
    }

    #[test]
    #[cfg(not(feature = "chess_960"))]
    fn castles_size_remains_consistent() {
        assert_eq!(std::mem::size_of::<Castles>(), 1);
    }

    #[test]
    fn default_rights_are_empty() {
        assert_eq!(Castles::default().rights, CastleRights::None);
    }

    #[test_case(Player::White, CastleDirection::QueenSide, Bitboard(0xE); "white queen side")]
    #[test_case(Player::White, CastleDirection::KingSide, Bitboard(0x60); "white king side")]
    #[test_case(Player::Black, CastleDirection::QueenSide, Bitboard(0x0E00_0000_0000_0000); "black queen side")]
    #[test_case(Player::Black, CastleDirection::KingSide, Bitboard(0x6000_0000_0000_0000); "black king side")]
    fn get_unoccupied_path_works(side: Player, direction: CastleDirection, expected: Bitboard) {
        let castles = Castles::default();
        assert_eq!(get_unoccupied_path(castles.king_from_square(side), castles.rook_from_square(side, direction)), expected);
    }
}