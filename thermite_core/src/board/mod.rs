use std::fmt::{Debug, Formatter};
#[cfg(feature = "zobrist")]
use std::hash::Hasher;
use derive_more::{AsMut, AsRef};

use crate::bitboard::Bitboard;
#[cfg(feature = "zobrist")]
use crate::board::zobrist::{ZobristHasher, ZobristInner};
use crate::board::state::State;
use crate::castles::CastleRights;
use crate::chess_move::ChessMove;
use crate::move_type::MoveType;
use crate::piece_type::{ByPieceType, NUM_PIECE_TYPES, PieceType};
use crate::player::{ByPlayer, NUM_PLAYERS, Player};
use crate::PlyCount;
#[cfg(feature = "material_eval")]
use crate::score::SidedPieceCounts;
#[cfg(feature = "piece_square_eval")]
use crate::score::TaperedPawnApproximationEvaluation;
use crate::sided_piece::SidedPiece;
use crate::square::{BySquare, NUM_FILES, NUM_RANKS, Square};

mod state;
mod make_move;
mod unmake_move;
/// FEN string parsing to serialize and deserialize a board
pub mod fen;
/// Zobrist hashing for matching board transpositions, or positions with the same piece arrangement and game state (side to move, en-passant, castle rights).
#[cfg(feature = "zobrist")]
pub mod zobrist;

/// A snapshot of a game of chess, containing board state for resuming any chess position.
/// Positional state like piece placements, the side to move.
/// As well as game [state](State) like, the halfmove-clock (for 50 move limit), pieces giving check, en-passant square, previously captured piece, etc.
///
/// With appropriate features enabled, will also incrementally keep track of certain evaluation parameters.
/// - `piece_square_eval` - To keep track of the positional piece square evaluation, [`TaperedPawnApproximationEvaluation`].
/// - `material_eval` - To keep track of the material count by piece for quick evaluation.
/// - `chess_960` - Internal flag to keep track if the position was created as a variant of chess 960.
#[derive(Copy, Clone, Eq, PartialEq, AsRef, AsMut)]
pub struct Board {
    /// By each square whether or not it contains a piece
    piece_squares: BySquare<Option<PieceType>>,
    /// By each piece the mask of where every matching piece is on the board
    piece_masks: ByPieceType<Bitboard>,
    /// By each player, where every matching piece for that side is on the board
    side_masks: ByPlayer<Bitboard>,
    /// The player who's turn it is to move
    side_to_move: Player,
    /// How many moves have been played
    halfmove_count: PlyCount,
    /// State that should be preserved between moves
    #[as_ref]
    #[as_mut]
    pub(crate) state: State,

    /// The positional hash for finding transpositions
    #[cfg(feature = "zobrist")]
    pub(super) hasher: ZobristHasher,

    /// The piece counts for material evaluation
    #[cfg(feature = "material_eval")]
    pub sided_piece_counts: SidedPieceCounts,

    #[cfg(feature = "piece_square_eval")]
    piece_square_eval: TaperedPawnApproximationEvaluation,
}

impl Board {
    /// Empty (illegal because no kings), starting position with white to move
    #[must_use]
    pub fn empty_position() -> Self {
        Self {
            piece_squares: BySquare::new(None),
            piece_masks: ByPieceType::new(Bitboard::EMPTY),
            side_masks: ByPlayer::new(Bitboard::EMPTY),
            side_to_move: Player::White,
            state: State::empty(),
            halfmove_count: 0,
            #[cfg(feature = "zobrist")]
            hasher: ZobristHasher::empty(),
            #[cfg(feature = "piece_square_eval")]
            piece_square_eval: TaperedPawnApproximationEvaluation::EMPTY,
            #[cfg(feature = "material_eval")]
            sided_piece_counts: SidedPieceCounts::empty(),
        }
    }

