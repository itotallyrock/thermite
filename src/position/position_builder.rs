use crate::castles::CastleRights;
use crate::half_move_clock::HalfMoveClock;
use crate::pieces::{OwnedPiece, Piece, PieceType, PlacedPiece};
use crate::player_color::PlayerColor;
use crate::ply_count::PlyCount;
use crate::square::{EnPassantSquare, File, Square};
use core::str::FromStr;
use enum_map::{Enum, EnumMap};

/// Allows setting up a board and performing pseudo-legal moves without checking legality.
/// With the end goal being to [`convert`](std::convert) this into a [`LegalPosition`](position::LegalPosition)
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

/// Errors that can occur while parsing a FEN string.  Typically if unable to parse or it represents an invalid chess position.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FenParseError {
    /// Contains an illegal or unexpected characater
    InvalidChar,
    /// If the FEN string is missing the position (an empty string)
    MissingPosition,
    /// Missing the side to move, 'w' or 'b' after the position.
    MissingSide,
    /// Missing castle rights, 'KQkq', 'Kq', etc, '-' after side to move.
    MissingCastleRights,
    /// Missing the en-passant square after the castle rights
    MissingEnPassant,
    /// Position segment contained more rows or columns than expected
    InvalidBoardDimensions,
    /// Side to move segment wasn't a valid side to move 'w' or 'b'
    IllegalSideChar,
    /// If the en-passant square is not a valid en-passant square (ranks 3 and 6) or cannot be parsed.
    IllegalEnPassant,
    /// If the castle rights are not a valid UCI representation (`-`, `KQkq`, `KQk`, `k`, etc)
    IllegalCastleRights,
    /// If the halfmove clock isn't a valid number or is out of bounds
    IllegalHalfmoveClock,
    /// If the full move counter isn't a valid number or is out of bounds
    IllegalFullmoveCounter,
}

/// A parsed positional char from a FEN string
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FenPositionChar {
    /// A number of squares within a rank to skip before the next piece
    RankOffset(u8),
    /// A slash indicating the end of the current rank
    NextRank,
    /// A [player](PlayerColor)s [piece](PlacedPiece) on the board
    Piece(OwnedPiece),
}

impl TryFrom<char> for FenPositionChar {
    type Error = FenParseError;

    fn try_from(fen_char: char) -> Result<Self, Self::Error> {
        match fen_char {
            '0'..='9' => Ok(Self::RankOffset(fen_char.to_digit(10).unwrap() as u8)),
            '/' => Ok(Self::NextRank),
            'P' => Ok(Self::Piece(PieceType::Pawn.owned_by(PlayerColor::White))),
            'p' => Ok(Self::Piece(PieceType::Pawn.owned_by(PlayerColor::Black))),
            'N' => Ok(Self::Piece(PieceType::Knight.owned_by(PlayerColor::White))),
            'n' => Ok(Self::Piece(PieceType::Knight.owned_by(PlayerColor::Black))),
            'B' => Ok(Self::Piece(PieceType::Bishop.owned_by(PlayerColor::White))),
            'b' => Ok(Self::Piece(PieceType::Bishop.owned_by(PlayerColor::Black))),
            'R' => Ok(Self::Piece(PieceType::Rook.owned_by(PlayerColor::White))),
            'r' => Ok(Self::Piece(PieceType::Rook.owned_by(PlayerColor::Black))),
            'Q' => Ok(Self::Piece(PieceType::Queen.owned_by(PlayerColor::White))),
            'q' => Ok(Self::Piece(PieceType::Queen.owned_by(PlayerColor::Black))),
            'K' => Ok(Self::Piece(PieceType::King.owned_by(PlayerColor::White))),
            'k' => Ok(Self::Piece(PieceType::King.owned_by(PlayerColor::Black))),
            _ => Err(FenParseError::InvalidChar),
        }
    }
}

/// Parse a known legal FEN into a [`LegalPosition`](position::LegalPosition), panicking otherwise
#[macro_export]
macro_rules! fen {
    ($fen:expr) => {{
        use $crate::position::{LegalPosition, PositionBuilder};
        let position_builder = $fen.parse::<PositionBuilder>().unwrap();

        LegalPosition::try_from(position_builder).unwrap()
    }};
}

