use crate::bitboard::BoardMask;
use crate::pieces::{NonPawnPieceType, PieceType, SlidingPieceType};
use crate::square::Square;
use enum_iterator::all;
use enum_map::EnumMap;
use once_cell::sync::Lazy;

/// Get an iterator over all square combinations, including duplicate start & end pairs
fn all_square_pairs() -> impl Iterator<Item = (Square, Square)> {
    all::<Square>().flat_map(|a_square| all::<Square>().map(move |b_square| (a_square, b_square)))
}

/// Get an iterator over all non-duplicate square combinations, where the pair will never have the same `a_square` as its `b_square`
fn separate_square_pairs() -> impl Iterator<Item = (Square, Square)> {
    all_square_pairs().filter(|&(a_square, b_square)| a_square != b_square)
}

/// Get an iterator over squares that share a diagonal or cardinal line, the piece type that connected, and the attacks from that piece from the first square
fn connectable_iter() -> impl Iterator<Item = (Square, Square, NonPawnPieceType, BoardMask)> {
    separate_square_pairs().filter_map(|(a_square, b_square)| {
        // Loop over the two types of sliding pieces and get their attacks and keep the piece that intersects, if any, and its attack mask
        [NonPawnPieceType::Rook, NonPawnPieceType::Bishop]
            .into_iter()
            .map(|piece| {
                (
                    a_square,
                    b_square,
                    piece,
                    BoardMask::pseudo_attacks_for(piece, a_square),
                )
            })
            .find(|&(_, b_square, _, a_attacks)| a_attacks & b_square.to_mask() != BoardMask::EMPTY)
    })
}

/// The intersection between two aligned squares including the second (end) [`Square`]'s mask
/// If non-aligned, only the end [`Square`]'s mask will be included.
static LINE_BETWEEN: Lazy<EnumMap<Square, EnumMap<Square, BoardMask>>> = Lazy::new(|| {
    let mut items: EnumMap<Square, EnumMap<Square, BoardMask>> = EnumMap::default();

    // Add the intersection of the aligned sliding attacks from both squares to get the in-between
    for (a, b, piece, _) in connectable_iter() {
        let piece = SlidingPieceType::try_from(PieceType::from(piece)).unwrap();
        items[a][b] = BoardMask::sliding_attacks_for(piece, a, a.to_mask() | b.to_mask())
            & BoardMask::sliding_attacks_for(piece, b, b.to_mask() | a.to_mask());
    }

    // Add destination square always
    for (a, b) in all_square_pairs() {
        items[a][b] |= b.to_mask();
    }

    items
});

/// Full board crossing line through two aligned [squares](Square)
static LINE_THROUGH: Lazy<EnumMap<Square, EnumMap<Square, BoardMask>>> = Lazy::new(|| {
    let mut items: EnumMap<Square, EnumMap<Square, BoardMask>> = EnumMap::default();

    for (a, b, piece, a_attacks) in connectable_iter() {
        items[a][b] =
            (a_attacks & BoardMask::pseudo_attacks_for(piece, b)) | b.to_mask() | a.to_mask();
    }

    items
});

impl BoardMask {
    /// Get the board mask of the line through two squares, if any, the line extends from edge to edge.
    pub fn line_through(start: Square, end: Square) -> Self {
        LINE_THROUGH[start][end]
    }

    /// Get the board mask of the line between two squares, if any, not including the start squares.
    pub fn line_between(start: Square, end: Square) -> Self {
        LINE_BETWEEN[start][end]
    }