    /// Classical chess starting position with white to move
    #[must_use]
    pub fn starting_position() -> Self {
        let mut board = Self::empty_position();

        // Add white's pieces
        board.add_piece(Square::A2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::B2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::C2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::D2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::E2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::F2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::G2, SidedPiece::new(PieceType::Pawn, Player::White));
        board.add_piece(Square::H2, SidedPiece::new(PieceType::Pawn, Player::White));

        board.add_piece(Square::A1, SidedPiece::new(PieceType::Rook, Player::White));
        board.add_piece(Square::H1, SidedPiece::new(PieceType::Rook, Player::White));

        board.add_piece(Square::B1, SidedPiece::new(PieceType::Knight, Player::White));
        board.add_piece(Square::G1, SidedPiece::new(PieceType::Knight, Player::White));

        board.add_piece(Square::C1, SidedPiece::new(PieceType::Bishop, Player::White));
        board.add_piece(Square::F1, SidedPiece::new(PieceType::Bishop, Player::White));

        board.add_piece(Square::D1, SidedPiece::new(PieceType::Queen, Player::White));

        board.add_piece(Square::E1, SidedPiece::new(PieceType::King, Player::White));

        // Add black's pieces
        board.add_piece(Square::A7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::B7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::C7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::D7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::E7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::F7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::G7, SidedPiece::new(PieceType::Pawn, Player::Black));
        board.add_piece(Square::H7, SidedPiece::new(PieceType::Pawn, Player::Black));

        board.add_piece(Square::A8, SidedPiece::new(PieceType::Rook, Player::Black));
        board.add_piece(Square::H8, SidedPiece::new(PieceType::Rook, Player::Black));

        board.add_piece(Square::B8, SidedPiece::new(PieceType::Knight, Player::Black));
        board.add_piece(Square::G8, SidedPiece::new(PieceType::Knight, Player::Black));

        board.add_piece(Square::C8, SidedPiece::new(PieceType::Bishop, Player::Black));
        board.add_piece(Square::F8, SidedPiece::new(PieceType::Bishop, Player::Black));

        board.add_piece(Square::D8, SidedPiece::new(PieceType::Queen, Player::Black));

        board.add_piece(Square::E8, SidedPiece::new(PieceType::King, Player::Black));

        // Update state
        board.state.set_castle_rights(CastleRights::All);
        board.update_move_gen_masks();

        board
    }

    /// Switch the side to move to the next player
    pub fn switch_sides(&mut self) {
        self.side_to_move = self.side_to_move.switch();
        #[cfg(feature = "zobrist")]
        self.hasher.switch_sides();
    }

    /// Get the player who's turn it is to move
    #[must_use]
    pub const fn side_to_move(&self) -> Player {
        self.side_to_move
    }

    /// Get the current fullmove the game has been played to
    #[must_use]
    pub const fn fullmove_count(&self) -> PlyCount {
        self.halfmove_count / 2
    }

    /// Get a mask of the occupied squares for a given side
    pub const fn side_mask(&self, side: Player) -> Bitboard {
        *self.side_masks.get_side(side)
    }

    /// Get a mask of the squares with specific pieces placed on them
    pub const fn piece_mask(&self, piece: PieceType) -> Bitboard {
        *self.piece_masks.get_piece(piece)
    }

    /// Get a mask of the squares with multiple types of pieces placed on them
    pub fn pieces_mask<const N: usize>(&self, pieces: [PieceType; N]) -> Bitboard {
        let mut pieces_mask = Bitboard::EMPTY;
        let mut piece_index = 0;
        while piece_index < N {
            pieces_mask |= *self.piece_masks.get_piece(pieces[piece_index]);
            piece_index += 1;
        }

        pieces_mask
    }

    /// Get a mask of all occupied squares on the board
    pub fn occupied(&self) -> Bitboard {
        self.side_mask(Player::White) | self.side_mask(Player::Black)
    }

    /// The computed [Zobrist hash](https://www.chessprogramming.org/Zobrist_Hashing) key
    #[must_use]
    #[cfg(feature = "zobrist")]
    pub fn zobrist_key(&self) -> ZobristInner {
        self.hasher.finish()
    }

    /// Compute a mask of attackers for all sides that can target a given square, given a set of blockers
    pub fn attackers_to(&self, square: Square, occupied: Bitboard) -> Bitboard {
        let square_mask = square.to_mask();

        let pawns = self.piece_mask(PieceType::Pawn);
        let white_pawns = pawns & self.side_mask(Player::White);
        let black_pawns = pawns & self.side_mask(Player::Black);
        let knights = self.piece_mask(PieceType::Knight);
        let rooks_and_queens = self.pieces_mask([PieceType::Rook, PieceType::Queen]);
        let bishops_and_queens = self.pieces_mask([PieceType::Bishop, PieceType::Queen]);
        let kings = self.piece_mask(PieceType::King);

        let white_pawn_attacks = square_mask.pawn_attacks(Player::Black) & white_pawns;
        let black_pawn_attacks = square_mask.pawn_attacks(Player::White) & black_pawns;
        let knight_attacks = square_mask.knight_attacks() & knights;
        let rook_attacks = square_mask.cardinal_sliding_attacks(occupied) & rooks_and_queens;
        let bishop_attacks = square_mask.ordinal_sliding_attacks(occupied) & bishops_and_queens;
        let king_attacks = square_mask.king_attacks() & kings;

        white_pawn_attacks | black_pawn_attacks | knight_attacks | rook_attacks | bishop_attacks | king_attacks
    }

