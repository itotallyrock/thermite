use crate::bitboard::BoardMask;
use crate::chess_move::capture::Capture;
use crate::chess_move::quiet::Quiet;
use crate::chess_move::ChessMove;
use crate::pieces::{NonKingPieceType, Piece, PieceType};
use crate::position::LegalPosition;
use crate::square::Square;

mod evasions;
mod non_evasion;
mod pawns;

impl LegalPosition {
    /// TODO
    // TODO: Wrap in a ScoredChessMove which contains a score for ordering and impl Ord and use in a BinaryHeap?
    #[must_use]
    pub fn generate_legal_moves(&self) -> Vec<ChessMove> {
        if self.in_check() {
            self.generate_evasion_moves().collect()
        } else {
            self.get_non_evasion_moves().collect()
        }
    }

    /// Create a quiet move for the current player
    /// # Panics
    /// - If from and to are the same square
    fn create_quiet(&self, from: Square, to: Square, piece: PieceType) -> Quiet {
        Quiet::new(from, to, piece.owned_by(self.player_to_move))
            .expect("attempting to create capturing attacking itself")
    }

    /// Get the piece on a square being captured
    /// # Panics
    /// - If there is no piece on the square
    /// - If the captured piece is a king
    fn get_captured_piece(&self, to: Square) -> NonKingPieceType {
        let captured_piece = self
            .piece_type_on(to)
            .expect("cannot capture to empty square");

        NonKingPieceType::try_from(captured_piece).expect("cannot capture king")
    }

    /// Create a capture move for the current player
    /// # Panics
    /// - if from and to are the same square
    /// - If there is no piece on the square
    /// - If the captured piece is a king
    fn create_capture(&self, from: Square, to: Square, piece: PieceType) -> Capture {
        let quiet = self.create_quiet(from, to, piece);
        let captured_piece = self.get_captured_piece(to);
        Capture::new(quiet, captured_piece)
    }

    /// Check if a piece moving is pinned or not
    fn is_non_pinned_piece(&self, from: Square, to: Square) -> bool {
        (from.to_mask() & self.state.blockers_for[self.player_to_move]).is_empty()
            || BoardMask::is_aligned(from, to, self.king_squares[self.player_to_move])
    }
}

#[cfg(test)]
mod test {
    use crate::node_count::NodeCount;
    use crate::ply_count::PlyCount;
    use crate::position::LegalPosition;

    use crate::fen;
    use test_case::test_case;

    pub fn perft<const IS_ROOT: bool>(position: &mut LegalPosition, depth: PlyCount) -> NodeCount {
        if depth == PlyCount::new(0) {
            NodeCount::new(1)
        } else {
            position
                .generate_legal_moves()
                .into_iter()
                .map(|m| {
                    let state = position.make_move(m);
                    let nodes = perft::<false>(position, depth - PlyCount::new(1));
                    if IS_ROOT {
                        println!("{m}: {nodes}");
                    }
                    position.unmake_move(m, state);

                    nodes
                })
                .sum()
        }
    }

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const STARTPOS_C2C3: &str = "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1";
    const STARTPOS_C2C3_D7D5: &str = "rnbqkbnr/ppp1pppp/8/3p4/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 2";
    const STARTPOS_C2C3_D7D5_D1A4: &str =
        "rnbqkbnr/ppp1pppp/8/3p4/Q7/2P5/PP1PPPPP/RNB1KBNR b KQkq - 1 2";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const KIWIPETE_E5G7: &str =
        "r3k2r/p1ppqpN1/bn2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1";
    const KIWIPETE_E5D7: &str =
        "r3k2r/p1pNqpb1/bn2pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1";
    const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const POSITION_4_MIRRORED: &str =
        "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ";

    #[test_case(STARTPOS, PlyCount::new(0), NodeCount::new(1))]
    #[test_case(STARTPOS, PlyCount::new(1), NodeCount::new(20))]
    #[test_case(STARTPOS, PlyCount::new(2), NodeCount::new(400))]
    #[test_case(STARTPOS, PlyCount::new(3), NodeCount::new(8_902))]
    #[test_case(STARTPOS_C2C3, PlyCount::new(3), NodeCount::new(9_272))]
    #[test_case(STARTPOS_C2C3_D7D5, PlyCount::new(2), NodeCount::new(566))]
    #[test_case(STARTPOS_C2C3_D7D5_D1A4, PlyCount::new(1), NodeCount::new(6))]
    #[test_case(STARTPOS, PlyCount::new(4), NodeCount::new(197_281))]
    #[test_case(STARTPOS, PlyCount::new(5), NodeCount::new(4_865_609))]
    #[test_case(KIWIPETE, PlyCount::new(1), NodeCount::new(48))]
    #[test_case(KIWIPETE, PlyCount::new(2), NodeCount::new(2_039))]
    #[test_case(KIWIPETE_E5D7, PlyCount::new(1), NodeCount::new(45))]
    #[test_case(KIWIPETE_E5G7, PlyCount::new(1), NodeCount::new(2))]
    #[test_case(KIWIPETE_E5G7, PlyCount::new(2), NodeCount::new(92))]
    #[test_case(KIWIPETE, PlyCount::new(3), NodeCount::new(97_862))]
    #[test_case(KIWIPETE, PlyCount::new(4), NodeCount::new(4_085_603))]
    #[test_case(POSITION_3, PlyCount::new(1), NodeCount::new(14))]
    #[test_case(POSITION_3, PlyCount::new(2), NodeCount::new(191))]
    #[test_case(POSITION_3, PlyCount::new(3), NodeCount::new(2_812))]
    #[test_case(POSITION_3, PlyCount::new(4), NodeCount::new(43_238))]
    #[test_case(POSITION_3, PlyCount::new(5), NodeCount::new(674_624))]
    #[test_case(POSITION_4, PlyCount::new(1), NodeCount::new(6))]
    #[test_case(POSITION_4_MIRRORED, PlyCount::new(1), NodeCount::new(6))]
    #[test_case(POSITION_4, PlyCount::new(2), NodeCount::new(264))]
    #[test_case(POSITION_4_MIRRORED, PlyCount::new(2), NodeCount::new(264))]
    #[test_case(POSITION_4, PlyCount::new(3), NodeCount::new(9_467))]
    #[test_case(POSITION_4_MIRRORED, PlyCount::new(3), NodeCount::new(9_467))]
    #[test_case(POSITION_4, PlyCount::new(4), NodeCount::new(422_333))]
    #[test_case(POSITION_4_MIRRORED, PlyCount::new(4), NodeCount::new(422_333))]
    fn perft_works(fen: &str, depth: PlyCount, expected_nodes: NodeCount) {
        assert_eq!(perft::<true>(&mut fen!(fen), depth), expected_nodes);
    }
}
