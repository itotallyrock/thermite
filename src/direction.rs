use crate::player_color::PlayerColor;
use enum_map::Enum;
use subenum::subenum;

/// An absolute (always white's perspective) direction for rays and shifts
#[subenum(
    CardinalDirection,
    OrdinalDirection,
    PawnCaptureDirection,
    WhitePawnCaptureDirection,
    BlackPawnCaptureDirection
)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[repr(i8)]
pub enum Direction {
    /// Up on the board from white's perspective (towards black's back rank)
    #[subenum(CardinalDirection)]
    North = 8,
    /// Down on the board from white's perspective (towards white's back rank)
    #[subenum(CardinalDirection)]
    South = -8,
    /// Right on the board from white's perspective
    #[subenum(CardinalDirection, PawnCaptureDirection)]
    East = 1,
    /// Left on the board from white's perspective
    #[subenum(CardinalDirection, PawnCaptureDirection)]
    West = -1,
    /// Up-Right on the board from white's perspective
    #[subenum(OrdinalDirection, WhitePawnCaptureDirection)]
    NorthEast = 9,
    /// Up-Left on the board from white's perspective
    #[subenum(OrdinalDirection, WhitePawnCaptureDirection)]
    NorthWest = 7,
    /// Down-Left on the board from white's perspective
    #[subenum(OrdinalDirection, BlackPawnCaptureDirection)]
    SouthEast = -7,
    /// Down-Right on the board from white's perspective
    #[subenum(OrdinalDirection, BlackPawnCaptureDirection)]
    SouthWest = -9,
}

impl PawnCaptureDirection {
    /// Convert an east/west [`PawnCaptureDirection`] to a north/south-east/west for a given [player](PlayerColor)
    #[must_use]
    pub const fn to_sided_direction(self, player: PlayerColor) -> Direction {
        match (self, player) {
            (Self::East, PlayerColor::White) => Direction::NorthEast,
            (Self::East, PlayerColor::Black) => Direction::SouthEast,
            (Self::West, PlayerColor::White) => Direction::NorthWest,
            (Self::West, PlayerColor::Black) => Direction::SouthWest,
        }
    }

    /// Get the forward [`Direction`] a pawn would push for a given [player](PlayerColor)
    #[must_use]
    pub const fn get_pawn_push_for(player: PlayerColor) -> Direction {
        match player {
            PlayerColor::White => Direction::North,
            PlayerColor::Black => Direction::South,
        }
    }
}
