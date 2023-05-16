use std::fmt::{Display, Formatter};
use std::ops::Mul;
use derive_more::{Add, AddAssign, Neg, Sub, SubAssign, Sum};

use crate::{PieceCount, PlyCount};
use crate::score::{EvaluationInner, GameStage};
use crate::score::game_stage::GameStageInner;

// Arbitrary large value
const MAX_INNER: EvaluationInner = 1_417_339_207;
const MATE_PLIES: PlyCount = 50;
const MATE_THRESHOLD: EvaluationInner = MAX_INNER - MATE_PLIES as EvaluationInner;
const MATED_THRESHOLD: EvaluationInner = -MATE_THRESHOLD;

/// TODO
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default, Neg, Add, Sub, Sum, SubAssign, AddAssign)]
pub struct PositionEvaluation(EvaluationInner);

impl PositionEvaluation {
    /// TODO
    pub const MAX: Self = Self::new_mating(0);
    /// TODO
    pub const MIN: Self = Self::new_mated(0);
    /// TODO
    pub const DRAW: Self = Self::new_centipawns(0);

    /// TODO
    #[must_use]
    pub const fn new_mating(mate_in: PlyCount) -> Self {
        debug_assert!(mate_in <= MATE_PLIES);
        Self(MAX_INNER - (mate_in as EvaluationInner))
    }

    /// TODO
    #[must_use]
    pub const fn new_mated(mated_in: PlyCount) -> Self {
        debug_assert!(mated_in <= MATE_PLIES);
        Self(-MAX_INNER + (mated_in as EvaluationInner))
    }

    /// TODO
    #[must_use]
    pub const fn new_centipawns(centipawns: EvaluationInner) -> Self {
        debug_assert!(centipawns < MATE_THRESHOLD);
        debug_assert!(centipawns > MATED_THRESHOLD);
        Self(centipawns)
    }

    /// TODO
    #[must_use]
    pub const fn is_mating(self) -> bool {
        self.0 >= MATE_THRESHOLD
    }

    /// TODO
    #[must_use]
    pub const fn is_mated(self) -> bool {
        self.0 <= MATED_THRESHOLD
    }

    #[must_use]
    const fn centipawns(self) -> EvaluationInner {
        // TODO: Scale this to actually equate to centi-pawn value
        self.0
    }

    #[must_use]
    const fn mated_in(self) -> EvaluationInner {
        debug_assert!(self.is_mated());
        self.0 + MAX_INNER
    }

    #[must_use]
    const fn mating_in(self) -> EvaluationInner {
        debug_assert!(self.is_mating());
        MAX_INNER - self.0
    }
}

impl Mul for PositionEvaluation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<PieceCount> for PositionEvaluation {
    type Output = Self;

    fn mul(self, rhs: PieceCount) -> Self::Output {
        #[allow(clippy::cast_lossless)]
        Self(rhs as EvaluationInner * self.0)
    }
}

impl Mul<GameStage> for PositionEvaluation {
    type Output = Self;

    fn mul(self, rhs: GameStage) -> Self::Output {
        #[allow(clippy::cast_lossless)]
        Self((rhs.0 * self.0 as GameStageInner) as EvaluationInner)
    }
}

