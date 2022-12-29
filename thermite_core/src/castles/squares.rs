use crate::castles::by_castle_direction::ByCastleDirection;
use crate::player::ByPlayer;
use crate::square::Square;

pub const STANDARD_KING_FROM_SQUARES: ByPlayer<Square> = ByPlayer::new_with(Square::E1, Square::E8);

pub const STANDARD_KING_TO_SQUARES: ByCastleDirection<ByPlayer<Square>> = ByCastleDirection::new_with(
    ByPlayer::new_with(Square::G1, Square::G8),
    ByPlayer::new_with(Square::C1, Square::C8)
);

pub const STANDARD_ROOK_FROM_SQUARES: ByCastleDirection<ByPlayer<Square>> = ByCastleDirection::new_with(
    ByPlayer::new_with(Square::H1, Square::H8),
    ByPlayer::new_with(Square::A1, Square::A8),
);

pub const STANDARD_ROOK_TO_SQUARES: ByCastleDirection<ByPlayer<Square>> = ByCastleDirection::new_with(
    ByPlayer::new_with(Square::F1, Square::F8),
    ByPlayer::new_with(Square::D1, Square::D8),
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::castles::direction::CastleDirection;
    use crate::player::Player;

    #[test]
    fn default_castle_squares_are_correct() {
        assert_eq!(STANDARD_KING_FROM_SQUARES.get_side(Player::White), &Square::E1);
        assert_eq!(STANDARD_KING_FROM_SQUARES.get_side(Player::Black), &Square::E8);
        assert_eq!(STANDARD_ROOK_FROM_SQUARES.get_direction(CastleDirection::KingSide).get_side(Player::White), &Square::H1);
        assert_eq!(STANDARD_ROOK_FROM_SQUARES.get_direction(CastleDirection::KingSide).get_side(Player::Black), &Square::H8);
        assert_eq!(STANDARD_ROOK_FROM_SQUARES.get_direction(CastleDirection::QueenSide).get_side(Player::White), &Square::A1);
        assert_eq!(STANDARD_ROOK_FROM_SQUARES.get_direction(CastleDirection::QueenSide).get_side(Player::Black), &Square::A8);
        assert_eq!(STANDARD_ROOK_TO_SQUARES.get_direction(CastleDirection::KingSide).get_side(Player::White), &Square::F1);
        assert_eq!(STANDARD_ROOK_TO_SQUARES.get_direction(CastleDirection::KingSide).get_side(Player::Black), &Square::F8);
        assert_eq!(STANDARD_ROOK_TO_SQUARES.get_direction(CastleDirection::QueenSide).get_side(Player::White), &Square::D1);
        assert_eq!(STANDARD_ROOK_TO_SQUARES.get_direction(CastleDirection::QueenSide).get_side(Player::Black), &Square::D8);
    }
}