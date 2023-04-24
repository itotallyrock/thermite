use nutype::nutype;
use std::hash::Hasher;

#[nutype]
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct HistoryHash(u8);

#[nutype]
#[derive(Copy, Clone, Eq, PartialEq, Debug, AsRef)]
pub struct ZobristHash(u64);

impl Hasher for ZobristHash {
    fn finish(&self) -> u64 {
        self.into_inner()
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes
            .chunks_exact(u64::BITS as usize / 8)
            .map(|bits| u64::from_be_bytes(bits.try_into().unwrap()))
            .for_each(|chunk| self.write_u64(chunk));
    }

    fn write_u64(&mut self, i: u64) {
        *self = Self::new(self.into_inner() ^ i);
    }
}

impl Default for ZobristHash {
    fn default() -> Self {
        Self::new(0xF1DC_4349_4EA4_76CE)
    }
}

impl From<ZobristHash> for HistoryHash {
    fn from(value: ZobristHash) -> Self {
        // Intentional truncation for a smaller memory footprint with still enough bits to avoid a hash collision
        #[allow(clippy::cast_possible_truncation)]
        Self::new(*value.as_ref() as u8)
    }
}

impl PartialEq<HistoryHash> for ZobristHash {
    fn eq(&self, other: &HistoryHash) -> bool {
        HistoryHash::from(*self) == *other
    }
}