    /// Get the piece, if any, for a given square
    #[must_use]
    pub fn piece_on(&self, square: Square) -> Option<PieceType> {
        let square_mask = square.to_mask();
        PieceType::PIECES.into_iter()
            .find(|&p| !(self.piece_mask(p) & square_mask).is_empty())
    }

    /// Get the side, if any, for a given square
    #[must_use]
    pub fn side_on(&self, square: Square) -> Option<Player> {
        let square_mask = square.to_mask();
        Player::PLAYERS.into_iter()
            .find(|&p| !(self.side_mask(p) & square_mask).is_empty())
    }

    /// Get the square that a player's king resides on
    #[must_use]
    #[inline]
    pub fn king_square(&self, side: Player) -> Option<Square> {
        let king_mask = self.piece_mask(PieceType::King) & self.side_mask(side);

        #[allow(clippy::cast_possible_truncation)]
        Square::try_from(king_mask.0.trailing_zeros() as u8).ok()
    }

    #[inline]
    fn is_blocker(&self, defending_side: Player, blocker_square: Square) -> bool {
        (*self.state.blockers_for.get_side(defending_side) & blocker_square.to_mask()).is_empty()
    }

    #[inline]
    fn is_discovery(&self, defending_side: Player, from_square: Square, to_square: Square) -> bool {
        let king_square = self.king_square(defending_side).expect("missing king in legal position");

        self.is_blocker(defending_side, from_square) && !Bitboard::is_aligned(from_square, to_square, king_square)
    }

    #[inline]
    fn is_direct_check(&self, piece_type: PieceType, attacking_square: Square) -> bool {
        !(self.state.check_squares_for(piece_type) & attacking_square.to_mask()).is_empty()
    }

    /// Check if a given legal move will give check
    #[must_use]
    pub fn gives_check(&self, chess_move: ChessMove) -> bool {
        let defending_side = self.side_to_move.switch();
        let ChessMove { move_type, from, to } = chess_move;

        match move_type {
            MoveType::Quiet { piece_type } | MoveType::Capture { piece_type, .. } => {
                self.is_direct_check(piece_type, to)
                    || self.is_discovery(defending_side, from, to)
            }
            MoveType::DoublePawnPush { .. } => {
                self.is_direct_check(PieceType::Pawn, to)
                    || self.is_discovery(defending_side, from, to)
            }
            MoveType::EnPassantCapture { captured_pawn_square } => {
                let attackers = self.side_mask(self.side_to_move);
                let defending_king_square = self.king_square(defending_side).expect("missing king in legal position");
                let occupied = self.occupied() ^ from.to_mask() ^ captured_pawn_square.to_mask() | to.to_mask();
                let cardinal_attackers = self.pieces_mask([PieceType::Queen, PieceType::Rook]) & attackers;
                let ordinal_attackers = self.pieces_mask([PieceType::Queen, PieceType::Bishop]) & attackers;
                let cardinal_checks = Bitboard::occluded_attacks_mask(PieceType::Rook, defending_king_square, occupied) & cardinal_attackers;
                let ordinal_checks = Bitboard::occluded_attacks_mask(PieceType::Bishop, defending_king_square, occupied) & ordinal_attackers;

                !(cardinal_checks | ordinal_checks).is_empty()
                    || self.is_direct_check(PieceType::Pawn, to)
                    || self.is_discovery(defending_side, from, to)
            }
            MoveType::Castle { castle_direction } => {
                let defending_king_mask = self.piece_mask(PieceType::King) & self.side_mask(defending_side);
                let rook_to_square = self.as_ref().castles().rook_to_square(self.side_to_move, castle_direction);
                let occupied = self.occupied() ^ from.to_mask() ^ to.to_mask();
                let rook_attacks = Bitboard::occluded_attacks_mask(PieceType::Rook, rook_to_square, occupied) & defending_king_mask;

                !rook_attacks.is_empty()
            }
            MoveType::Promotion { promotion } | MoveType::PromotingCapture { promotion, .. } => {
                let defending_king_mask = self.piece_mask(PieceType::King) & self.side_mask(defending_side);
                let promotion_attacks = Bitboard::occluded_attacks_mask(promotion.into(), to, self.occupied() ^ from.to_mask()) & defending_king_mask;

                !promotion_attacks.is_empty()
                    || self.is_discovery(defending_side, from, to)
            }
        }
    }

