mod rights;

pub use rights::{CastleRights, IllegalCastleRights};

/// How many castle moves there are total.
/// 4 for white king side, white queen side, black king side, black queen side.
pub const NUM_CASTLES: usize = 4;
