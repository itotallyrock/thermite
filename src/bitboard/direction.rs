use subenum::subenum;

#[subenum(CardinalDirection, OrdinalDirection)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    #[subenum(CardinalDirection)]
    North = 8,
    #[subenum(CardinalDirection)]
    South = -8,
    #[subenum(CardinalDirection)]
    East = 1,
    #[subenum(CardinalDirection)]
    West = -1,
    #[subenum(OrdinalDirection)]
    NorthEast = 9,
    #[subenum(OrdinalDirection)]
    NorthWest = 7,
    #[subenum(OrdinalDirection)]
    SouthEast = -7,
    #[subenum(OrdinalDirection)]
    SouthWest = -9,
}