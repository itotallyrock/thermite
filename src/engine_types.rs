use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::num::{NonZeroI16, NonZeroU64, NonZeroU8};
use std::ops::Neg;

use crate::game::SimpleChessMove;

/// Number of hits in the endgame table database
pub type TableBaseHits = u64;
/// Number of positions (nodes) visited in a search tree (non-zero)
pub type NodeCount = NonZeroU64;
/// Number of sequential moves (plies)
pub type PlyCount = u8;
/// Number of moves to search deep (non-zero plies)
pub type SearchDepth = NonZeroU8;
/// Simple move list for moves provided from the GUI
pub type SimpleMoveList = Vec<SimpleChessMove>;

/// Approximate board evaluation in 1 / 100th pawns
pub type CentiPawns = i16;

/// How many [plies](PlyCount) until checkmate, signed to include both sides.
/// Negative for getting checkmated, positive for checkmating.
pub type MateDepth = NonZeroI16;

/// Engine's evaluation/rating for a given position.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Score {
    /// Approximate valuation of the position as an integer of 1/100ths of a pawn
    Centipawns(CentiPawns),
    /// The number of moves until a mate
    Mate(MateDepth),
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Centipawns(centipawns) => write!(f, "cp {}", centipawns),
            Score::Mate(mate_plies) => write!(f, "mate {}", mate_plies),
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Mates need to be compared specially to handle reversal and negatives
            (Score::Mate(self_mate), Score::Mate(other_mate)) => {
                let self_mate = self_mate.get();
                let other_mate = other_mate.get();

                if self_mate.is_negative() {
                    if other_mate.is_negative() {
                        // Getting checkmated, compare to choose largest
                        self_mate.cmp(&other_mate)
                    } else {
                        Ordering::Less
                    }
                } else if other_mate.is_negative() {
                    Ordering::Greater
                } else {
                    // Checkmating, compare to choose smallest
                    other_mate.cmp(&self_mate)
                }
            }
            // Pawn values can be directly compared
            (Score::Centipawns(self_centipawns), Score::Centipawns(other_centipawns)) => {
                self_centipawns.cmp(other_centipawns)
            }
            // Any mate > any pawn value
            (Score::Mate(_), Score::Centipawns(_)) => Ordering::Greater,
            // Any pawn value < any mate value
            (Score::Centipawns(_), Score::Mate(_)) => Ordering::Less,
        }
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Score::Centipawns(centipawns) => Score::Centipawns(centipawns.neg()),
            Score::Mate(mate_plies) => Score::Mate(
                MateDepth::new(mate_plies.get().neg()).expect("negated zero is still non-zero"),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ord_works_for_positive_centipawns() {
        let a = Score::Centipawns(233);
        let b = Score::Centipawns(107);
        assert!(a > b);
    }

    #[test]
    fn ord_works_for_negative_centipawns() {
        let a = Score::Centipawns(-180);
        let b = Score::Centipawns(-450);
        assert!(a > b);
    }

    #[test]
    fn ord_works_for_mixed_positive_negative_centipawns() {
        let a = Score::Centipawns(924);
        let b = Score::Centipawns(-655);
        assert!(a > b);
    }

    #[test]
    fn ord_works_for_mixed_positive_negative_mate() {
        let a = Score::Mate(MateDepth::new(12).unwrap());
        let b = Score::Mate(MateDepth::new(-5).unwrap());
        assert!(a > b);
        let a = Score::Mate(MateDepth::new(-1).unwrap());
        let b = Score::Mate(MateDepth::new(23).unwrap());
        assert!(b > a);
    }

    #[test]
    fn ord_works_for_positive_mate() {
        let a = Score::Mate(MateDepth::new(7).unwrap());
        let b = Score::Mate(MateDepth::new(5).unwrap());
        assert!(b > a);
    }

    #[test]
    fn ord_works_for_negative_mate() {
        let a = Score::Mate(MateDepth::new(-4).unwrap());
        let b = Score::Mate(MateDepth::new(-1).unwrap());
        assert!(b > a);
    }

    #[test]
    fn neg_centipawns_is_cyclical() {
        let score = Score::Centipawns(320);
        assert_ne!(score.neg(), score);
        assert_eq!(score.neg().neg(), score);
    }

    #[test]
    fn neg_mate_is_cyclical() {
        let score = Score::Mate(MateDepth::new(-4).unwrap());
        assert_ne!(score.neg(), score);
        assert_eq!(score.neg().neg(), score);
    }
}
