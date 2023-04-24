use crate::half_move_clock::HalfMoveClock;
use crate::half_move_clock::HALF_MOVE_LIMIT_USIZE;
use crate::raw_position::{RawPosition, RawPositionState};
use crate::zobrist::HistoryHash;
use arrayvec::ArrayVec;
use derive_more::{AsMut, AsRef};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum IllegalPosition {}

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
            halfmove_clock: HalfMoveClock::default(),
        };

        Ok(Self {
            position,
            state,
            hash_history: Box::default(),
        })
    }
}
