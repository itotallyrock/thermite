use crate::ply_count::PlyCount;
use derive_more::{AsRef, Display, FromStr, Into};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, AsRef, Into, Display, FromStr)]
pub struct HalfMoveClock(PlyCount);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InvalidHalfMoveClock;

impl HalfMoveClock {
    pub fn new(half_moves: PlyCount) -> Result<Self, InvalidHalfMoveClock> {
        #[allow(clippy::cast_possible_truncation)]
        if *half_moves.as_ref() <= HALF_MOVE_LIMIT_USIZE as u8 {
            Ok(Self(half_moves))
        } else {
            Err(InvalidHalfMoveClock)
        }
    }
}

impl HalfMoveClock {
    pub fn increment(&mut self) {
        *self = Self::new(PlyCount::new((*self.as_ref().as_ref()).saturating_add(1))).unwrap();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;
