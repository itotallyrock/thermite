use std::ops::Neg;

pub(super) type GameStageInner = f32;

/// TODO
#[derive(Copy, Clone, PartialEq)]
pub struct GameStage(pub(crate) GameStageInner);

impl Neg for GameStage {
    type Output = Self;

    fn neg(self) -> Self::Output {
        debug_assert!(self.0 >= 0.0 && self.0 <= 1.0, "out of bounds game_state");
        Self(1.0 - self.0)
    }
}