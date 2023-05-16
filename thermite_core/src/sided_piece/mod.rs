use crate::piece_type::PieceType;
use crate::player::Player;

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SidedPiece {
    pub piece_type: PieceType,
    pub player: Player,
}

impl SidedPiece {
    /// Create a new `SidedPiece`
    #[must_use]
    pub const fn new(piece_type: PieceType, player: Player) -> Self {
        Self {
            piece_type,
            player,
        }
    }
}

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct IllegalSidedPiece(char);

impl TryFrom<char> for SidedPiece {
    type Error = IllegalSidedPiece;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let (piece_type, player) = match c {
            // Pawn
            'P' => (PieceType::Pawn, Player::White),
            'p' => (PieceType::Pawn, Player::Black),
            // Knight
            'N' => (PieceType::Knight, Player::White),
            'n' => (PieceType::Knight, Player::Black),
            // Bishop
            'B' => (PieceType::Bishop, Player::White),
            'b' => (PieceType::Bishop, Player::Black),
            // Rook
            'R' => (PieceType::Rook, Player::White),
            'r' => (PieceType::Rook, Player::Black),
            // Queen
            'Q' => (PieceType::Queen, Player::White),
            'q' => (PieceType::Queen, Player::Black),
            // King
            'K' => (PieceType::King, Player::White),
            'k' => (PieceType::King, Player::Black),
            _ => return Err(IllegalSidedPiece(c)),
        };

        Ok(SidedPiece {
            piece_type,
            player,
        })
    }
}
