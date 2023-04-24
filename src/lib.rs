#![warn(missing_docs, clippy::pedantic, rustdoc::missing_doc_code_examples, clippy::nursery, clippy::cargo, clippy::style)]

use arrayvec::ArrayVec;
use nutype::nutype;
use derive_more::{AsMut, AsRef};
use raw_position::{RawPosition, RawPositionState};
use crate::half_move_clock::HalfMoveClock;
use crate::zobrist::HistoryHash;

mod player_color;
mod square;
mod pieces;
mod castles;
mod half_move_clock;
mod raw_position;
mod board_mask;
mod zobrist;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum IllegalPosition {

}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct LegalPositionState {
    raw_state: RawPositionState,
    halfmove_clock: HalfMoveClock,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, AsRef, AsMut)]
pub struct LegalPosition {
    #[as_ref()]
    #[as_mut()]
    position: RawPosition,
    state: LegalPositionState,
    hash_history: Box<ArrayVec<HistoryHash, { HALF_MOVE_LIMIT_USIZE }>>,
}

impl TryFrom<RawPosition> for LegalPosition {
    type Error = IllegalPosition;

    fn try_from(position: RawPosition) -> Result<Self, Self::Error> {
        let state = LegalPositionState {
            raw_state: position.state,
            halfmove_clock: Default::default(),
        };

        Ok(Self {
            position,
            state,
            hash_history: Box::new(Default::default()),
        })
    }
}

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;

#[nutype(validate(max = 255))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct PlyCount(u8);

impl PlyCount {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn decrement(&mut self) {
        *self = Self::new(self.into_inner().saturating_sub(1)).unwrap();
    }
}


#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Game {
    #[as_ref()]
    #[as_mut()]
    legal_position: LegalPosition,
    halfmove_count: PlyCount,
}

#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Searchable {
    #[as_ref()]
    #[as_mut()]
    game: Game,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}
