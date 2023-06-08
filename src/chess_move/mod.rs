use crate::castles::{CastleDirection, KING_FROM_SQUARES, KING_TO_SQUARES};
use crate::chess_move::promotion::Promotion;
use crate::pieces::{NonKingPieceType, Piece};
use crate::player_color::PlayerColor;
use crate::square::Square;
use core::fmt::{Display, Formatter};
use double_pawn_push::DoublePawnPush;
use en_passant_capture::EnPassantCapture;
use quiet::QuietMove;

/// A valid double pushing pawn for a [chess move](ChessMove)
pub mod double_pawn_push;
/// A valid capture of a pawn on its skipped square for a pawn that *just* [double jumped](DoublePawnPush)
pub mod en_passant_capture;
/// A valid pawn promotion for a [chess move](ChessMove)
pub mod promotion;
/// A valid plain chess move, take a piece from a square and move it to another square for a [chess move](ChessMove)
pub mod quiet;

/// The different types of chess moves and the relevant metadata to make (or undo) them
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ChessMove {
    /// Plain chess move, take a piece from a square and move it to another square
    Quiet {
        /// The inner quiet move
        quiet: QuietMove,
    },
    /// Special starting rank unobstructed two square pawn push
    DoublePawnPush {
        /// The inner double pawn push
        pawn_push: DoublePawnPush,
    },
    /// Simple capturing chess move, capture a piece on the target square moving the piece from the to square to the target square
    Capture {
        /// The inner quiet move
        quiet: QuietMove,
        /// The piece being captured
        captured_piece: NonKingPieceType,
    },
    /// Capture a pawn on its skipped square for a pawn that just double jumped
    EnPassantCapture {
        /// The inner capture
        capture: EnPassantCapture,
    },
    /// Castle, or swap the rook and king shifting the king towards the center if both have not been moved yet
    Castle {
        /// The direction the king is castling towards
        castle_direction: CastleDirection,
        /// The player doing the castling
        player: PlayerColor,
    },
    /// Push a pawn to the opposite side's back rank to upgrade the pawn to a [`PromotablePieceType`](crate::pieces::PromotablePieceType)
    Promotion {
        /// The inner promotion
        promotion: Promotion,
    },
    /// Capture on the last rank for promoting a pawn
    PromotingCapture {
        /// The inner promotion
        promotion: Promotion,
        /// The piece being captured
        captured_piece: NonKingPieceType,
    },
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Quiet { quiet } | Self::Capture { quiet, .. } => {
                let from = quiet.from();
                let to = quiet.to();
                write!(f, "{from}{to}")
            }
            Self::DoublePawnPush { pawn_push } => {
                let from = Square::from(pawn_push.from());
                let to = Square::from(pawn_push.to());
                write!(f, "{from}{to}")
            }
            Self::EnPassantCapture { capture } => {
                let from = Square::from(capture.from());
                let to = Square::from(capture.to());
                write!(f, "{from}{to}")
            }
            Self::Castle {
                castle_direction,
                player,
            } => {
                let from = KING_FROM_SQUARES[player];
                let to = KING_TO_SQUARES[castle_direction][player];
                write!(f, "{from}{to}")
            }
            Self::Promotion { promotion } | Self::PromotingCapture { promotion, .. } => {
                let from = Square::from(promotion.from());
                let to = Square::from(promotion.to());
                let piece = promotion.piece.get_lower_char();
                write!(f, "{from}{to}{piece}")
            }
        }
    }
}

impl ChessMove {
    /// Create a new quiet [move](ChessMove)
    #[must_use]
    pub const fn new_quiet(quiet: QuietMove) -> Self {
        Self::Quiet { quiet }
    }

    /// Create a new capture [move](ChessMove)
    #[must_use]
    pub const fn new_capture(quiet: QuietMove, captured_piece: NonKingPieceType) -> Self {
        Self::Capture {
            quiet,
            captured_piece,
        }
    }

    /// Create a new double pawn push [move](ChessMove)
    #[must_use]
    pub const fn new_double_pawn_push(pawn_push: DoublePawnPush) -> Self {
        Self::DoublePawnPush { pawn_push }
    }

    /// Create a new en-passant capture [move](ChessMove)
    #[must_use]
    pub const fn new_en_passant_capture(capture: EnPassantCapture) -> Self {
        Self::EnPassantCapture { capture }
    }

