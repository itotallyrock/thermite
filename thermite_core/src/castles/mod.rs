pub use rights::{CastleRights, IllegalCastleRights};

use crate::bitboard::Bitboard;
use crate::castles::by_castle_direction::ByCastleDirection;
pub use crate::castles::direction::CastleDirection;
use crate::castles::squares::{STANDARD_KING_FROM_SQUARES, STANDARD_KING_TO_SQUARES, STANDARD_ROOK_FROM_SQUARES, STANDARD_ROOK_TO_SQUARES};
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
    clear_paths: ByCastleDirection<ByPlayer<Bitboard>>,
    #[cfg(feature = "chess_960")]
    unattacked_paths: ByCastleDirection<ByPlayer<Bitboard>>,
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

#[cfg(feature = "chess_960")]
macro_rules! create_path {
    ($start_squares:ident, $end_squares:ident, $mapper:ident) => {{
        ByCastleDirection::new_with(
        ByPlayer::new_with(
            $mapper(*$start_squares.get_side(Player::White), *$end_squares.get_direction(CastleDirection::KingSide).get_side(Player::White)),
            $mapper(*$start_squares.get_side(Player::Black), *$end_squares.get_direction(CastleDirection::KingSide).get_side(Player::Black))
        ),
        ByPlayer::new_with(
            $mapper(*$start_squares.get_side(Player::White), *$end_squares.get_direction(CastleDirection::KingSide).get_side(Player::White)),
            $mapper(*$start_squares.get_side(Player::Black), *$end_squares.get_direction(CastleDirection::KingSide).get_side(Player::Black))
        ),
    )
    }};
}

#[cfg(feature = "chess_960")]
const fn create_castle_paths(king_squares: ByPlayer<Square>, rook_squares: ByCastleDirection<ByPlayer<Square>>) -> ByCastleDirection<ByPlayer<Bitboard>> {
    create_path!(king_squares, rook_squares, get_unoccupied_path)
}

#[cfg(feature = "chess_960")]
const fn create_king_paths(king_from_squares: ByPlayer<Square>, king_to_squares: ByCastleDirection<ByPlayer<Square>>) -> ByCastleDirection<ByPlayer<Bitboard>> {
    create_path!(king_from_squares, king_to_squares, get_unattacked_path)
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
        let rook_paths = create_castle_paths(king_starting_squares, rook_starting_squares);
        let king_paths = create_king_paths(king_starting_squares, STANDARD_KING_TO_SQUARES);
        // If a position doesnt have standard castle squares then assume chess_960 when constructing
        let is_chess_960 = DEFAULT_CASTLES.king_starting_squares != king_starting_squares
            || *DEFAULT_CASTLES.rook_starting_squares.get_direction(CastleDirection::KingSide) != *rook_starting_squares.get_direction(CastleDirection::KingSide)
            || *DEFAULT_CASTLES.rook_starting_squares.get_direction(CastleDirection::QueenSide) != *rook_starting_squares.get_direction(CastleDirection::QueenSide);

        Self {
            rights,
            clear_paths: rook_paths,
            unattacked_paths: king_paths,
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
        #[cfg(not(feature = "chess_960"))]
        { get_unoccupied_path(self.king_from_square(side), self.rook_from_square(side, direction)) }
        #[cfg(feature = "chess_960")]
        { *self.clear_paths.get_direction(direction).get_side(side) }
    }

    /// The mask that must not be attacked for the king to pass through in order to castle for a [player](Player) in a given [direction](CastleDirection)
    pub const fn get_unattacked_path(&self, side: Player, direction: CastleDirection) -> Bitboard {
        #[cfg(not(feature = "chess_960"))]
        { get_unattacked_path(self.king_from_square(side), self.king_to_square(side, direction)) }
        #[cfg(feature = "chess_960")]
        { *self.unattacked_paths.get_direction(direction).get_side(side) }
    }
}

impl const Default for Castles {
    fn default() -> Self {
        #[cfg(feature = "chess_960")]
        let default = Self {
            rights: CastleRights::None,
            clear_paths: create_castle_paths(STANDARD_KING_FROM_SQUARES, STANDARD_ROOK_FROM_SQUARES),
            unattacked_paths: create_king_paths(STANDARD_KING_FROM_SQUARES, STANDARD_KING_TO_SQUARES),
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