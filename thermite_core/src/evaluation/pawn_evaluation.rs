use derive_more::{
    Add, AddAssign, Constructor, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub,
    SubAssign,
};

/// The search approximation (in pawns)
#[derive(
    Copy,
    Clone,
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
    /// Get the absolute value of the evaluation
    ///
    /// ```
    /// use thermite_core::evaluation::PawnEvaluation;
    /// assert_eq!(PawnEvaluation::new(1.0).abs(), PawnEvaluation::new(1.0));
    /// assert_eq!(PawnEvaluation::new(-0.01).abs(), PawnEvaluation::new(0.01));
    /// assert_eq!(PawnEvaluation::new(12.3).abs(), PawnEvaluation::new(12.3));
    /// assert_eq!(PawnEvaluation::new(-15.09).abs(), PawnEvaluation::new(15.09));
    /// assert_eq!(PawnEvaluation::new(0.05).abs(), PawnEvaluation::new(0.05));
    /// assert_eq!(PawnEvaluation::new(0.0).abs(), PawnEvaluation::new(0.0));
    /// ```
    #[must_use]
    pub fn abs(self) -> Self {
        if self.0.is_sign_positive() {
            self
        } else {
            -self
        }
    }

    /// Get the rounded centi-pawn (1/100th of a pawn is 1 centi-pawn) representation
    #[must_use]
    pub fn centipawns(&self) -> i32 {
        (self.0 * 100.0) as i32
    }
}

impl PartialEq for PawnEvaluation {
    fn eq(&self, other: &Self) -> bool {
        // f32::EPSILON.sqrt()
        const EPSILON: PawnEvaluation = Self(0.000_345_266_98);
        (*self - *other).abs() <= EPSILON
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