    /// Create a new king/rook castling [move](ChessMove)
    #[must_use]
    pub const fn new_castle(castle_direction: CastleDirection, player: PlayerColor) -> Self {
        Self::Castle {
            castle_direction,
            player,
        }
    }

    /// Create a new pawn promotion [move](ChessMove)
    #[must_use]
    pub const fn new_promotion(promotion: Promotion) -> Self {
        Self::Promotion { promotion }
    }

    /// Create a new pawn promoting capture [move](ChessMove)
    #[must_use]
    pub const fn new_promoting_capture(
        promotion: Promotion,
        captured_piece: NonKingPieceType,
    ) -> Self {
        Self::PromotingCapture {
            promotion,
            captured_piece,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::castles::CastleDirection::*;
    use crate::chess_move::double_pawn_push::DoublePawnPush;
    use crate::chess_move::en_passant_capture::EnPassantCapture;
    use crate::chess_move::promotion::Promotion;
    use crate::chess_move::quiet::QuietMove;
    use crate::chess_move::ChessMove;
    use crate::direction::PawnCaptureDirection;
    use crate::pieces::{NonKingPieceType, PieceType::*};
    use crate::pieces::{Piece, PromotablePieceType};
    use crate::player_color::PlayerColor::*;
    use crate::square::{
        DoublePawnToSquare, EastShiftableFile, File, Square::*, WestShiftableFile,
    };
    use test_case::test_case;

    #[test_case(ChessMove::new_quiet(QuietMove::new(E4, E5, Pawn.owned_by(White)).unwrap()), "e4e5")]
    #[test_case(ChessMove::new_quiet(QuietMove::new(F1, G3, Knight.owned_by(White)).unwrap()), "f1g3")]
    #[test_case(ChessMove::new_double_pawn_push(DoublePawnPush { player: White, file: File::B }), "b2b4")]
    #[test_case(ChessMove::new_double_pawn_push(DoublePawnPush { player: Black, file: File::H }), "h7h5")]
    #[test_case(ChessMove::new_capture(QuietMove::new(E2, F3, Pawn.owned_by(White)).unwrap(), NonKingPieceType::Pawn), "e2f3")]
    #[test_case(ChessMove::new_capture(QuietMove::new(D7, C6, Pawn.owned_by(Black)).unwrap(), NonKingPieceType::Pawn), "d7c6")]
    #[test_case(ChessMove::new_en_passant_capture(EnPassantCapture::new(DoublePawnToSquare::G5, PawnCaptureDirection::West, White).unwrap()), "g5f6")]
    #[test_case(ChessMove::new_en_passant_capture(EnPassantCapture::new(DoublePawnToSquare::A4, PawnCaptureDirection::East, Black).unwrap()), "a4b3")]
    #[test_case(ChessMove::new_castle(KingSide, White), "e1g1")]
    #[test_case(ChessMove::new_castle(QueenSide, White), "e1c1")]
    #[test_case(ChessMove::new_castle(KingSide, Black), "e8g8")]
    #[test_case(ChessMove::new_castle(QueenSide, Black), "e8c8")]
    #[test_case(
        ChessMove::new_promotion(Promotion::new(PromotablePieceType::Queen, File::E, White),),
        "e7e8q"
    )]
    #[test_case(
        ChessMove::new_promotion(Promotion::new(PromotablePieceType::Rook, File::F, Black),),
        "f2f1r"
    )]
    #[test_case(
        ChessMove::new_promotion(Promotion::new(PromotablePieceType::Bishop, File::H, Black),),
        "h2h1b"
    )]
    #[test_case(
        ChessMove::new_promotion(Promotion::new(PromotablePieceType::Knight, File::C, White),),
        "c7c8n"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            Promotion::new_east_capture(PromotablePieceType::Queen, EastShiftableFile::F, White),
            NonKingPieceType::Rook
        ),
        "f7g8q"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            Promotion::new_west_capture(PromotablePieceType::Knight, WestShiftableFile::D, White),
            NonKingPieceType::Bishop
        ),
        "d7c8n"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            Promotion::new_east_capture(PromotablePieceType::Rook, EastShiftableFile::D, Black),
            NonKingPieceType::Knight
        ),
        "d2e1r"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            Promotion::new_east_capture(PromotablePieceType::Bishop, EastShiftableFile::G, Black),
            NonKingPieceType::Bishop
        ),
        "g2h1b"
    )]
    fn display_works(chess_move: ChessMove, expected: &str) {
        use alloc::format;
        assert_eq!(format!("{chess_move}").as_str(), expected);
    }
}
