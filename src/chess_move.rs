use crate::castles::{CastleDirection, KING_FROM_SQUARES, KING_TO_SQUARES};
use crate::pieces::{PieceType, PromotablePieceType};
use crate::player_color::PlayerColor;
use crate::square::{EnPassantSquare, PromotionSquare, Square};
use core::fmt::{Display, Formatter};

/// The different types of chess moves and the relevant metadata to make (or undo) them
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ChessMove {
    /// Plain chess move, take a piece from a square and move it to another square
    Quiet {
        /// The starting [`square`](Square) the piece is moving `from`
        from: Square,
        /// The ending [`square`](Square) the piece moving `to`
        to: Square,
        /// The piece moving
        piece_type: PieceType,
    },
    /// Special starting rank unobstructed two square pawn push
    DoublePawnPush {
        /// The starting [`square`](Square) the pawn is moving `from`
        from: Square,
        /// The ending [`square`](Square) the pawn jumped `to`
        to: Square,
        /// The square being jumped over
        en_passant_square: EnPassantSquare,
    },
    /// Simple capturing chess move, capture a piece on the target square moving the piece from the to square to the target square
    Capture {
        /// The starting [`square`](Square) the piece is moving `from`
        from: Square,
        /// The ending [`square`](Square) the piece moving `to` and capturing on
        to: Square,
        /// The piece moving
        piece_type: PieceType,
        /// The piece being captured
        captured_piece: PieceType,
    },
    /// Capture a pawn on its skipped square for a pawn that just double jumped
    EnPassantCapture {
        /// The starting [`square`](Square) the piece is moving `from`
        from: Square,
        /// The ending [`en-passant square`](EnPassantSquare) the piece moving `to`
        to: EnPassantSquare,
        /// The square of the pawn that double-jumped
        captured_pawn_square: Square,
    },
    /// Castle, or swap the rook and king shifting the king towards the center if both have not been moved yet
    Castle {
        /// The direction the king is castling towards
        castle_direction: CastleDirection,
        /// The player doing the castling
        player: PlayerColor,
    },
    /// Push a pawn to the opposite side's back rank to upgrade the pawn to a [`PromotionPieceType`]
    Promotion {
        /// The starting [`square`](Square) the piece is moving `from`
        from: Square,
        /// The ending [`promotion-square`](PromotionSquare) the piece moving `to`
        to: PromotionSquare,
        /// The piece to promote the pawn to
        promotion: PromotablePieceType,
    },
    /// Capture on the last rank for promoting a pawn
    PromotingCapture {
        /// The starting [`square`](Square) the piece is moving `from`
        from: Square,
        /// The ending [`promotion-square`](PromotionSquare) the piece moving `to` and capturing on
        to: PromotionSquare,
        /// The piece to promote the pawn to
        promotion: PromotablePieceType,
        /// The piece being captured
        captured_piece: PieceType,
    },
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Quiet { from, to, .. }
            | Self::DoublePawnPush { from, to, .. }
            | Self::Capture { from, to, .. } => {
                write!(f, "{from}{to}")
            }
            Self::EnPassantCapture { from, to, .. } => {
                let to = Square::from(to);
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
            Self::PromotingCapture {
                from,
                to,
                promotion,
                ..
            }
            | Self::Promotion {
                from,
                to,
                promotion,
            } => {
                let promotion = PieceType::from(promotion).get_lower_char();
                let to = Square::from(to);
                write!(f, "{from}{to}{promotion}")
            }
        }
    }
}

impl ChessMove {
    /// Create a new quiet [move](ChessMove)
    #[must_use]
    pub fn new_quiet(from: Square, to: Square, piece_type: PieceType) -> Self {
        debug_assert_ne!(from, to, "attempting to create `Quiet` moving `from` the same `Square` as the destination `to` `Square`");
        Self::Quiet {
            from,
            to,
            piece_type,
        }
    }

    /// Create a new capture [move](ChessMove)
    #[must_use]
    pub fn new_capture(
        from: Square,
        to: Square,
        piece_type: PieceType,
        captured_piece: PieceType,
    ) -> Self {
        debug_assert_ne!(from, to, "attempting to create `Capture` moving `from` the same `Square` as the destination `to` `Square`");
        debug_assert_ne!(
            captured_piece,
            PieceType::King,
            "attempting to create `Capture` targeting `King`"
        );
        Self::Capture {
            from,
            to,
            piece_type,
            captured_piece,
        }
    }

    /// Create a new double pawn push [move](ChessMove)
    #[must_use]
    pub fn new_double_pawn_push(
        from: Square,
        to: Square,
        en_passant_square: EnPassantSquare,
    ) -> Self {
        debug_assert_eq!((from.rank() as u8).abs_diff(to.rank() as u8), 2, "attempting to create `DoublePawnPush` with `from` `Square` that is not two `Rank`s off the `to` `Square`");
        debug_assert_eq!((from.file() as u8).abs_diff(to.file() as u8), 0, "attempting to create `DoublePawnPush` with `from` `Square` that is not the same `File` as the `to` `Square`");
        Self::DoublePawnPush {
            from,
            to,
            en_passant_square,
        }
    }

