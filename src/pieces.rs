use crate::player_color::PlayerColor;
use crate::square::Square;
use enum_map::Enum;
use subenum::subenum;

#[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum PieceType {
    /// A pawn that can only push forward one square at a time, except for its first move, which can move two squares forward (if unobstructed).
    /// Can also attack diagonally forward-left/right.  Upon reaching the opposite sides back-rank the pawn will promote to a promotion piece ([`Queen`](Self::Queen), [`Rook`](Self::Rook), [`Knight`](Self::Knight), [`Bishop`](Self::Bishop)).
    #[subenum(NonKingPieceType)]
    Pawn,
    /// A knight can move in an L shape (2x1) and can move to any square.
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Knight,
    /// A bishop can move diagonally on either axis as far is it can without capturing or bumping into its own side's pieces.
    /// A bishop is locked to the specific colored square it starts on.
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Bishop,
    /// A rook can move cardinally on either axis as far is it can without capturing or bumping into its own side's pieces.
    /// A rook can also [castle](crate::castles::CastleRights) or semi-switch places with its king.
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Rook,
    /// Most powerful piece, featuring the same moves as [`Bishop`](Self::Bishop) and [`Rook`](Self::Rook) combined.
    #[subenum(NonKingPieceType, NonPawnPieceType, PromotablePieceType)]
    Queen,
    /// The piece that must be protected in order to win.  Your king being attacked is checked, and
    /// ending your turn with your king attacked is check-mate (a loss).  A king can move/capture a single
    /// square in each direction (if each square is not attacked).
    #[subenum(NonPawnPieceType)]
    King,
}

/// A colored piece that a specific player owns
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OwnedPiece {
    /// The type of piece
    pub piece: PieceType,
    /// The piece's owner
    pub player: PlayerColor,
}

/// A piece that is owned by a player placed on a square
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PlacedPiece {
    /// The piece and its owner
    pub owned_piece: OwnedPiece,
    /// The square the piece is placed on
    pub square: Square,
}
