use crate::player_color::PlayerColor;
use crate::square::Square;
use enum_map::Enum;
use subenum::subenum;

#[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum PieceType {
    #[subenum(NonKingPieceType)]
    Pawn,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Knight,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Bishop,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Rook,
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Queen,
    #[subenum(NonPawnPieceType)]
    King,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OwnedPiece {
    piece: PieceType,
    player: PlayerColor,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PlacedPiece {
    owned_piece: OwnedPiece,
    square: Square,
}
