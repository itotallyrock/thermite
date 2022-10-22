use crate::piece_type::PieceType;

/// Types of pieces that can be promoted to by a pawn reaching the opposite player's back rank
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PromotionPieceType {
    /// Promote to a [`Queen`](crate::piece_type::PieceType::Queen)
    Queen,
    /// Promote to a [`Rook`](crate::piece_type::PieceType::Rook)
    Rook,
    /// Promote to a [`Bishop`](crate::piece_type::PieceType::Bishop)
    Bishop,
    /// Promote to a [`Knight`](crate::piece_type::PieceType::Knight)
    Knight,
}

/// Attempting to create a promotion for invalid piece
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IllegalPromotion(PieceType);

impl const TryFrom<PieceType> for PromotionPieceType {
    type Error = IllegalPromotion;

    fn try_from(piece_type: PieceType) -> Result<Self, Self::Error> {
        match piece_type {
            PieceType::Knight => Ok(Self::Knight),
            PieceType::Bishop => Ok(Self::Bishop),
            PieceType::Rook => Ok(Self::Rook),
            PieceType::Queen => Ok(Self::Queen),
            PieceType::Pawn | PieceType::King => Err(IllegalPromotion(piece_type)),
        }
    }
}

impl const From<PromotionPieceType> for PieceType {
    fn from(promotion: PromotionPieceType) -> Self {
        match promotion {
            PromotionPieceType::Queen => Self::Queen,
            PromotionPieceType::Rook => Self::Rook,
            PromotionPieceType::Bishop => Self::Bishop,
            PromotionPieceType::Knight => Self::Knight,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(PromotionPieceType::Queen, PieceType::Queen)]
    #[test_case(PromotionPieceType::Rook, PieceType::Rook)]
    #[test_case(PromotionPieceType::Bishop, PieceType::Bishop)]
    #[test_case(PromotionPieceType::Knight, PieceType::Knight)]
    fn piece_type_from_promotion_piece_type_works(input: PromotionPieceType, expected_piece_type: PieceType) {
        assert_eq!(PieceType::from(input), expected_piece_type);
    }

    #[test_case(PieceType::Queen, Some(PromotionPieceType::Queen))]
    #[test_case(PieceType::Rook, Some(PromotionPieceType::Rook))]
    #[test_case(PieceType::Bishop, Some(PromotionPieceType::Bishop))]
    #[test_case(PieceType::Knight, Some(PromotionPieceType::Knight))]
    #[test_case(PieceType::King, None)]
    #[test_case(PieceType::Pawn, None)]
    fn promotion_piece_type_from_piece_type_works(input: PieceType, expected_result: Option<PromotionPieceType>) {
        assert_eq!(PromotionPieceType::try_from(input).ok(), expected_result);
    }
}