impl Display for PositionEvaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_mated() {
            write!(f, "#-{}", self.mated_in())
        } else if self.is_mating() {
            write!(f, "#{}", self.mating_in())
        } else {
            write!(f, "{}cp", self.centipawns())
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use test_case::test_case;

    use super::*;

    #[test]
    fn size_of_positional_evaluation_is_4_bytes() {
        assert_eq!(std::mem::size_of::<PositionEvaluation>(), 4);
    }

    #[test_case(PositionEvaluation::MAX, PositionEvaluation::MIN, Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mating(0), Ordering::Equal)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mating(1), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mating(30), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mating(50), Ordering::Greater)]
    #[test_case(PositionEvaluation::new_mating(0), PositionEvaluation::new_mating(1), Ordering::Greater)]
    #[test_case(PositionEvaluation::new_mating(1), PositionEvaluation::new_mating(2), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mated(0), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mated(10), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mated(24), Ordering::Greater)]
    #[test_case(PositionEvaluation::MAX, PositionEvaluation::new_mated(50), Ordering::Greater)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mating(50), Ordering::Less)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mating(1), Ordering::Less)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mated(0), Ordering::Equal)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mated(1), Ordering::Less)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mated(2), Ordering::Less)]
    #[test_case(PositionEvaluation::new_mated(0), PositionEvaluation::new_mated(1), Ordering::Less)]
    #[test_case(PositionEvaluation::new_mated(1), PositionEvaluation::new_mated(2), Ordering::Less)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mated(19), Ordering::Less)]
    #[test_case(PositionEvaluation::MIN, PositionEvaluation::new_mated(50), Ordering::Less)]
    #[test_case(PositionEvaluation(2_147_483_647), PositionEvaluation(-128_599), Ordering::Greater)]
    // TODO: Test a bunch of a pawn approx
    fn ord_works(left: PositionEvaluation, right: PositionEvaluation, expected: Ordering) {
        assert_eq!(left.cmp(&right), expected);
        assert_eq!((-left).cmp(&(-right)), expected.reverse());
        assert_eq!(right.cmp(&left), expected.reverse());
    }

    // TODO: Test negating and a bunch of conditions

    #[test_case(PositionEvaluation::new_mating(0), true)]
    #[test_case(PositionEvaluation::new_mating(1), true)]
    #[test_case(PositionEvaluation::new_mating(2), true)]
    #[test_case(PositionEvaluation::new_mating(3), true)]
    #[test_case(PositionEvaluation::new_mating(4), true)]
    #[test_case(PositionEvaluation::new_mating(23), true)]
    #[test_case(PositionEvaluation::new_mating(MATE_PLIES), true)]
    #[test_case(PositionEvaluation::new_mating(MATE_PLIES + 1), false)]
    #[test_case(PositionEvaluation::new_mated(0), false)]
    #[test_case(PositionEvaluation::new_mated(1), false)]
    #[test_case(PositionEvaluation::new_mated(2), false)]
    #[test_case(PositionEvaluation::new_mated(3), false)]
    #[test_case(PositionEvaluation::new_mated(4), false)]
    #[test_case(PositionEvaluation::new_mated(23), false)]
    #[test_case(PositionEvaluation::new_mated(MATE_PLIES), false)]
    #[test_case(PositionEvaluation::new_mated(MATE_PLIES + 1), false)]
    fn is_mating_works(score: PositionEvaluation, expected:bool ) {
        assert_eq!(score.is_mating(), expected);
    }

    #[test_case(PositionEvaluation::new_mated(0), true)]
    #[test_case(PositionEvaluation::new_mated(1), true)]
    #[test_case(PositionEvaluation::new_mated(2), true)]
    #[test_case(PositionEvaluation::new_mated(3), true)]
    #[test_case(PositionEvaluation::new_mated(4), true)]
    #[test_case(PositionEvaluation::new_mated(23), true)]
    #[test_case(PositionEvaluation::new_mated(MATE_PLIES), true)]
    #[test_case(PositionEvaluation::new_mated(MATE_PLIES + 1), false)]
    #[test_case(PositionEvaluation::new_mating(0), false)]
    #[test_case(PositionEvaluation::new_mating(1), false)]
    #[test_case(PositionEvaluation::new_mating(2), false)]
    #[test_case(PositionEvaluation::new_mating(3), false)]
    #[test_case(PositionEvaluation::new_mating(4), false)]
    #[test_case(PositionEvaluation::new_mating(23), false)]
    #[test_case(PositionEvaluation::new_mating(MATE_PLIES), false)]
    #[test_case(PositionEvaluation::new_mating(MATE_PLIES + 1), false)]
    fn is_mated_works(score: PositionEvaluation, expected: bool) {
        assert_eq!(score.is_mated(), expected);
    }

    #[test_case(PositionEvaluation::new_mated(0), 0)]
    #[test_case(PositionEvaluation::new_mated(1), 1)]
    #[test_case(PositionEvaluation::new_mated(2), 2)]
    #[test_case(PositionEvaluation::new_mated(3), 3)]
    #[test_case(PositionEvaluation::new_mated(4), 4)]
    #[test_case(PositionEvaluation::new_mated(23), 23)]
    #[test_case(PositionEvaluation::new_mated(MATE_PLIES), #[allow(clippy::cast_lossless)] { MATE_PLIES as EvaluationInner })]
    fn mated_in_works(score: PositionEvaluation, expected: EvaluationInner) {
        assert_eq!(score.mated_in(), expected);
    }

    #[test_case(PositionEvaluation::new_mating(0), 0)]
    #[test_case(PositionEvaluation::new_mating(1), 1)]
    #[test_case(PositionEvaluation::new_mating(2), 2)]
    #[test_case(PositionEvaluation::new_mating(3), 3)]
    #[test_case(PositionEvaluation::new_mating(4), 4)]
    #[test_case(PositionEvaluation::new_mating(23), 23)]
    #[test_case(PositionEvaluation::new_mating(MATE_PLIES), #[allow(clippy::cast_lossless)] { MATE_PLIES as EvaluationInner })]
    fn mating_in_works(score: PositionEvaluation, expected: EvaluationInner) {
        assert_eq!(score.mating_in(), expected);
    }
}