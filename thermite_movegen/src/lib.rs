// const features
#![feature(
    const_mut_refs,
    const_trait_impl,
    const_option,
)]

pub use generator::MoveGenerator;
pub use move_list::LegalMoveContainer;

mod move_list;
mod pseudo_legal_move;
mod create_moves;
mod generator;