    /// Create a new en-passant capture [move](ChessMove)
    #[must_use]
    pub fn new_en_passant_capture(
        from: Square,
        to: EnPassantSquare,
        captured_pawn_square: Square,
    ) -> Self {
        debug_assert_eq!((from.rank() as u8).abs_diff(Square::from(to).rank() as u8), 1, "attempting to create `EnPassantCapture` with `from` `Square` that is not one `Rank` off the `to` `EnPassantSquare`");
        debug_assert_eq!((from.file() as u8).abs_diff(Square::from(to).file() as u8), 1, "attempting to create `EnPassantCapture` with `from` `Square` that is not one `File` off the `to` `EnPassantSquare`");
        debug_assert_eq!((captured_pawn_square.file() as u8).abs_diff(Square::from(to).file() as u8), 0, "attempting to create `EnPassantCapture` with `captured_pawn_square` `Square` that is not the same `File` as the `to` `EnPassantSquare`");
        debug_assert_eq!((captured_pawn_square.rank() as u8).abs_diff(Square::from(to).rank() as u8), 1, "attempting to create `EnPassantCapture` with `captured_pawn_square` `Square` that is not one `Rank` off from the `to` `EnPassantSquare`");
        Self::EnPassantCapture {
            from,
            to,
            captured_pawn_square,
        }
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
    pub fn new_promotion(
        from: Square,
        to: PromotionSquare,
        promotion: PromotablePieceType,
    ) -> Self {
        debug_assert_eq!((from.rank() as u8).abs_diff(Square::from(to).rank() as u8), 1, "attempting to create `Promotion` with `from` `Square` that is not one `Rank` off the `to` `PromotionSquare`");
        debug_assert_eq!((from.file() as u8).abs_diff(Square::from(to).file() as u8), 0, "attempting to create `Promotion` with `from` `Square` that is not the same `File` of the `to` `PromotionSquare`");
        Self::Promotion {
            from,
            to,
            promotion,
        }
    }

    /// Create a new pawn promoting capture [move](ChessMove)
    #[must_use]
    pub fn new_promoting_capture(
        from: Square,
        to: PromotionSquare,
        promotion: PromotablePieceType,
        captured_piece: PieceType,
    ) -> Self {
        debug_assert_eq!((from.rank() as u8).abs_diff(Square::from(to).rank() as u8), 1, "attempting to create `PromotingCapture` with `from` `Square` that is not one `Rank` off the `to` `PromotionSquare`");
        debug_assert_eq!((from.file() as u8).abs_diff(Square::from(to).file() as u8), 1, "attempting to create `PromotingCapture` with `from` `Square` that is not one `File` off the `to` `PromotionSquare`");
        Self::PromotingCapture {
            from,
            to,
            promotion,
            captured_piece,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::castles::CastleDirection::*;
    use crate::chess_move::ChessMove;
    use crate::pieces::PieceType::*;
    use crate::pieces::PromotablePieceType;
    use crate::player_color::PlayerColor::*;
    use crate::square::{EnPassantSquare, PromotionSquare, Square::*};
    use test_case::test_case;

    #[test_case(ChessMove::new_quiet(E4, E5, Pawn), "e4e5")]
    #[test_case(ChessMove::new_quiet(F1, G3, Knight), "f1g3")]
    #[test_case(ChessMove::new_double_pawn_push(B2, B4, EnPassantSquare::B3), "b2b4")]
    #[test_case(ChessMove::new_double_pawn_push(H7, H5, EnPassantSquare::H6), "h7h5")]
    #[test_case(ChessMove::new_capture(E2, F3, Pawn, Pawn), "e2f3")]
    #[test_case(ChessMove::new_capture(D7, C6, Pawn, Pawn), "d7c6")]
    #[test_case(ChessMove::new_en_passant_capture(G5, EnPassantSquare::F6, F5), "g5f6")]
    #[test_case(ChessMove::new_en_passant_capture(A4, EnPassantSquare::B3, B4), "a4b3")]
    #[test_case(ChessMove::new_castle(KingSide, White), "e1g1")]
    #[test_case(ChessMove::new_castle(QueenSide, White), "e1c1")]
    #[test_case(ChessMove::new_castle(KingSide, Black), "e8g8")]
    #[test_case(ChessMove::new_castle(QueenSide, Black), "e8c8")]
    #[test_case(
        ChessMove::new_promotion(E7, PromotionSquare::E8, PromotablePieceType::Queen),
        "e7e8q"
    )]
    #[test_case(
        ChessMove::new_promotion(F2, PromotionSquare::F1, PromotablePieceType::Rook),
        "f2f1r"
    )]
    #[test_case(
        ChessMove::new_promotion(H2, PromotionSquare::H1, PromotablePieceType::Bishop),
        "h2h1b"
    )]
    #[test_case(
        ChessMove::new_promotion(C7, PromotionSquare::C8, PromotablePieceType::Knight),
        "c7c8n"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            F7,
            PromotionSquare::G8,
            PromotablePieceType::Queen,
            Rook
        ),
        "f7g8q"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            D7,
            PromotionSquare::C8,
            PromotablePieceType::Knight,
            Bishop
        ),
        "d7c8n"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            D2,
            PromotionSquare::E1,
            PromotablePieceType::Rook,
            Knight
        ),
        "d2e1r"
    )]
    #[test_case(
        ChessMove::new_promoting_capture(
            G2,
            PromotionSquare::H1,
            PromotablePieceType::Bishop,
            Bishop
        ),
        "g2h1b"
    )]
    fn display_works(chess_move: ChessMove, expected: &str) {
        assert_eq!(chess_move.to_string().as_str(), expected);
    }
}
