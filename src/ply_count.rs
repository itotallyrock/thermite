use derive_more::{AsRef, Constructor, Display, FromStr, Into};

/// Represents a counter incrementing for a single player's move
#[derive(
    Constructor,
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
    /// Increment the counter by one, saturating at the max
    ///
    /// ```
    /// use thermite::ply_count::PlyCount;
    ///
    /// let mut a = PlyCount::new(0);
    /// a.increment();
    /// assert_eq!(a, PlyCount::new(1))
    /// ```
    pub fn increment(&mut self) {
        *self = Self::new(self.as_ref().saturating_add(1));
    }

    /// Decrease the counter by one, saturating at 0
    ///
    /// ```
    /// use thermite::ply_count::PlyCount;
    ///
    /// let mut a = PlyCount::new(1);
    /// a.decrement();
    /// assert_eq!(a, PlyCount::new(0))
    /// ```
    pub fn decrement(&mut self) {
        *self = Self::new(self.as_ref().saturating_sub(1));
    }
}

#[cfg(test)]
mod test {
    use crate::ply_count::PlyCount;

    #[test]
    fn increment_works() {
        let mut a = PlyCount::new(0);
        for i in 1..=256 {
            a.increment();
            assert_eq!(a.0, 255.min(i) as u8);
        }
    }

    #[test]
    fn decrement_works() {
        let mut a = PlyCount::new(255);
        for i in (1..255).rev() {
            a.decrement();
            assert_eq!(a.0, 0.max(i) as u8);
        }
    }
}