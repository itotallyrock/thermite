use crate::evaluation::pawn_evaluation::PawnEvaluation;
use crate::ply_count::PlyCount;
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
}
