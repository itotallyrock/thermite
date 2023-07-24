use crate::evaluation::pawn_evaluation::PawnEvaluation;
use crate::ply_count::PlyCount;
use core::cmp::Ordering;
use core::ops::Neg;

/// The search evaluation of a position
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Score {
    /// Neither side wins
    Stalemate,
    /// Giving checkmate in a certain number of [plies](PlyCount)
    Mating(PlyCount),
    /// Getting checkmated in a certain number of [plies](PlyCount)
    Mated(PlyCount),
    /// A [pawn based](PawnEvaluation) approximate guess for the advantage of one side over the other
    Approximate(PawnEvaluation),
}

impl Score {
    /// Lowest possible score, mated in 0 plies
    pub const MIN: Self = Self::Mated(PlyCount(0));
    /// Highest possible score, mating in 0 plies
    pub const MAX: Self = Self::Mating(PlyCount(0));
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Stalemate, Self::Stalemate) => Some(Ordering::Equal),
            (Self::Stalemate | Self::Mated(_) | Self::Approximate(_), Self::Mating(_))
            | (Self::Mated(_), Self::Approximate(_) | Self::Stalemate) => Some(Ordering::Less),
            (Self::Stalemate | Self::Mating(_) | Self::Approximate(_), Self::Mated(_))
            | (Self::Mating(_), Self::Stalemate | Self::Approximate(_)) => Some(Ordering::Greater),
            (Self::Stalemate, Self::Approximate(approx)) => {
                PawnEvaluation::new(0.0).partial_cmp(approx)
            }
            (Self::Mating(a_mate_plies), Self::Mating(b_mate_plies)) => {
                b_mate_plies.partial_cmp(a_mate_plies)
            }
            (Self::Approximate(a_approx), Self::Approximate(b_approx)) => {
                a_approx.partial_cmp(b_approx)
            }
            (Self::Mated(a_mated_plies), Self::Mated(b_mated_plies)) => {
                a_mated_plies.partial_cmp(b_mated_plies)
            }
            (Self::Approximate(approx), Self::Stalemate) => {
                approx.partial_cmp(&PawnEvaluation::new(0.0))
            }
        }
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Mating(plies) => Self::Mated(plies),
            Self::Mated(plies) => Self::Mating(plies),
            Self::Approximate(pawn_evaluation) => Self::Approximate(-pawn_evaluation),
            Self::Stalemate => Self::Stalemate,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::evaluation::pawn_evaluation::PawnEvaluation;
    use crate::evaluation::Score;
    use crate::ply_count::PlyCount;
    use core::cmp::Ordering;
    use test_case::test_case;

    #[test_case(Score::Mating(PlyCount::new(0)), Score::Mated(PlyCount::new(0)))]
    #[test_case(Score::Mated(PlyCount::new(0)), Score::Mating(PlyCount::new(0)))]
    #[test_case(Score::Mated(PlyCount::new(1)), Score::Mating(PlyCount::new(1)))]
    #[test_case(Score::Mating(PlyCount::new(1)), Score::Mated(PlyCount::new(1)))]
    #[test_case(Score::Mating(PlyCount::new(20)), Score::Mated(PlyCount::new(20)))]
    #[test_case(Score::Mated(PlyCount::new(20)), Score::Mating(PlyCount::new(20)))]
    #[test_case(Score::Mated(PlyCount::new(50)), Score::Mating(PlyCount::new(50)))]
    #[test_case(Score::Mating(PlyCount::new(50)), Score::Mated(PlyCount::new(50)))]
    #[test_case(Score::Mating(PlyCount::new(230)), Score::Mated(PlyCount::new(230)))]
    #[test_case(Score::Mated(PlyCount::new(230)), Score::Mating(PlyCount::new(230)))]
    #[test_case(Score::Stalemate, Score::Stalemate)]
    #[test_case(
        Score::Approximate(PawnEvaluation::new(0.0)),
        Score::Approximate(PawnEvaluation::new(0.0))
    )]
    #[test_case(Score::Approximate(PawnEvaluation::new(2.0)), Score::Approximate(PawnEvaluation::new(-2.0)))]
    #[test_case(Score::Approximate(PawnEvaluation::new(-2.5)), Score::Approximate(PawnEvaluation::new(2.5)))]
    #[test_case(Score::Approximate(PawnEvaluation::new(-40.0)), Score::Approximate(PawnEvaluation::new(40.0)))]
    #[test_case(Score::Approximate(PawnEvaluation::new(130.0)), Score::Approximate(PawnEvaluation::new(-130.0)))]
    fn neg_works(input: Score, expected: Score) {
        assert_eq!(-input, expected);
    }

    #[test_case(Score::MAX, Score::MIN, Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mating(PlyCount::new(0)), Ordering::Equal)]
    #[test_case(Score::MAX, Score::Mating(PlyCount::new(1)), Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mating(PlyCount::new(30)), Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mating(PlyCount::new(50)), Ordering::Greater)]
    #[test_case(
        Score::Mating(PlyCount::new(0)),
        Score::Mating(PlyCount::new(1)),
        Ordering::Greater
    )]
    #[test_case(
        Score::Mating(PlyCount::new(1)),
        Score::Mating(PlyCount::new(2)),
        Ordering::Greater
    )]
    #[test_case(Score::MAX, Score::Mated(PlyCount::new(0)), Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mated(PlyCount::new(10)), Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mated(PlyCount::new(24)), Ordering::Greater)]
    #[test_case(Score::MAX, Score::Mated(PlyCount::new(50)), Ordering::Greater)]
    #[test_case(Score::MIN, Score::Mating(PlyCount::new(50)), Ordering::Less)]
    #[test_case(Score::MIN, Score::Mating(PlyCount::new(1)), Ordering::Less)]
    #[test_case(Score::MIN, Score::Mated(PlyCount::new(0)), Ordering::Equal)]
    #[test_case(Score::MIN, Score::Mated(PlyCount::new(1)), Ordering::Less)]
    #[test_case(Score::MIN, Score::Mated(PlyCount::new(2)), Ordering::Less)]
    #[test_case(
        Score::Mated(PlyCount::new(0)),
        Score::Mated(PlyCount::new(1)),
        Ordering::Less
    )]
    #[test_case(
        Score::Mated(PlyCount::new(1)),
        Score::Mated(PlyCount::new(2)),
        Ordering::Less
    )]
    #[test_case(Score::MIN, Score::Mated(PlyCount::new(19)), Ordering::Less)]
    #[test_case(Score::MIN, Score::Mated(PlyCount::new(50)), Ordering::Less)]
    #[test_case(Score::Stalemate, Score::Approximate(PawnEvaluation::new(-1.0)), Ordering::Greater)]
    #[test_case(
        Score::Stalemate,
        Score::Approximate(PawnEvaluation::new(1.0)),
        Ordering::Less
    )]
    #[test_case(Score::Stalemate, Score::Mated(PlyCount::new(1)), Ordering::Greater)]
    #[test_case(Score::Stalemate, Score::Mating(PlyCount::new(1)), Ordering::Less)]
    #[test_case(Score::Stalemate, Score::Stalemate, Ordering::Equal)]
    #[test_case(
        Score::Approximate(PawnEvaluation::new(123.0)),
        Score::Approximate(PawnEvaluation::new(123.0)),
        Ordering::Equal
    )]
    #[test_case(Score::Approximate(PawnEvaluation::new(-33.322)), Score::Approximate(PawnEvaluation::new(-33.322)), Ordering::Equal)]
    #[test_case(Score::Approximate(PawnEvaluation::new(2_147_483_647.0)), Score::Approximate(PawnEvaluation::new(-128_599.0)), Ordering::Greater)]
    fn ord_works(a: Score, b: Score, expected: Ordering) {
        assert_eq!(a.partial_cmp(&b), Some(expected));
        assert_eq!(b.partial_cmp(&a), Some(expected.reverse()));
    }
}
