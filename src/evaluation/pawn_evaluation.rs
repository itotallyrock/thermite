use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

/// The search approximation (in pawns)
#[derive(
    Copy,
    Clone,
    PartialEq,
    Debug,
    PartialOrd,
    Constructor,
    Neg,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Deref,
    DerefMut,
)]
pub struct PawnEvaluation(pub(crate) f32);

impl PawnEvaluation {
    /// Get the rounded centi-pawn (1/100th of a pawn is 1 centi-pawn) representation
    #[must_use]
    pub fn centipawns(&self) -> i32 {
        #[allow(clippy::cast_possible_truncation)]
        {
            (self.0 * 100.0) as i32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::evaluation::pawn_evaluation::PawnEvaluation;
    use test_case::test_case;

    #[test_case(PawnEvaluation::new(0.0), 0)]
    #[test_case(PawnEvaluation::new(1.0), 100)]
    #[test_case(PawnEvaluation::new(2.0), 200)]
    #[test_case(PawnEvaluation::new(-4.0), -400)]
    #[test_case(PawnEvaluation::new(-230.0), -23000)]
    fn centipawns_works(input: PawnEvaluation, expected: i32) {
        assert_eq!(input.centipawns(), expected);
    }
}
