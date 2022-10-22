use crate::piece_type::PieceType;
use crate::promotion_piece_type::PromotionPieceType;
use crate::square::Square;

/// The different types of chess moves and the relevant metadata to make (or undo) them
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveType {
    /// Plain chess move, take a piece from a square and move it to another square
    Quiet {
        /// The piece moving
        piece_type: PieceType,
    },
    /// Special starting rank unobstructed two square pawn push
    DoublePawnPush {
        /// The square being jumped over
        en_passant_square: Square,
    },
    /// Simple capturing chess move, capture a piece on the target square moving the piece from the to square to the target square
    Capture {
        /// The piece moving
        piece_type: PieceType,
        /// The piece being captured
        captured_piece: PieceType,
    },
    /// Capture a pawn on its skipped square for a pawn that just double jumped
    EnPassantCapture {
        /// The square of the pawn that double-jumped
        captured_pawn_square: Square,
    },
    /// Castle towards the king-side, moving the rook between the H file and the F file and the king between E file and G file
    KingSideCastle {
        /// The square the rook is moving from
        rook_from_square: Square,
    },
    /// Castle towards the queen-side, moving the rook between the A file and the D file and the king between E file and C file
    QueenSideCastle {
        /// The square the rook is moving from
        rook_from_square: Square,
    },
    /// Push a pawn to the opposite side's back rank to upgrade the pawn to a [`PromotionPieceType`]
    Promotion {
        /// The piece to promote the pawn to
        promotion: PromotionPieceType,
    },
    /// Capture on the last rank for promoting a pawn
    PromotingCapture {
        /// The piece to promote the pawn to
        promotion: PromotionPieceType,
        /// The piece being captured
        captured_piece: PieceType,
    },
}
