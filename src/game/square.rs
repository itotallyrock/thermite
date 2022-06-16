use std::fmt::{Display, Formatter};

/// Little-Endian square offset 0-63
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[rustfmt::skip]
pub enum SquareOffset {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Display for SquareOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SquareOffset::A1 => write!(f, "a1"),
            SquareOffset::B1 => write!(f, "b1"),
            SquareOffset::C1 => write!(f, "c1"),
            SquareOffset::D1 => write!(f, "d1"),
            SquareOffset::E1 => write!(f, "e1"),
            SquareOffset::F1 => write!(f, "f1"),
            SquareOffset::G1 => write!(f, "g1"),
            SquareOffset::H1 => write!(f, "h1"),

            SquareOffset::A2 => write!(f, "a2"),
            SquareOffset::B2 => write!(f, "b2"),
            SquareOffset::C2 => write!(f, "c2"),
            SquareOffset::D2 => write!(f, "d2"),
            SquareOffset::E2 => write!(f, "e2"),
            SquareOffset::F2 => write!(f, "f2"),
            SquareOffset::G2 => write!(f, "g2"),
            SquareOffset::H2 => write!(f, "h2"),

            SquareOffset::A3 => write!(f, "a3"),
            SquareOffset::B3 => write!(f, "b3"),
            SquareOffset::C3 => write!(f, "c3"),
            SquareOffset::D3 => write!(f, "d3"),
            SquareOffset::E3 => write!(f, "e3"),
            SquareOffset::F3 => write!(f, "f3"),
            SquareOffset::G3 => write!(f, "g3"),
            SquareOffset::H3 => write!(f, "h3"),

            SquareOffset::A4 => write!(f, "a4"),
            SquareOffset::B4 => write!(f, "b4"),
            SquareOffset::C4 => write!(f, "c4"),
            SquareOffset::D4 => write!(f, "d4"),
            SquareOffset::E4 => write!(f, "e4"),
            SquareOffset::F4 => write!(f, "f4"),
            SquareOffset::G4 => write!(f, "g4"),
            SquareOffset::H4 => write!(f, "h4"),

            SquareOffset::A5 => write!(f, "a5"),
            SquareOffset::B5 => write!(f, "b5"),
            SquareOffset::C5 => write!(f, "c5"),
            SquareOffset::D5 => write!(f, "d5"),
            SquareOffset::E5 => write!(f, "e5"),
            SquareOffset::F5 => write!(f, "f5"),
            SquareOffset::G5 => write!(f, "g5"),
            SquareOffset::H5 => write!(f, "h5"),

            SquareOffset::A6 => write!(f, "a6"),
            SquareOffset::B6 => write!(f, "b6"),
            SquareOffset::C6 => write!(f, "c6"),
            SquareOffset::D6 => write!(f, "d6"),
            SquareOffset::E6 => write!(f, "e6"),
            SquareOffset::F6 => write!(f, "f6"),
            SquareOffset::G6 => write!(f, "g6"),
            SquareOffset::H6 => write!(f, "h6"),

            SquareOffset::A7 => write!(f, "a7"),
            SquareOffset::B7 => write!(f, "b7"),
            SquareOffset::C7 => write!(f, "c7"),
            SquareOffset::D7 => write!(f, "d7"),
            SquareOffset::E7 => write!(f, "e7"),
            SquareOffset::F7 => write!(f, "f7"),
            SquareOffset::G7 => write!(f, "g7"),
            SquareOffset::H7 => write!(f, "h7"),

            SquareOffset::A8 => write!(f, "a8"),
            SquareOffset::B8 => write!(f, "b8"),
            SquareOffset::C8 => write!(f, "c8"),
            SquareOffset::D8 => write!(f, "d8"),
            SquareOffset::E8 => write!(f, "e8"),
            SquareOffset::F8 => write!(f, "f8"),
            SquareOffset::G8 => write!(f, "g8"),
            SquareOffset::H8 => write!(f, "h8"),
        }
    }
}
