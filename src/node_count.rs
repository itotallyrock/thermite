use derive_more::{
    Add, AddAssign, AsRef, Constructor, Display, FromStr, Into, Sub, SubAssign, Sum,
};

/// Represents a counter incrementing for a single position visited in a search
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
    Sub,
    SubAssign,
    Add,
    AddAssign,
    Sum,
    PartialOrd,
    Ord,
)]
pub struct NodeCount(u64);
