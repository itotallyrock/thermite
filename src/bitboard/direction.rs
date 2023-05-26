use enum_map::Enum;
use subenum::subenum;

/// An absolute (always white's perspective) direction for rays and shifts
#[subenum(CardinalDirection, OrdinalDirection)]
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    /// Up on the board from white's perspective (towards black's back rank)
    #[subenum(CardinalDirection)]
    North = 8,
    /// Down on the board from white's perspective (towards white's back rank)
    #[subenum(CardinalDirection)]
    South = -8,
    /// Right on the board from white's perspective
    #[subenum(CardinalDirection)]
    East = 1,
    /// Left on the board from white's perspective
    #[subenum(CardinalDirection)]
    West = -1,
    /// Up-Right on the board from white's perspective
    #[subenum(OrdinalDirection)]
    NorthEast = 9,
    /// Up-Left on the board from white's perspective
    #[subenum(OrdinalDirection)]
    NorthWest = 7,
    /// Down-Left on the board from white's perspective
    #[subenum(OrdinalDirection)]
    SouthEast = -7,
    /// Down-Right on the board from white's perspective
    #[subenum(OrdinalDirection)]
    SouthWest = -9,
}
