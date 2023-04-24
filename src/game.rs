use derive_more::{AsMut, AsRef};
use crate::legal_position::LegalPosition;
use crate::ply_count::PlyCount;

#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Game {
    #[as_ref()]
    #[as_mut()]
    legal_position: LegalPosition,
    halfmove_count: PlyCount,
}
