
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OrdinalDirection {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl From<OrdinalDirection> for Direction {
    fn from(value: OrdinalDirection) -> Self {
        match value {
            OrdinalDirection::NorthEast => Self::NorthEast,
            OrdinalDirection::NorthWest => Self::NorthWest,
            OrdinalDirection::SouthEast => Self::SouthEast,
            OrdinalDirection::SouthWest => Self::SouthWest,
        }
    }
}

impl From<CardinalDirection> for Direction {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Self::North,
            CardinalDirection::South => Self::South,
            CardinalDirection::East => Self::East,
            CardinalDirection::West => Self::West,
        }
    }
}

impl TryFrom<Direction> for CardinalDirection {
    type Error = ();

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::North => Ok(Self::North),
            Direction::South => Ok(Self::South),
            Direction::East => Ok(Self::East),
            Direction::West => Ok(Self::West),
            Direction::NorthEast | Direction::NorthWest | Direction::SouthEast | Direction::SouthWest => Err(()),
        }
    }
}

impl TryFrom<Direction> for OrdinalDirection {
    type Error = ();

    fn try_from(value: Direction) -> Result<Self, Self::Error> {
        match value {
            Direction::NorthEast => Ok(Self::NorthEast),
            Direction::NorthWest => Ok(Self::NorthWest),
            Direction::SouthEast => Ok(Self::SouthEast),
            Direction::SouthWest => Ok(Self::SouthWest),
            Direction::North | Direction::South | Direction::East | Direction::West => Err(()),
        }
    }
}
