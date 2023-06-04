use crate::player_color::PlayerColor;
use crate::square::Square;
use enum_iterator::Sequence;
use enum_map::Enum;
use subenum::subenum;

/// Any type of standard chess piece (no [owned](OwnedPiece) or specific to a [`Player`])
#[subenum(
    NonKingPieceType,
    NonPawnPieceType,
    SlidingPieceType,
    PromotablePieceType
)]
#[derive(Enum, Sequence, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
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
    #[subenum(
        NonKingPieceType,
        SlidingPieceType,
        NonPawnPieceType,
        PromotablePieceType
    )]
    Bishop,
    /// A rook can move cardinally on either axis as far is it can without capturing or bumping into its own side's pieces.
    /// A rook can also [castle](crate::castles::CastleRights) or semi-switch places with its king.
    #[subenum(
        NonKingPieceType,
        SlidingPieceType,
        NonPawnPieceType,
        PromotablePieceType
    )]
    Rook,
    /// Most powerful piece, featuring the same moves as [`Bishop`](Self::Bishop) and [`Rook`](Self::Rook) combined.
    #[subenum(
        NonKingPieceType,
        SlidingPieceType,
        NonPawnPieceType,
        PromotablePieceType
    )]
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

impl PieceType {
    /// Associate this [piece](PieceType) with a [player](PlayerColor)
    ///
    /// ```
    /// use thermite::pieces::{OwnedPiece, PieceType};
    /// use thermite::player_color::PlayerColor;
    ///
    /// assert_eq!(PieceType::Pawn.owned_by(PlayerColor::White), OwnedPiece { piece: PieceType::Pawn, player: PlayerColor::White });
    /// assert_eq!(PieceType::Pawn.owned_by(PlayerColor::Black), OwnedPiece { piece: PieceType::Pawn, player: PlayerColor::Black });
    /// assert_eq!(PieceType::Rook.owned_by(PlayerColor::White), OwnedPiece { piece: PieceType::Rook, player: PlayerColor::White });
    /// assert_eq!(PieceType::Rook.owned_by(PlayerColor::Black), OwnedPiece { piece: PieceType::Rook, player: PlayerColor::Black });
    /// assert_eq!(PieceType::Queen.owned_by(PlayerColor::White), OwnedPiece { piece: PieceType::Queen, player: PlayerColor::White });
    /// assert_eq!(PieceType::Queen.owned_by(PlayerColor::Black), OwnedPiece { piece: PieceType::Queen, player: PlayerColor::Black });
    /// ```
    #[must_use]
    pub const fn owned_by(self, player: PlayerColor) -> OwnedPiece {
        OwnedPiece {
            piece: self,
            player,
        }
    }

    /// Get the lower-case (or [white](PlayerColor::White) FEN char representation of a given piece)
    ///
    /// ```
    /// use thermite::pieces::PieceType;
    /// assert_eq!(PieceType::Bishop.get_lower_char(), 'b');
    /// assert_eq!(PieceType::Pawn.get_lower_char(), 'p');
    /// assert_eq!(PieceType::King.get_lower_char(), 'k');
    /// assert_eq!(PieceType::Knight.get_lower_char(), 'n');
    /// assert_eq!(PieceType::Queen.get_lower_char(), 'q');
    /// assert_eq!(PieceType::Rook.get_lower_char(), 'r');
    /// ```
    #[must_use]
    pub const fn get_lower_char(self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }

    /// Get the upper-case (or [black](PlayerColor::Black) FEN char representation of a given piece)
    ///
    /// ```
    /// use thermite::pieces::PieceType;
    /// assert_eq!(PieceType::King.get_upper_char(), 'K');
    /// assert_eq!(PieceType::Rook.get_upper_char(), 'R');
    /// assert_eq!(PieceType::Pawn.get_upper_char(), 'P');
    /// assert_eq!(PieceType::Queen.get_upper_char(), 'Q');
    /// assert_eq!(PieceType::Knight.get_upper_char(), 'N');
    /// assert_eq!(PieceType::Bishop.get_upper_char(), 'B');
    /// ```
    #[must_use]
    pub const fn get_upper_char(self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
}

impl OwnedPiece {
    /// Associate this [player's piece](OwnedPiece) with a specific [`Square`] on the board
    ///
    /// ```
    /// use thermite::pieces::{OwnedPiece, PieceType, PlacedPiece};
    /// use thermite::player_color::PlayerColor;
    /// use thermite::square::Square;
    ///
    /// assert_eq!(PieceType::Pawn.owned_by(PlayerColor::White).placed_on(Square::A2), PlacedPiece { owned_piece: OwnedPiece { piece: PieceType::Pawn, player: PlayerColor::White}, square: Square::A2 });
    /// assert_eq!(PieceType::Pawn.owned_by(PlayerColor::Black).placed_on(Square::A2), PlacedPiece { owned_piece: OwnedPiece { piece: PieceType::Pawn, player: PlayerColor::Black}, square: Square::A2 });
    /// assert_eq!(PieceType::Rook.owned_by(PlayerColor::Black).placed_on(Square::A1), PlacedPiece { owned_piece: OwnedPiece { piece: PieceType::Rook, player: PlayerColor::Black}, square: Square::A1 });
    /// ```
    #[must_use]
    pub const fn placed_on(self, square: Square) -> PlacedPiece {
        PlacedPiece {
            owned_piece: self,
            square,
        }
    }
}
