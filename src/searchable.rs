use derive_more::{AsMut, AsRef};
use crate::half_move_clock::Game;

#[derive(Clone, Eq, PartialEq, Debug, AsRef, AsMut)]
pub struct Searchable {
    #[as_ref()]
    #[as_mut()]
    game: Game,
}
