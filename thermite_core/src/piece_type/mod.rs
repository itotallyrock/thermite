mod by_piece_type;

pub use by_piece_type::ByPieceType;

/// The number of piece types in standard chess
pub const NUM_PIECE_TYPES: usize = 6;

/// A specific type of piece that can occupy a square or be moved.
#[cfg_attr(feature = "hashable_chess_move", derive(Hash))]
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    /// A pawn that can only push forward one square at a time, except for its first move, which can move two squares forward (if unobstructed).
    /// Can also attack diagonally forward-left/right.  Upon reaching the opposite sides back-rank the pawn will promote to a promotion piece ([`Queen`](Self::Queen), [`Rook`](Self::Rook), [`Knight`](Self::Knight), [`Bishop`](Self::Bishop)).
    Pawn,
    /// A knight can move in an L shape (2x1) and can move to any square.
    Knight,
    /// A bishop can move diagonally on either axis as far is it can without capturing or bumping into its own side's pieces.
    /// A bishop is locked to the specific colored square it starts on.
    Bishop,
    /// A rook can move cardinally on either axis as far is it can without capturing or bumping into its own side's pieces.
    /// A rook can also [castle](crate::castles::CastleRights) or semi-switch places with its king.
    Rook,
    /// Most powerful piece, featuring the same moves as [`Bishop`](Self::Bishop) and [`Rook`](Self::Rook) combined.
    Queen,
    /// The piece that must be protected in order to win.  Your king being attacked is checked, and
    /// ending your turn with your king attacked is check-mate (a loss).  A king can move/capture a single
    /// square in each direction (if each square is not attacked).
    King,
}

impl PieceType {
    /// Array of all six pieces (Pawn, Knight, Bishop, Rook, Queen, King)
    pub const PIECES: [Self; NUM_PIECE_TYPES] = [Self::Pawn, Self::Knight, Self::Bishop, Self::Rook, Self::Queen, Self::King];

    /// Get the UCI standard character representation for a piece in lowercase.
    /// - `Self::Pawn` - `'p'`
    /// - `Self::Knight` - `'n'`
    /// - `Self::Bishop` - `'b'`
    /// - `Self::Rook` - `'r'`
    /// - `Self::Queen` - `'q'`
    /// - `Self::King` - `'k'`
    #[must_use]
    pub const fn get_lower_char(&self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }

    /// Get the UCI standard character representation for a piece in uppercase.
    /// - `Self::Pawn` - `'P'`
    /// - `Self::Knight` - `'N'`
    /// - `Self::Bishop` - `'B'`
    /// - `Self::Rook` - `'R'`
    /// - `Self::Queen` - `'Q'`
    /// - `Self::King` - `'K'`
    #[must_use]
    pub const fn get_upper_char(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn option_piece_type_is_one_byte() {
        assert_eq!(std::mem::size_of::<Option<PieceType>>(), 1);
        assert_eq!(std::mem::size_of_val(&Some(PieceType::King)), 1);
        assert_eq!(std::mem::size_of_val::<Option<PieceType>>(&None), 1);
    }

    #[test]
    fn pieces_ordering_is_consistent() {
        assert_eq!(PieceType::PIECES[0], PieceType::Pawn);
        assert_eq!(PieceType::PIECES[1], PieceType::Knight);
        assert_eq!(PieceType::PIECES[2], PieceType::Bishop);
        assert_eq!(PieceType::PIECES[3], PieceType::Rook);
        assert_eq!(PieceType::PIECES[4], PieceType::Queen);
        assert_eq!(PieceType::PIECES[5], PieceType::King);
        assert_eq!(PieceType::PIECES[NUM_PIECE_TYPES - 1], PieceType::King);
        assert!(PieceType::PIECES.is_sorted());
    }

    #[test_case(PieceType::Pawn, 'p')]
    #[test_case(PieceType::Knight, 'n')]
    #[test_case(PieceType::Bishop, 'b')]
    #[test_case(PieceType::Rook, 'r')]
    #[test_case(PieceType::Queen, 'q')]
    #[test_case(PieceType::King, 'k')]
    fn get_lower_char_works(piece: PieceType, expected: char) {
        assert_eq!(piece.get_lower_char(), expected);
    }

    #[test_case(PieceType::Pawn, 'P')]
    #[test_case(PieceType::Knight, 'N')]
    #[test_case(PieceType::Bishop, 'B')]
    #[test_case(PieceType::Rook, 'R')]
    #[test_case(PieceType::Queen, 'Q')]
    #[test_case(PieceType::King, 'K')]
    fn get_upper_char_works(piece: PieceType, expected: char) {
        assert_eq!(piece.get_upper_char(), expected);
    }
}
