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
    pub fn increment(&mut self) -> Result<(), InvalidHalfMoveClock> {
        #[allow(clippy::cast_possible_truncation)]
        if self.0 < PlyCount::new(HALF_MOVE_LIMIT_USIZE as u8) {
            self.0.increment();
            Ok(())
        } else {
            Err(InvalidHalfMoveClock)
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub const HALF_MOVE_LIMIT_USIZE: usize = 50;

#[cfg(test)]
mod test {
    use crate::half_move_clock::{HalfMoveClock, InvalidHalfMoveClock};
    use crate::ply_count::PlyCount;
    use test_case::test_case;

    #[test_case(0, Ok(0))]
    #[test_case(1, Ok(1))]
    #[test_case(2, Ok(2))]
    #[test_case(3, Ok(3))]
    #[test_case(4, Ok(4))]
    #[test_case(5, Ok(5))]
    #[test_case(6, Ok(6))]
    #[test_case(7, Ok(7))]
    #[test_case(8, Ok(8))]
    #[test_case(9, Ok(9))]
    #[test_case(10, Ok(10))]
    #[test_case(25, Ok(25))]
    #[test_case(50, Ok(50))]
    #[test_case(51, Err(InvalidHalfMoveClock))]
    #[test_case(120, Err(InvalidHalfMoveClock))]
    #[test_case(250, Err(InvalidHalfMoveClock))]
    fn new_works(input: u8, expected: Result<u8, InvalidHalfMoveClock>) {
        let input = PlyCount::new(input);
        let expected = expected.map(|n| HalfMoveClock::new(PlyCount::new(n)).expect("invalid test setup"));
        assert_eq!(HalfMoveClock::new(input), expected);
    }

    #[test_case(0, Ok(1))]
    #[test_case(1, Ok(2))]
    #[test_case(2, Ok(3))]
    #[test_case(3, Ok(4))]
    #[test_case(4, Ok(5))]
    #[test_case(5, Ok(6))]
    #[test_case(6, Ok(7))]
    #[test_case(7, Ok(8))]
    #[test_case(8, Ok(9))]
    #[test_case(49, Ok(50))]
    #[test_case(50, Err(InvalidHalfMoveClock))]
    fn increment_works(input: u8, expected: Result<u8, InvalidHalfMoveClock>) {
        let mut input = HalfMoveClock::new(PlyCount::new(input)).expect("invalid test input");
        let expected = expected.map(|n| HalfMoveClock::new(PlyCount::new(n)).expect("invalid test setup"));
        assert_eq!(input.increment().map(|_| input), expected);
    }

    #[test]
    fn reset_works() {
        let mut input = HalfMoveClock::new(PlyCount::new(30)).expect("invalid test setup");
        let expected = HalfMoveClock::new(PlyCount::new(0)).expect("invalid test setup");
        input.reset();
        assert_eq!(input, expected);
    }
}