    /// If three squares share a common rank, file, or diagonal
    #[must_use]
    pub fn is_aligned(a: Square, b: Square, c: Square) -> bool {
        !(Self::line_through(a, b) & c.to_mask()).is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::BoardMask;
    use crate::square::Square;
    use crate::square::Square::*;
    use test_case::test_case;

    #[test_case(A2, A4, A6, true)]
    #[test_case(A2, A4, A8, true)]
    #[test_case(B2, A4, A8, false)]
    #[test_case(A2, B4, A8, false)]
    #[test_case(A2, A4, B8, false)]
    #[test_case(A2, B4, B8, false)]
    #[test_case(B2, B4, B8, true)]
    #[test_case(H1, A1, C1, true)]
    #[test_case(H1, A1, C2, false)]
    #[test_case(H8, A1, D4, true)]
    fn is_aligned_works(a: Square, b: Square, c: Square, expected: bool) {
        assert_eq!(BoardMask::is_aligned(a, b, c), expected);
    }

    #[test_case(C4, F7, BoardMask(0x0020_1008_0000_0000))]
    #[test_case(E6, F8, BoardMask(0x2000_0000_0000_0000))]
    // A1-H8 diagonal
    #[test_case(A1, H8, BoardMask(0x8040_2010_0804_0200))]
    #[test_case(A1, G7, BoardMask(0x0040_2010_0804_0200))]
    #[test_case(A1, F6, BoardMask(0x2010_0804_0200))]
    #[test_case(A1, E5, BoardMask(0x0010_0804_0200))]
    #[test_case(B2, E5, BoardMask(0x0010_0804_0000))]
    #[test_case(B2, D4, BoardMask(0x0804_0000))]
    #[test_case(B3, D4, BoardMask(0x0800_0000))]
    // G2-G6 vertical
    #[test_case(G2, G6, BoardMask(0x4040_4040_0000))]
    #[test_case(G3, G6, BoardMask(0x4040_4000_0000))]
    #[test_case(G4, G6, BoardMask(0x4040_0000_0000))]
    #[test_case(G4, G5, BoardMask(0x0040_0000_0000))]
    // F5-A5 horizontal
    #[test_case(F5, A5, BoardMask(0x001F_0000_0000))]
    #[test_case(E5, A5, BoardMask(0x000F_0000_0000))]
    #[test_case(D5, A5, BoardMask(0x0007_0000_0000))]
    #[test_case(D5, B5, BoardMask(0x0006_0000_0000))]
    #[test_case(D5, C5, BoardMask(0x0004_0000_0000))]
    // Non aligned between
    #[test_case(A5, B7, B7.to_mask())]
    #[test_case(H1, C8, C8.to_mask())]
    #[test_case(E4, C1, C1.to_mask())]
    #[test_case(E4, D1, D1.to_mask())]
    #[test_case(E4, F1, F1.to_mask())]
    #[test_case(E4, G1, G1.to_mask())]
    #[test_case(E4, E4, E4.to_mask())]
    #[test_case(H8, H8, H8.to_mask())]
    fn line_between_works(a: Square, b: Square, expected: BoardMask) {
        assert_eq!(BoardMask::line_between(a, b), expected);
    }

    // Non aligned
    #[test_case(A1, B5, BoardMask(0x0))]
    #[test_case(A1, B4, BoardMask(0x0))]
    #[test_case(A1, C4, BoardMask(0x0))]
    // Diagonal A1-H8
    #[test_case(A1, D4, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(B2, D4, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(C3, D4, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(D4, C3, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(D4, E5, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(D4, H8, BoardMask(0x8040_2010_0804_0201))]
    #[test_case(A1, H8, BoardMask(0x8040_2010_0804_0201))]
    // Diagonal A8-H1
    #[test_case(A8, D5, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(B7, D5, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(C6, D5, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(D5, C6, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(D5, E4, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(D5, H1, BoardMask(0x0102_0408_1020_4080))]
    #[test_case(A8, H1, BoardMask(0x0102_0408_1020_4080))]
    // Non-major diagonal D8-H4
    #[test_case(E7, G5, BoardMask(0x0810_2040_8000_0000))]
    #[test_case(G5, E7, BoardMask(0x0810_2040_8000_0000))]
    #[test_case(G5, H4, BoardMask(0x0810_2040_8000_0000))]
    #[test_case(D8, H4, BoardMask(0x0810_2040_8000_0000))]
    // Vertical G1-G4
    #[test_case(G1, G4, BoardMask(0x4040_4040_4040_4040))]
    #[test_case(G1, G3, BoardMask(0x4040_4040_4040_4040))]
    #[test_case(G1, G2, BoardMask(0x4040_4040_4040_4040))]
    #[test_case(G4, G1, BoardMask(0x4040_4040_4040_4040))]
    // Horizontal A5-F5
    #[test_case(A5, F5, BoardMask(0x00FF_0000_0000))]
    #[test_case(A5, E5, BoardMask(0x00FF_0000_0000))]
    #[test_case(A5, D5, BoardMask(0x00FF_0000_0000))]
    #[test_case(A5, C5, BoardMask(0x00FF_0000_0000))]
    #[test_case(B5, C5, BoardMask(0x00FF_0000_0000))]
    #[test_case(C5, F5, BoardMask(0x00FF_0000_0000))]
    fn line_through_works(a: Square, b: Square, expected: BoardMask) {
        assert_eq!(BoardMask::line_through(a, b), expected);
    }
}