    /// If the current [player](crate::Player) is in check
    #[must_use]
    pub const fn in_check(&self) -> bool {
        !self.state.checkers.is_empty()
    }

    /// Mask of pieces giving the current [player](crate::Player) check
    pub const fn checkers(&self) -> Bitboard {
        self.state.checkers
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f)?;
            writeln!(f, "  A B C D E F G H  ")?;
            for rank in (0..NUM_RANKS).rev() {
                write!(f, "{} ", rank + 1)?;
                for file in 0..NUM_FILES {
                    #[allow(clippy::cast_possible_truncation)]
                        let square = Square::try_from((rank * NUM_FILES + file) as u8).map_err(|_| std::fmt::Error)?;
                    write!(
                        f,
                        "{} ",
                        self.piece_on(square).map_or_else(
                            || '-',
                            |p| match self.side_on(square).unwrap() {
                                Player::White => p.get_upper_char(),
                                Player::Black => p.get_lower_char(),
                            },
                        )
                    )?;
                }
                write!(f, "{}", rank + 1)?;

                match rank {
                    6 => write!(f, "\t\tFEN: {}", self.get_fen_string())?,
                    4 => write!(f, "\t\tCheckers: {}", self.checkers().pop_square().map_or_else(|| "-".into(), |s| s.to_string()))?,
                    #[cfg(feature = "zobrist")]
                    3 => write!(f, "\t\tHash: {:#X}", self.zobrist_key())?,
                    #[cfg(feature = "material_eval")]
                    2 => write!(f, "\t\tGame Stage: {:<.3}", self.sided_piece_counts.game_stage().0)?,
                    #[cfg(all(feature = "piece_square_eval", feature = "material_eval"))]
                    1 => write!(f, "\t\tPSQ Eval: {}", self.piece_square_eval.evaluate(self.sided_piece_counts.game_stage()))?,
                    _ => {}
                }

                writeln!(f)?;
            }
            writeln!(f, "  A B C D E F G H  ")
        } else {
            f.write_str(self.get_fen_string().as_str())
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use test::{Bencher, black_box};

    use test_case::test_case;

    use crate::board::Board;
    use crate::chess_move::ChessMove;
    use crate::move_type::MoveType;
    use crate::square::Square;

    use super::*;

    #[test_case("8/2p5/3p4/1P6/K3Pp1r/6k1/6P1/1R6 b - e3 0 3", ChessMove { from: Square::F4, to: Square::E3, move_type: MoveType::EnPassantCapture { captured_pawn_square: Square::E4 }}, true; "discovered en-passant capture")]
    #[test_case("r7/8/2pp3Q/p7/1qP1P1k1/8/P4PK1/8 w - - 0 3", ChessMove { from: Square::F2, to: Square::F3, move_type: MoveType::Quiet { piece_type: PieceType::Pawn } }, true)]
    fn gives_check_works(fen: &str, chess_move: ChessMove, expected: bool) {
        assert_eq!(Board::from_fen(fen).expect("illegal FEN").gives_check(chess_move), expected);
    }

    #[bench]
    fn gives_check_startpos_e2e4_bench(bencher: &mut Bencher) {
        let chess_move = black_box(ChessMove { from: Square::E2, to: Square::E4, move_type: MoveType::DoublePawnPush { en_passant_square: Square::E3 } });
        bencher.iter(|| assert_eq!(black_box(Board::starting_position()).gives_check(chess_move), black_box(false)));
    }

    #[bench]
    fn king_square_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| assert_eq!(Board::king_square(&black_box(Board::starting_position()), black_box(Player::Black)), black_box(Some(Square::E8))));
    }

    #[bench]
    fn is_blocker_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| assert_eq!(Board::is_blocker(&black_box(Board::starting_position()), black_box(Player::Black), black_box(Square::E2)), black_box(false)));
    }

    #[bench]
    fn is_discovery_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| assert_eq!(Board::is_discovery(&black_box(Board::starting_position()), black_box(Player::Black), black_box(Square::E2), black_box(Square::E4)), black_box(false)));
    }

    #[bench]
    fn is_direct_check_startpos_bench_blackboxed_pawn(bencher: &mut Bencher) {
        bencher.iter(|| assert_eq!(Board::is_direct_check(&black_box(Board::starting_position()), black_box(PieceType::Pawn), black_box(Square::E4)), black_box(false)));
    }

    #[bench]
    fn is_direct_check_startpos_bench(bencher: &mut Bencher) {
        bencher.iter(|| assert_eq!(Board::is_direct_check(&black_box(Board::starting_position()), PieceType::Pawn, black_box(Square::E4)), black_box(false)));
    }
}
