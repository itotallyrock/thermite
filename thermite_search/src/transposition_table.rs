use arrayvec::ArrayVec;

use thermite_core::board::Board;
use thermite_core::board::zobrist::ZobristInner;
use thermite_core::chess_move::ChessMove;
use thermite_core::PlyCount;
use thermite_core::score::PositionEvaluation;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BoundedEvaluation {
    Upper(PositionEvaluation),
    Lower(PositionEvaluation),
    Exact(PositionEvaluation),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TranspositionTableEntry {
    pub key: ZobristInner,
    pub score: BoundedEvaluation,
    pub search_depth: PlyCount,
    pub best_move: ChessMove,
}

impl TranspositionTableEntry {
    /// Create a new exact [search score](BoundedEvaluation) entry for a given [position](Board) searched to a [depth](PlyCount)
    pub fn new_exact(board: &Board, search_depth: PlyCount, score: PositionEvaluation, best_move: ChessMove) -> Self {
        let key = board.zobrist_key();

        Self {
            key,
            score: BoundedEvaluation::Exact(score),
            search_depth,
            best_move,
        }
    }
    /// Create a new upper [search score](BoundedEvaluation) entry for a given [position](Board) searched to a [depth](PlyCount)
    pub fn new_upper(board: &Board, search_depth: PlyCount, score: PositionEvaluation, best_move: ChessMove) -> Self {
        let key = board.zobrist_key();

        Self {
            key,
            score: BoundedEvaluation::Upper(score),
            search_depth,
            best_move,
        }
    }
    /// Create a new lower [search score](BoundedEvaluation) entry for a given [position](Board) searched to a [depth](PlyCount)
    pub fn new_lower(board: &Board, search_depth: PlyCount, score: PositionEvaluation, best_move: ChessMove) -> Self {
        let key = board.zobrist_key();

        Self {
            key,
            score: BoundedEvaluation::Lower(score),
            search_depth,
            best_move,
        }
    }
}

type Bucket = ArrayVec<TranspositionTableEntry, 5>;

const TT_CAPACITY: usize = 256_000;

/// TODO
#[derive(Clone, Debug)]
pub struct TranspositionTable(Box<[Bucket; TT_CAPACITY]>);

impl Default for TranspositionTable {
    fn default() -> Self {
        let mut raw_bucket = Vec::with_capacity(TT_CAPACITY);
        for _ in 0..TT_CAPACITY {
            raw_bucket.push(Bucket::new());
        }

        Self(raw_bucket.try_into().unwrap())
    }
}

impl TranspositionTable {
    /// TODO
    fn get_index(key: ZobristInner) -> usize {
        key as usize % TT_CAPACITY
    }

    fn get_mut_bucket(&mut self, key: ZobristInner) -> &mut Bucket {
        self.0.get_mut(Self::get_index(key)).unwrap()
    }


    /// TODO
    pub fn upsert(&mut self, entry: TranspositionTableEntry) {
        let bucket = self.get_mut_bucket(entry.key);
        // Push to the bucket otherwise we need to try and replace an entry
        if bucket.try_push(entry).is_err() {
            if let Some(entry_to_replace) = bucket.iter_mut().find(|e| e.search_depth < entry.search_depth) {
                *entry_to_replace = entry;
            }
        }
    }

    pub fn try_make_exact(&mut self, board: &Board, best_move: ChessMove) -> bool {
        let board_key = board.zobrist_key();
        let bucket = self.get_mut_bucket(board_key);
        if let Some(entry) = bucket.iter_mut().find(|e| e.key == board_key) {
            // Update the entry
            entry.best_move = best_move;
            match entry.score {
                BoundedEvaluation::Lower(score) | BoundedEvaluation::Upper(score) => entry.score = BoundedEvaluation::Exact(score),
                BoundedEvaluation::Exact(_) => {},
            };

            true
        } else {
            false
        }
    }

    /// TODO
    pub fn lookup(&self, board: &Board) -> Option<TranspositionTableEntry> {
        let board_key = board.zobrist_key();

        self.0.get(Self::get_index(board_key))
            .and_then(|bucket| bucket.iter().find(|&&e| e.key == board_key))
            .copied()
    }
}