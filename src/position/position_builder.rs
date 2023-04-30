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
        self.halfmove_count = PlyCount::new(*fullmove_count.as_ref() * 2);
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

#[cfg(test)]
mod test {
    use crate::castles::CastleRights;
    use crate::half_move_clock::HalfMoveClock;
    use crate::pieces::{PieceType, PlacedPiece};
    use crate::player_color::PlayerColor;
    use crate::ply_count::PlyCount;
    use crate::position::PositionBuilder;
    use crate::square::{Square, Square::*};
    use test_case::test_case;

    #[test_case(CastleRights::None)]
    #[test_case(CastleRights::WhiteKing)]
    #[test_case(CastleRights::WhiteQueen)]
    #[test_case(CastleRights::WhiteBoth)]
    #[test_case(CastleRights::BlackKing)]
    #[test_case(CastleRights::BothKings)]
    #[test_case(CastleRights::WhiteQueenBlackKing)]
    #[test_case(CastleRights::WhiteBothBlackKing)]
    #[test_case(CastleRights::BlackQueen)]
    #[test_case(CastleRights::WhiteKingBlackQueen)]
    #[test_case(CastleRights::BothQueens)]
    #[test_case(CastleRights::WhiteBothBlackQueen)]
    #[test_case(CastleRights::BlackBoth)]
    #[test_case(CastleRights::WhiteKingBlackBoth)]
    #[test_case(CastleRights::WhiteQueenBlackBoth)]
    #[test_case(CastleRights::All)]
    fn with_castle_rights_works(input: CastleRights) {
        let pos = PositionBuilder::default().with_castle_rights(input);
        assert_eq!(pos.castle_rights, input);
    }

    #[test_case(PieceType::Pawn, PlayerColor::White, Square::E2)]
    #[test_case(PieceType::Pawn, PlayerColor::Black, Square::E7)]
    #[test_case(PieceType::Knight, PlayerColor::White, Square::F4)]
    #[test_case(PieceType::Queen, PlayerColor::Black, Square::G2)]
    #[test_case(PieceType::King, PlayerColor::Black, Square::G8)]
    #[test_case(PieceType::Pawn, PlayerColor::Black, Square::E2)]
    #[test_case(PieceType::Pawn, PlayerColor::White, Square::E7)]
    #[test_case(PieceType::Knight, PlayerColor::Black, Square::F4)]
    #[test_case(PieceType::Queen, PlayerColor::White, Square::G2)]
    #[test_case(PieceType::King, PlayerColor::White, Square::G8)]
    fn with_piece_works(piece: PieceType, player: PlayerColor, square: Square) {
        let owned = piece.owned_by(player);
        let placed = owned.placed_on(square);
        let pos = PositionBuilder::default().with_piece(placed);
        assert_eq!(pos.squares[square], Some(owned));
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    #[test_case(4)]
    #[test_case(5)]
    #[test_case(6)]
    #[test_case(7)]
    #[test_case(20)]
    #[test_case(50)]
    fn with_halfmove_clock_works(input: u8) {
        let input = HalfMoveClock::new(PlyCount::new(input)).unwrap();
        let pos = PositionBuilder::default().with_halfmove_clock(input);
        assert_eq!(pos.halfmove_clock, input);
    }

    #[test_case(0, PlayerColor::White, 0)]
    #[test_case(0, PlayerColor::Black, 0)]
    #[test_case(1, PlayerColor::White, 2)]
    #[test_case(1, PlayerColor::Black, 2)]
    #[test_case(2, PlayerColor::White, 4)]
    #[test_case(2, PlayerColor::Black, 4)]
    #[test_case(30, PlayerColor::White, 60)]
    #[test_case(30, PlayerColor::Black, 60)]
    #[test_case(50, PlayerColor::White, 100)]
    #[test_case(50, PlayerColor::Black, 100)]
    fn with_fullmove_count_works(
        input_full_moves: u8,
        player: PlayerColor,
        expected_half_move_count: u8,
    ) {
        let input_full_moves = PlyCount::new(input_full_moves);
        let expected_half_move_count = PlyCount::new(expected_half_move_count);
        let pos = PositionBuilder::default()
            .with_starting_player(player)
            .with_fullmove_count(input_full_moves);
        assert_eq!(pos.halfmove_count, expected_half_move_count);
    }

    #[test_case(A3)]
    #[test_case(B3)]
    #[test_case(C3)]
    #[test_case(D3)]
    #[test_case(E3)]
    #[test_case(F3)]
    #[test_case(G3)]
    #[test_case(H3)]
    #[test_case(C6)]
    #[test_case(D6)]
    #[test_case(E6)]
    #[test_case(F6)]
    #[test_case(G6)]
    #[test_case(H6)]
    fn with_en_passant_square_works(square: Square) {
        let square = square.try_into().unwrap();
        let pos = PositionBuilder::default().with_en_passant_square(square);
        assert_eq!(pos.en_passant_square, Some(square));
    }
}
