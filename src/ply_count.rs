use derive_more::{AsRef, Display, FromStr, Into};
use derive_new::new;

#[derive(
    new,
    Copy,
    Clone,
    Eq,
    Default,
    PartialEq,
    Debug,
    Hash,
    AsRef,
    Into,
    Display,
    FromStr,
    PartialOrd,
    Ord,
)]
pub struct PlyCount(u8);

impl PlyCount {
    pub fn increment(&mut self) {
        *self = Self::new(self.as_ref().saturating_add(1));
    }

    pub fn decrement(&mut self) {
        *self = Self::new(self.as_ref().saturating_sub(1));
    }
}
