use std::str::FromStr;

use crate::{PlyCount, STANDARD_MOVE_CLOCK};
use crate::board::Board;
#[cfg(feature = "chess_960")]
use crate::castles::Castles;
use crate::castles::CastleRights;
use crate::piece_type::PieceType;
use crate::player::Player;
use crate::sided_piece::SidedPiece;
use crate::square::{NUM_FILES, NUM_RANKS, Square};

/// Errors that can occur while parsing a FEN string.  Typically if unable to parse or it represents an invalid chess position.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FenParseError {
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
    IllegalSideChar(char),
    /// If the en-passant square is not a valid en-passant square (ranks 3 and 6) or cannot be parsed.
    IllegalEnPassant,
    /// If the castle rights are not a valid UCI representation (`-`, `KQkq`, `KQk`, `k`, etc)
    IllegalCastleRights,
    /// If the halfmove clock isn't a valid number or is out of bounds
    IllegalHalfmoveClock,
    /// If the full move counter isn't a valid number or is out of bounds
    IllegalFullmoveCounter,
}

impl Board {
    /// TODO
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_fen_string(&self) -> String {
        let mut fen = String::with_capacity(84);
        for rank in (0..NUM_RANKS).rev() {
            let mut row_offset = 0u8;
            for file in 0..NUM_FILES {
                #[allow(clippy::cast_possible_truncation)]
                let square = Square::try_from((rank * NUM_FILES + file) as u8).unwrap();
                if let Some(piece) = self.piece_on(square) {
                    if row_offset > 0 {
                        #[allow(clippy::cast_lossless)]
                        fen.push(char::from_digit(row_offset as u32, 10).unwrap());
                        row_offset = 0;
                    }
                    fen.push(if self.side_on(square).unwrap() == Player::White { piece.get_upper_char() } else { piece.get_lower_char() });
                } else {
                    if square.file() == NUM_FILES - 1 {
                        #[allow(clippy::cast_lossless)]
                        fen.push(char::from_digit(row_offset as u32 + 1, 10).unwrap());
                        break;
                    }
                    row_offset += 1;
                }
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push_str(self.side_to_move().to_string().as_str());
        fen.push(' ');
        fen.push_str(self.as_ref().castles().as_ref().to_string().as_str());
        fen.push(' ');
        if let Some(en_passant_square) = self.as_ref().en_passant_square() {
            fen.push_str(en_passant_square.to_string().as_str());
        } else {
            fen.push('-');
        }
        fen.push(' ');
        fen.push_str(self.as_ref().halfmove_clock().to_string().as_str());
        fen.push(' ');
        fen.push_str(self.fullmove_count().to_string().as_str());

        fen
    }

    /// Attempt to parse a [Board] from a [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) string
    ///
    /// # Errors
    /// TODO
    #[allow(clippy::cognitive_complexity)]
    pub fn from_fen(fen: &str) -> Result<Self, FenParseError> {
        let mut fen_chunks = fen.split_ascii_whitespace().fuse();

        let mut board = fen_chunks.next()
            .ok_or(FenParseError::MissingPosition)?
            .chars()
            .try_fold((Self::empty_position(), Square::A8), |(mut board, mut square), token| {
                if let Some(digit) = token.to_digit(10) {
                    square = square.checked_add(digit as u8).ok_or(FenParseError::InvalidBoardDimensions)?;// TODO: XXX: BUG: BROKEN?
                } else if token == '/' {
                    square = square.checked_sub(NUM_FILES as u8).ok_or(FenParseError::InvalidBoardDimensions)?;// TODO: XXX: BUG: BROKEN?
                } else if let Ok(SidedPiece { player, piece_type }) = SidedPiece::try_from(token) {
                    board.add_piece(square, SidedPiece::new(piece_type, player));
                    square = square.checked_add(1).ok_or(FenParseError::InvalidBoardDimensions)?;// TODO: XXX: BUG: BROKEN?
                }

                Ok((board, square))
            })?
            .0;

        // Switch sides if not white starting
        match fen_chunks.next()
            .ok_or(FenParseError::MissingSide)?
            .chars()
            .next()
            .ok_or(FenParseError::MissingSide)?
        {
            'w' | 'W' => {},
            'b' | 'B' => board.switch_sides(),
            side_char => return Err(FenParseError::IllegalSideChar(side_char)),
        };

        // Update castle rights
        board.state.set_castle_rights(CastleRights::from_str(fen_chunks.next().ok_or(FenParseError::MissingCastleRights)?).ok().ok_or(FenParseError::IllegalCastleRights)?);

        // Set en-passant square
        board.state.set_en_passant(Square::from_str(fen_chunks.next().ok_or(FenParseError::MissingEnPassant)?).ok().ok_or(FenParseError::MissingEnPassant)?);

        // Set half-move clock if possible
        if let Some(half_move_chunk) = fen_chunks.next() {
            board.state.set_halfmove_clock(half_move_chunk.parse().ok().filter(|&m| m < STANDARD_MOVE_CLOCK).ok_or(FenParseError::IllegalHalfmoveClock)?);
        }

        // Set the full-move counter if possible
        if let Some(full_move_count_chunk) = fen_chunks.next() {
            let full_moves: PlyCount = full_move_count_chunk.parse().ok().ok_or(FenParseError::IllegalFullmoveCounter)?;
            board.halfmove_count = full_moves * 2 + if board.side_to_move == Player::Black { 1 } else { 0 };
        }

        // Update check masks
        board.switch_sides();
        board.update_checkers(true);
        board.switch_sides();

        // Update checkers, pinners, pinned, and other move gen state
        board.update_move_gen_masks();

        // Check if rooks are in unconventional location (indicating chess 960, if so enable chess 960)
        #[cfg(feature = "chess_960")]
        if !board.state.castles().eq_starting_squares(Castles::empty()) {
            board.state.castles.set_chess_960(true);
        }

        Ok(board)
    }
}

impl FromStr for Board {
    type Err = FenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_fen(s)
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; "startpos")]
    #[test_case("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"; "kiwipete")]
    #[test_case("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"; "position_3")]
    #[test_case("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"; "position_4")]
    #[test_case("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1"; "position_4_mirrored")]
    #[test_case("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"; "position_5")]
    #[test_case("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"; "position_6")]
    fn from_to_fen_is_symmetric(fen: &str) {
        assert_eq!(Board::from_fen(fen).expect("illegal FEN").get_fen_string().as_str(), fen);
    }
}