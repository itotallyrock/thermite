use crate::chess_move::promotion::Promotion;
use crate::pieces::Piece;
use crate::square::Square;
use capture::Capture;
use castle::Castle;
use core::fmt::{Display, Formatter};
use double_pawn_push::DoublePawnPush;
use en_passant_capture::EnPassantCapture;
use promoting_capture::PromotingCapture;
use quiet::QuietMove;

/// A valid capturing chess move, a move that goes to another player's square and *captures* their [piece](crate::pieces::NonKingPieceType)
pub mod capture;
/// A valid castle move
pub mod castle;
/// A valid double pushing pawn for a [chess move](ChessMove)
pub mod double_pawn_push;
/// A valid capture of a pawn on its skipped square for a pawn that *just* [double jumped](DoublePawnPush)
pub mod en_passant_capture;
/// A valid promoting capture for a [chess move](ChessMove)
pub mod promoting_capture;
/// A valid pawn promotion for a [chess move](ChessMove)
pub mod promotion;
/// A valid plain chess move, take a piece from a square and move it to another square for a [chess move](ChessMove)
pub mod quiet;

/// The different types of chess moves and the relevant metadata to make (or undo) them
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ChessMove {
    /// Plain chess move, take a piece from a square and move it to another square
    Quiet(QuietMove),
    /// Special starting rank unobstructed two square pawn push
    DoublePawnPush(DoublePawnPush),
    /// Simple capturing chess move, capture a piece on the target square moving the piece from the to square to the target square
    Capture(Capture),
    /// Capture a pawn on its skipped square for a pawn that just double jumped
    EnPassantCapture(EnPassantCapture),
    /// Castle, or swap the rook and king shifting the king towards the center if both have not been moved yet
    Castle(Castle),
    /// Push a pawn to the opposite side's back rank to upgrade the pawn to a [`PromotablePieceType`](crate::pieces::PromotablePieceType)
    Promotion(Promotion),
    /// Capture on the last rank for promoting a pawn
    PromotingCapture(PromotingCapture),
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Quiet(quiet) => {
                let from = quiet.from();
                let to = quiet.to();
                write!(f, "{from}{to}")
            }
            Self::Capture(capture) => {
                let from = capture.from();
                let to = capture.to();
                write!(f, "{from}{to}")
            }
            Self::DoublePawnPush(pawn_push) => {
                let from = Square::from(pawn_push.from());
                let to = Square::from(pawn_push.to());
                write!(f, "{from}{to}")
            }
            Self::EnPassantCapture(capture) => {
                let from = Square::from(capture.from());
                let to = Square::from(capture.to());
                write!(f, "{from}{to}")
            }
            Self::Castle(castle) => {
                let from = castle.king_from();
                let to = castle.king_to();
                write!(f, "{from}{to}")
            }
            Self::Promotion(promotion) => {
                let from = Square::from(promotion.from());
                let to = Square::from(promotion.to());
                let piece = promotion.piece.get_lower_char();
                write!(f, "{from}{to}{piece}")
            }
            Self::PromotingCapture(promoting_capture) => {
                let promotion = promoting_capture.promotion();
                let from = Square::from(promotion.from());
                let to = Square::from(promotion.to());
                let piece = promotion.piece.get_lower_char();
                write!(f, "{from}{to}{piece}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::castles::CastleDirection::*;
    use crate::chess_move::capture::Capture;
    use crate::chess_move::castle::Castle;
    use crate::chess_move::double_pawn_push::DoublePawnPush;
    use crate::chess_move::en_passant_capture::EnPassantCapture;
    use crate::chess_move::promoting_capture::PromotingCapture;
    use crate::chess_move::promotion::Promotion;
    use crate::chess_move::quiet::QuietMove;
    use crate::chess_move::ChessMove;
    use crate::direction::PawnCaptureDirection;
    use crate::pieces::{NonKingPieceType, Piece, PieceType::*, PromotablePieceType};
    use crate::player_color::PlayerColor::*;
    use crate::square::{
        DoublePawnToSquare, EastShiftableFile, File, Square::*, WestShiftableFile,
    };
    use test_case::test_case;

    #[test_case(ChessMove::Quiet(QuietMove::new(E4, E5, Pawn.owned_by(White)).unwrap()), "e4e5")]
    #[test_case(ChessMove::Quiet(QuietMove::new(F1, G3, Knight.owned_by(White)).unwrap()), "f1g3")]
    #[test_case(ChessMove::DoublePawnPush(DoublePawnPush { player: White, file: File::B }), "b2b4")]
    #[test_case(ChessMove::DoublePawnPush(DoublePawnPush { player: Black, file: File::H }), "h7h5")]
    #[test_case(ChessMove::Capture(Capture { quiet: QuietMove::new(E2, F3, Pawn.owned_by(White)).unwrap(), captured_piece: NonKingPieceType::Pawn }), "e2f3")]
    #[test_case(ChessMove::Capture(Capture { quiet: QuietMove::new(D7, C6, Pawn.owned_by(Black)).unwrap(), captured_piece: NonKingPieceType::Pawn }), "d7c6")]
    #[test_case(ChessMove::EnPassantCapture(EnPassantCapture::new(DoublePawnToSquare::G5, PawnCaptureDirection::West, White).unwrap()), "g5f6")]
    #[test_case(ChessMove::EnPassantCapture(EnPassantCapture::new(DoublePawnToSquare::A4, PawnCaptureDirection::East, Black).unwrap()), "a4b3")]
    #[test_case(ChessMove::Castle(Castle::new(White, KingSide)), "e1g1")]
    #[test_case(ChessMove::Castle(Castle::new(White, QueenSide)), "e1c1")]
    #[test_case(ChessMove::Castle(Castle::new(Black, KingSide)), "e8g8")]
    #[test_case(ChessMove::Castle(Castle::new(Black, QueenSide)), "e8c8")]
    #[test_case(
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Queen, File::E, White),),
        "e7e8q"
    )]
    #[test_case(
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Rook, File::F, Black),),
        "f2f1r"
    )]
    #[test_case(
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Bishop, File::H, Black),),
        "h2h1b"
    )]
    #[test_case(
        ChessMove::Promotion(Promotion::new(PromotablePieceType::Knight, File::C, White),),
        "c7c8n"
    )]
    #[test_case(
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_east_capture(PromotablePieceType::Queen, EastShiftableFile::F, White),
            NonKingPieceType::Rook
        )),
        "f7g8q"
    )]
    #[test_case(
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_west_capture(PromotablePieceType::Knight, WestShiftableFile::D, White),
            NonKingPieceType::Bishop
        )),
        "d7c8n"
    )]
    #[test_case(
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_east_capture(PromotablePieceType::Rook, EastShiftableFile::D, Black),
            NonKingPieceType::Knight
        )),
        "d2e1r"
    )]
    #[test_case(
        ChessMove::PromotingCapture(PromotingCapture::new(
            Promotion::new_east_capture(PromotablePieceType::Bishop, EastShiftableFile::G, Black),
            NonKingPieceType::Bishop
        )),
        "g2h1b"
    )]
    fn display_works(chess_move: ChessMove, expected: &str) {
        use alloc::format;
        assert_eq!(format!("{chess_move}").as_str(), expected);
    }
}
