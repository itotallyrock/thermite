use enum_map::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum PlayerColor {
    White,
    Black,
}