impl FromStr for PositionBuilder {
    type Err = FenParseError;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut fen_chunks = fen.split_ascii_whitespace().fuse();
        let builder = fen_chunks
            .next()
            .ok_or(FenParseError::MissingPosition)?
            .chars()
            .map_while(|c| FenPositionChar::try_from(c).ok())
            .try_fold(
                (Self::default(), Square::A8 as u8),
                |(mut builder, mut square_offset), fen_char| {
                    const RANK_OFFSET: u8 = 2 * (File::LENGTH as u8);
                    match fen_char {
                        FenPositionChar::RankOffset(offset) => {
                            square_offset += offset;
                        }
                        FenPositionChar::NextRank => {
                            square_offset -= RANK_OFFSET;
                        }
                        FenPositionChar::Piece(piece) => {
                            let placed_piece = piece.placed_on(
                                Square::try_from(square_offset)
                                    .ok()
                                    .ok_or(FenParseError::InvalidBoardDimensions)?,
                            );
                            builder = builder.with_piece(placed_piece);
                            square_offset += 1;
                        }
                    }

                    Ok((builder, square_offset))
                },
            )
            .map(|(builder, _)| builder)?;

        // Read side to move
        let starting_player = fen_chunks.next().map_or_else(
            || Err(FenParseError::MissingSide),
            |s| match s {
                "w" | "W" => Ok(PlayerColor::White),
                "b" | "B" => Ok(PlayerColor::Black),
                _ => Err(FenParseError::IllegalSideChar),
            },
        )?;
        let mut builder = builder.with_starting_player(starting_player);

        // Read castle rights
        if let Some(castle_rights) = fen_chunks
            .next()
            .and_then(|s| s.parse::<CastleRights>().ok())
        {
            builder = builder.with_castle_rights(castle_rights);
        }

        // Read en passant square
        if let Some(en_passant_square) = fen_chunks
            .next()
            .and_then(|s| s.parse::<Square>().ok())
            .and_then(|s| EnPassantSquare::try_from(s).ok())
        {
            builder = builder.with_en_passant_square(en_passant_square);
        }

        // Read half move clock
        if let Some(halfmove_clock) = fen_chunks
            .next()
            .and_then(|s| s.parse::<u8>().ok())
            .map(PlyCount::new)
            .and_then(|s| HalfMoveClock::new(s).ok())
        {
            builder = builder.with_halfmove_clock(halfmove_clock);
        }

        // Read full move counter
        if let Some(fullmove_count) = fen_chunks
            .next()
            .and_then(|s| s.parse::<u8>().ok())
            .map(PlyCount::new)
        {
            builder = builder.with_fullmove_count(fullmove_count);
        }

        Ok(builder)
    }
}

#[cfg(test)]
mod test {
    use crate::castles::CastleRights;
    use crate::half_move_clock::HalfMoveClock;
    use crate::pieces::{Piece, PieceType};
    use crate::player_color::PlayerColor;
    use crate::ply_count::PlyCount;
    use crate::position::PositionBuilder;
    use crate::square::{EnPassantSquare, Square, Square::*};
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

    #[test_case("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/6rP/8 w - - 1 2", G3, G7)]
    fn from_fen_sets_king_squares_correctly(
        fen: &str,
        white_king_square: Square,
        black_king_square: Square,
    ) {
        let position = fen!(fen);
        assert_eq!(position.king_squares[PlayerColor::White], white_king_square);
        assert_eq!(position.king_squares[PlayerColor::Black], black_king_square);
    }

    #[test_case("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/6rP/8 w - - 1 2", None)]
    #[test_case(
        "8/2q3kp/6p1/3BpP2/8/Q3B1K1/1r5P/8 w - e6 0 1",
        Some(EnPassantSquare::E6)
    )]
    fn from_fen_sets_en_passant_correctly(fen: &str, en_passant_square: Option<EnPassantSquare>) {
        let position = fen!(fen);
        assert_eq!(position.state.en_passant_square, en_passant_square);
    }
}
