use crate::bitboard::Bitboard;
use crate::castles::{CastleRights, Castles};
use crate::piece_type::{ByPieceType, PieceType};
use crate::player::{ByPlayer, Player};
use crate::PlyCount;
use crate::square::Square;

/// The internal state for a board that should be kept separate in order to persist it between moves
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub(super) checkers: Bitboard,
    pub(super) pinners_for: ByPlayer<Bitboard>,
    pub(super) blockers_for: ByPlayer<Bitboard>,
    pub(super) check_squares: ByPieceType<Bitboard>,
    captured_piece: Option<PieceType>,
    en_passant_square: Option<Square>,
    pub(super) castles: Castles,
    halfmove_clock: PlyCount,
}

impl State {
    /// State for an empty board
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            checkers: Bitboard::EMPTY,
            pinners_for: ByPlayer::new(Bitboard::EMPTY),
            blockers_for: ByPlayer::new(Bitboard::EMPTY),
            check_squares: ByPieceType::new(Bitboard::EMPTY),
            captured_piece: None,
            en_passant_square: None,
            halfmove_clock: 0,
            castles: Castles::empty(),
        }
    }

    /// Set the [piece](PieceType) [captured](https://www.chessprogramming.org/Captures) from the previous [move](crate::chess_moveChessMove)
    pub(super) const fn set_capture(&mut self, piece: PieceType) {
        self.captured_piece = Some(piece);
    }

    /// Clear the [captured](https://www.chessprogramming.org/Captures) [piece](PieceType) from the previous [move](crate::chess_move::ChessMove)
    pub(super) const fn clear_capture(&mut self) {
        self.captured_piece = None;
    }

    /// Set the [en-passant](https://www.chessprogramming.org/En_passant) [square](Square) from the previous [move](crate::chess_move::ChessMove)
    pub(super) const fn set_en_passant(&mut self, square: Square) {
        debug_assert!(square.rank() == 2 || square.rank() == 5, "illegal en-passant square");
        self.en_passant_square = Some(square);
    }

    /// Clear the [en-passant](https://www.chessprogramming.org/En_passant) [square](Square) from the previous [move](crate::chess_move::ChessMove)
    pub(super) const fn clear_en_passant(&mut self) {
        self.en_passant_square = None;
    }

    /// Set the [castle rights](https://www.chessprogramming.org/Castling_Rights), which controls whether each player has the ability to [castle](https://www.chessprogramming.org/Castling) in a given [direction](crate::castles::CastleDirection)
    pub(super) fn set_castle_rights(&mut self, rights: CastleRights) {
        *self.castles.as_mut() = rights;
    }

    /// Remove a set of [castle rights](https://www.chessprogramming.org/Castling_Rights) from the current rights, which controls whether each player has the ability to [castle](https://www.chessprogramming.org/Castling) in a given [direction](crate::castles::CastleDirection)
    pub(super) fn remove_castle_rights(&mut self, rights: CastleRights) {
        *self.castles.as_mut() = self.castles.as_ref().filter(rights);
    }

    /// Add one to the [half-move](https://www.chessprogramming.org/Halfmove_Clock) clock, indicating another chess move has been completed
    pub(super) const fn increment_halfmove_clock(&mut self) {
        self.halfmove_clock = self.halfmove_clock.saturating_add(1);
    }

    /// Reset the [half-move](https://www.chessprogramming.org/Halfmove_Clock) clock to zero, indicating a pawn push or piece has been captured
    pub(super) const fn clear_halfmove_clock(&mut self) {
        self.halfmove_clock = 0;
    }

    /// Set the [half-move](https://www.chessprogramming.org/Halfmove_Clock) clock to a starting value
    pub(super) const fn set_halfmove_clock(&mut self, value: PlyCount) {
        self.halfmove_clock = value;
    }

    /// Get the [captured](https://www.chessprogramming.org/Captures) [piece](PieceType) from the previous [move](crate::chess_move::ChessMove)
    #[must_use]
    pub const fn captured_piece(&self) -> Option<PieceType> {
        self.captured_piece
    }

    /// Get the previous [move](crate::chess_move::ChessMove)'s [en-passant](https://www.chessprogramming.org/En_passant) [square](Square) if set
    pub const fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    /// Get the current [Castles]
    pub const fn castles(&self) -> Castles {
        self.castles
    }

    /// Get a [mask](Bitboard) of the pieces directly giving checks for the current [side](Player) to move
    pub const fn pieces_giving_check(&self) -> Bitboard {
        self.checkers
    }

    /// Get a [mask](Bitboard) of the [pinning sniper pieces](https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)) giving checks for a given [side](Player)
    pub const fn pinners_for(&self, side: Player) -> Bitboard {
        *self.pinners_for.get_side(side)
    }

    /// Get a [mask](Bitboard) of the [pinned pieces](https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)) blocking checks for a given [side](Player)
    pub const fn blockers_for(&self, side: Player) -> Bitboard {
        *self.blockers_for.get_side(side)
    }

    /// Get a [mask](Bitboard) of the squares that can [give check](https://www.chessprogramming.org/Checks_and_Pinned_Pieces_(Bitboards)#Checks) for a given [piece](PieceType)
    pub const fn check_squares_for(&self, piece_type: PieceType) -> Bitboard {
        *self.check_squares.get_piece(piece_type)
    }

    /// The [half-move](https://www.chessprogramming.org/Halfmove_Clock) counter for keeping track of the [50 move limit](https://www.chessprogramming.org/Fifty-move_Rule)
    pub const fn halfmove_clock(&self) -> PlyCount {
        self.halfmove_clock
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(all(feature = "zobrist", feature = "chess_960"))]
    fn state_size_remains_consistent() {
        assert_eq!(std::mem::size_of::<State>(), 104);
    }
}