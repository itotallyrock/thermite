use std::fmt::{Display, Formatter};
use thiserror::Error;

/// A specific kind of piece without respect to the piece's color
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

/// Piece kind that a pawn could promote to
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PromotionPieceType {
    Knight,
    Rook,
    Bishop,
    Queen,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "p"),
            PieceType::Knight => write!(f, "n"),
            PieceType::Rook => write!(f, "r"),
            PieceType::Bishop => write!(f, "b"),
            PieceType::Queen => write!(f, "q"),
            PieceType::King => write!(f, "k"),
        }
    }
}

impl Display for PromotionPieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", PieceType::from(*self))
    }
}

impl From<PromotionPieceType> for PieceType {
    fn from(promotional: PromotionPieceType) -> Self {
        match promotional {
            PromotionPieceType::Knight => PieceType::Knight,
            PromotionPieceType::Rook => PieceType::Rook,
            PromotionPieceType::Bishop => PieceType::Bishop,
            PromotionPieceType::Queen => PieceType::Queen,
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub struct InvalidPromotionPieceError {
    pub piece: PieceType,
}

impl Display for InvalidPromotionPieceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Promotion Piece: {}", self.piece)
    }
}

impl TryFrom<PieceType> for PromotionPieceType {
    type Error = InvalidPromotionPieceError;

    fn try_from(value: PieceType) -> Result<Self, Self::Error> {
        match value {
            PieceType::Pawn | PieceType::King => Err(InvalidPromotionPieceError { piece: value }),
            PieceType::Knight => Ok(PromotionPieceType::Knight),
            PieceType::Rook => Ok(PromotionPieceType::Rook),
            PieceType::Bishop => Ok(PromotionPieceType::Bishop),
            PieceType::Queen => Ok(PromotionPieceType::Queen),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn promotional_piece_is_commutative() {
        const PROMOTIONAL_PIECES: [PromotionPieceType; 4] = [PromotionPieceType::Bishop, PromotionPieceType::Knight, PromotionPieceType::Rook, PromotionPieceType::Queen];
        for promotional_piece in PROMOTIONAL_PIECES {
            assert_eq!(PromotionPieceType::try_from(PieceType::from(promotional_piece)), Ok(promotional_piece), "{} was not the same after converting to regular PieceType", promotional_piece);
        }
    }

    #[test]
    fn illegal_promotional_pieces_error() {
        const ILLEGAL_PROMOTIONAL_PIECES: [PieceType; 2] = [PieceType::Pawn, PieceType::King];
        for piece in ILLEGAL_PROMOTIONAL_PIECES {
            assert_eq!(PromotionPieceType::try_from(piece), Err(InvalidPromotionPieceError { piece }), "expected {} to return an error when converting to PromotionalPieceType", piece);
        }
    }
}