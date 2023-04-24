use nutype::nutype;
use derive_more::{AsMut, AsRef};
use crate::legal_position::LegalPosition;
use crate::ply_count::PlyCount;

#[nutype(validate(max = 50))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct HalfMoveClock(u8);

impl Default for HalfMoveClock {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl HalfMoveClock {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}


#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Game {
    #[as_ref()]
    #[as_mut()]
    legal_position: LegalPosition,
    halfmove_count: PlyCount,
}

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;
