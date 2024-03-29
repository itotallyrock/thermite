use crate::half_move_clock::HALF_MOVE_LIMIT;
use crate::ply_count::PlyCount;
use crate::zobrist::{HistoryHash, ZobristHash};
use alloc::collections::{BTreeMap, VecDeque};

/// A hash container for keeping track of previously visited positions (for repetition checks)
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct HashHistory(VecDeque<HistoryHash>);

impl HashHistory {
    /// Create a new empty [`HashHistory`] container
    pub fn new() -> Self {
        Self(VecDeque::with_capacity(HALF_MOVE_LIMIT))
    }

    /// Push a new hash to the history, popping the oldest entry if beyond [`HALF_MOVE_LIMIT`]
    pub fn push(&mut self, hash: ZobristHash) {
        // Keep the VecDeque limited to HALF_MOVE_LIMIT and just shift earlier moves out
        if self.0.len() == HALF_MOVE_LIMIT {
            self.0.pop_front();
        }
        let hash = hash.into();
        self.0.push_back(hash);
    }

    /// Remove the most recently added hash
    pub fn pop(&mut self) {
        self.0.pop_back();
    }

    /// Clear the whole hash history because a non-reversible move has been made; meaning, all prior positions cannot be repeated anymore
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Get an iterator over all hashes with >= `N` repetitions (order is not preserved)
    pub fn repetitions<const N: u8>(&self) -> impl Iterator<Item = HistoryHash> {
        self.0
            .iter()
            .fold(
                BTreeMap::<HistoryHash, PlyCount>::new(),
                |mut repetitions, hash| {
                    repetitions
                        .entry(*hash)
                        .and_modify(PlyCount::increment)
                        .or_insert_with(|| PlyCount::new(1));

                    repetitions
                },
            )
            .into_iter()
            .filter_map(|(hash, repetitions)| (repetitions >= PlyCount::new(N)).then_some(hash))
    }
}

impl Default for HashHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::position::hash_history::HashHistory;
    use crate::zobrist::random_hash;

    #[test]
    fn repetitions_works() {
        let a = random_hash();
        let b = random_hash();

        let mut history = HashHistory::new();
        assert_eq!(history.repetitions::<1>().count(), 0);

        history.push(a);
        assert_eq!(history.repetitions::<1>().count(), 1);
        assert_eq!(history.repetitions::<2>().count(), 0);

        history.push(b);
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 0);

        history.push(a);
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 1);
        assert_eq!(history.repetitions::<3>().count(), 0);

        history.push(b);
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 2);
        assert_eq!(history.repetitions::<3>().count(), 0);

        history.push(a);
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 2);
        assert_eq!(history.repetitions::<3>().count(), 1);

        history.push(b);
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 2);
        assert_eq!(history.repetitions::<3>().count(), 2);

        history.pop();
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 2);
        assert_eq!(history.repetitions::<3>().count(), 1);

        history.pop();
        assert_eq!(history.repetitions::<1>().count(), 2);
        assert_eq!(history.repetitions::<2>().count(), 2);
        assert_eq!(history.repetitions::<3>().count(), 0);

        history.clear();
        assert_eq!(history.repetitions::<1>().count(), 0);
        assert_eq!(history.repetitions::<2>().count(), 0);
        assert_eq!(history.repetitions::<3>().count(), 0);
    }
}
