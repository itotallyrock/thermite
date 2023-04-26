use nutype::nutype;

#[nutype]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BoardMask(u64);

impl Default for BoardMask {
    fn default() -> Self {
        Self::new(0)
    }
}
