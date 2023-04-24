use nutype::nutype;

#[nutype(validate(max = 255))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, AsRef, Into, TryFrom, Display, FromStr)]
pub struct PlyCount(u8);

impl PlyCount {
    pub fn increment(&mut self) {
        *self = Self::new(self.into_inner().saturating_add(1)).unwrap();
    }

    pub fn decrement(&mut self) {
        *self = Self::new(self.into_inner().saturating_sub(1)).unwrap();
    }
}
