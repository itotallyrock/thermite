use derive_more::{
    AsRef, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, LowerHex, Not, UpperHex,
};
use derive_new::new;

/// Board mask with single bits representing squares on a 64 tile board
#[derive(
    new,
    Copy,
    Clone,
    Eq,
    Default,
    PartialEq,
    PartialOrd,
    Hash,
    Debug,
    UpperHex,
    LowerHex,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Not,
    AsRef,
)]
#[must_use]
pub struct BoardMask(u64);
