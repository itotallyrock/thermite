use thermite_core::board::Board;
use thermite_core::board::fen::FenParseError;
use thermite_core::PlyCount;
use thermite_movegen::MoveGenerator;

pub type NodeCount = u64;

pub fn perft(mut board: Board, remaining_depth: PlyCount) -> NodeCount {
    if remaining_depth != 0 {
        board.generate_legal()
            .into_iter()
            .fold(0, |node_count, m| {
                #[cfg(debug_assertions)]
                let board_copy = board;
                let previous_state = board.make_move(m);
                let nodes = perft(board, remaining_depth - 1);
                board.unmake_move(m, previous_state);
                #[cfg(debug_assertions)]
                debug_assert_eq!(board, board_copy, "unmaking {m} was asymmetric");

                node_count + nodes
            })
    } else {
        1
    }
}

fn root_perft(mut board: Board, depth: PlyCount) {
    let mut total_nodes = 0;
    let mut root_moves = board.generate_legal();
    // Sort root moves alphabetically
    root_moves.sort_by_key(|m| m.to_string());

    for root_move in root_moves {
        let previous_state = board.make_move(root_move);
        let nodes = perft(board, depth - 1);

        total_nodes += nodes;
        println!("{root_move}: {nodes}");
        board.unmake_move(root_move, previous_state);
    }

    println!();
    println!("Nodes searched: {total_nodes}")
}

fn parse_input<S: AsRef<str>>(fen: S, depth: S) -> Result<(Board, PlyCount), &'static str>  {
    let (fen, depth) = (fen.as_ref(), depth.as_ref());
    let depth = depth.parse().ok().ok_or("invalid perft depth")?;
    let board = if fen.to_ascii_lowercase() == "startpos" {
        Board::starting_position()
    } else {
        Board::from_fen(fen).map_err(|err| match err {
            FenParseError::MissingPosition => "invalid position: missing position",
            FenParseError::MissingSide => "invalid position: missing side",
            FenParseError::MissingCastleRights => "invalid position: missing castle rights",
            FenParseError::MissingEnPassant => "invalid position: missing en passant",
            FenParseError::InvalidBoardDimensions => "invalid position: invalid board dimensions",
            FenParseError::IllegalSideChar(_) => "invalid position: illegal side char",
            FenParseError::IllegalEnPassant => "invalid position: illegal en passant",
            FenParseError::IllegalCastleRights => "invalid position: illegal castle rights",
            FenParseError::IllegalHalfmoveClock => "invalid position: illegal halfmoveC lock",
            FenParseError::IllegalFullmoveCounter => "invalid position: illegal fullmove counter",
        })?
    };

    Ok((board, depth))
}

fn arg_perft() -> Result<(), &'static str> {
    let position = std::env::args().nth(1).ok_or("missing perft position: 'perft [FEN] [SEARCH_DEPTH]'")?;
    let depth = std::env::args().nth(2).ok_or("missing perft depth: 'perft [FEN] [SEARCH_DEPTH]'")?;
    let (board, depth) = parse_input(position, depth)?;
    root_perft(board, depth);

    Ok(())
}

fn main() {
    if let Err(msg) = arg_perft() {
        eprintln!("{msg}");
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const POSITION_4_MIRRORED: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ";
    const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const POSITION_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    // Position 1 (Startpos)
    #[test_case(STARTPOS, 0, 1)]
    #[test_case(STARTPOS, 1, 20)]
    #[test_case(STARTPOS, 2, 400)]
    #[test_case(STARTPOS, 3, 8_902)]
    #[test_case(STARTPOS, 4, 197_281)]
    #[test_case(STARTPOS, 5, 4_865_609)]
    // Position 2 (Kiwipete)
    #[test_case(KIWIPETE, 1, 48)]
    #[test_case(KIWIPETE, 2, 2_039)]
    #[test_case(KIWIPETE, 3, 97_862)]
    #[test_case(KIWIPETE, 4, 4_085_603)]
    // Position 3
    #[test_case(POSITION_3, 1, 14)]
    #[test_case(POSITION_3, 2, 191)]
    #[test_case(POSITION_3, 3, 2_812)]
    #[test_case(POSITION_3, 4, 43_238)]
    #[test_case(POSITION_3, 5, 674_624)]
    // Position 4
    #[test_case(POSITION_4, 1, 6)]
    #[test_case(POSITION_4_MIRRORED, 1, 6)]
    #[test_case(POSITION_4, 2, 264)]
    #[test_case(POSITION_4_MIRRORED, 2, 264)]
    #[test_case(POSITION_4, 3, 9_467)]
    #[test_case(POSITION_4_MIRRORED, 3, 9_467)]
    #[test_case(POSITION_4, 4, 422_333)]
    #[test_case(POSITION_4_MIRRORED, 4, 422_333)]
    #[test_case(POSITION_4, 5, 15_833_292)]
    #[test_case(POSITION_4_MIRRORED, 5, 15_833_292)]
    // Position 5
    #[test_case(POSITION_5, 1, 44)]
    #[test_case(POSITION_5, 2, 1_486)]
    #[test_case(POSITION_5, 3, 62_379)]
    #[test_case(POSITION_5, 4, 2_103_487)]
    // Position 6
    #[test_case(POSITION_6, 1, 46)]
    #[test_case(POSITION_6, 2, 2_079)]
    #[test_case(POSITION_6, 3, 89_890)]
    #[test_case(POSITION_6, 4, 3_894_594)]
    // Others
    #[test_case("r4rk1/1pp1qBpp/p1np1n2/2b1p1B1/4P1b1/P1NP1N2/1PP1QPPP/R4RK1 b - - 0 10", 1, 4)]
    #[test_case("r3k2r/p1ppqpb1/bnN1pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 1 1", 1, 41)]
    #[test_case("8/2p5/3p4/1P6/K3Pp1r/6k1/6P1/1R6 b - e3 0 3", 2, 268)]
    #[test_case("8/2p5/3p4/1P6/K6r/4p1k1/6P1/1R6 w - - 0 4", 1, 4)]
    fn perft_works(fen: &str, depth: PlyCount, expected: NodeCount) {
        let board = Board::from_fen(fen).expect("illegal FEN");
        assert_eq!(perft(board, depth), expected);
    }

    #[ignore = "very slow to run"]
    // Position 1 (Startpos)
    #[test_case(STARTPOS, 6, 119_060_324)]
    #[test_case(STARTPOS, 7, 3_195_901_860)]
    // Position 2 (Kiwipete)
    #[test_case(KIWIPETE, 5, 193_690_690)]
    #[test_case(KIWIPETE, 6, 8_031_647_685)]
    // Position 3
    #[test_case(POSITION_3, 6, 11_030_083)]
    #[test_case(POSITION_3, 7, 178_633_661 )]
    #[test_case(POSITION_3, 8, 3_009_794_393)]
    // Position 5
    #[test_case(POSITION_5, 5, 89_941_194)]
    // Position 6
    #[test_case(POSITION_6, 5, 164_075_551)]
    #[test_case(POSITION_6, 6, 6_923_051_137)]
    fn perft_slow_works(fen: &str, depth: PlyCount, expected: NodeCount) {
        perft_works(fen, depth, expected);
    }
}