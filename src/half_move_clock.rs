use nutype::nutype;

#[nutype(validate(max = 50))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct HalfMoveClock(u8);

impl Default for HalfMoveClock {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}

impl HalfMoveClock {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
