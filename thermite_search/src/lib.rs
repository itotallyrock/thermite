#![feature(let_chains, iter_intersperse, stmt_expr_attributes)]

use std::fmt::Debug;
use std::iter::repeat;
use std::time::Instant;

use arrayvec::ArrayVec;
#[cfg(feature = "tracing")]
use tracing::{debug, info, trace, instrument, trace_span, field::Empty};
use search_constraints::SearchConstraints;

use search_error::{RootSearchError, SearchError};
use search_inputs::SearchInputs;
use search_results::{SearchResult, SearchResults};
use search_state::SearchState;

use thermite_core::{PlyCount, STANDARD_MOVE_CLOCK};
use thermite_core::board::Board;
use thermite_core::chess_move::ChessMove;
#[cfg(feature = "history_heuristic")]
use thermite_core::move_type::MoveType;
use thermite_core::piece_type::{ByPieceType, PieceType};
use thermite_core::score::PositionEvaluation;
use thermite_evaluation::Evaluator;
use thermite_movegen::{LegalMoveContainer, MoveGenerator};
#[cfg(feature = "multipv")]
use crate::search_results::MAX_MULTI_PV_LINES;
#[cfg(feature = "transposition_table")]
use crate::transposition_table::{BoundedEvaluation, TranspositionTableEntry};

#[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "countermove_heuristic", feature = "piece_square_heuristic", feature = "static_exchange_eval"))]
mod move_ordering;
#[cfg(feature = "transposition_table")]
mod transposition_table;
pub mod search_results;
pub mod search_error;
mod search_state;
pub mod search_inputs;// TODO: Make these private once wrapped above iterative_deepened_ab bin example
pub mod halt_flag;// TODO: Make these private once wrapped above iterative_deepened_ab bin example
pub mod search_constraints;// TODO: Make these private once wrapped above iterative_deepened_ab bin example

/// How many [plies](PlyCount) deep from the first edge node (with the search depth initially reached 0) should we quiescent_search for
const MAX_Q_SEARCH_DEPTH: PlyCount = 8;

/// Maximum number of moves to store in a PV line for [results](SearchResults)
const MAX_PV_MOVES: usize = 8;
type PvMoveContainer = ArrayVec<Option<ChessMove>, MAX_PV_MOVES>;

/// Type representing how many positions, or nodes have been searched
pub type NodeCount = u64;

/// TODO
#[derive(Clone, Debug)]
pub struct Search {
    /// The position to search
    board: Board,
    /// The search state that changes frequently and is typically used for constraints or debugging
    state: SearchState,
    /// The search results
    search_results: SearchResults,
    /// The required inputs for initializing a search
    inputs: SearchInputs,
}

impl Search {

    /// TODO
    pub fn create(board: Board, inputs: SearchInputs, #[cfg(feature = "multipv")] multi_pv_limit: usize) -> Self {
        #[cfg(feature = "multipv")]
        let search_results = repeat(SearchResult::default())
            .take(multi_pv_limit.min(MAX_MULTI_PV_LINES))
            .collect();
        #[cfg(not(feature = "multipv"))]
        let search_results = SearchResults::default();
        let state = SearchState::default();

        Self {
            board,
            inputs,
            state,
            search_results,
        }
    }

    /// TODO
    pub fn start(mut self, search_constraints: SearchConstraints) -> Result<SearchResults, RootSearchError> {
        #[cfg(feature = "tracing")]
        info!("starting search for {:#?}", self.board);

        self.state.started_at = Some(Instant::now());
        let root_moves = self.board.generate_legal();

        if root_moves.is_empty() {
            let score = if self.board.in_check() {
                PositionEvaluation::DRAW
            } else {
                PositionEvaluation::MIN
            };

            if let Some(results) = self.get_mut_search_result() {
                results.evaluation = Some(score);
            }

            return Ok(self.search_results);
        }

        self.iterative_deepen_root(search_constraints, root_moves)?;

        #[cfg(feature = "tracing")]
        info!("search took {:.3}s covering {}nodes ({}nps) {}",
            self.state.started_at.unwrap().elapsed().as_secs_f32(),
            self.state.nodes,
            self.state.nodes_per_second(),
            self.get_search_result()
                .map(|results| results.principle_variation.iter()
                    .flat_map(|potential_best_move| potential_best_move.map(|best_move| best_move.to_string()))
                    .intersperse(String::from(' '))
                    .collect::<String>()
                ).unwrap_or(String::from("-")),
        );

        Ok(self.search_results)
    }

    /// TODO
    fn iterative_deepen_root(&mut self, search_constraints: SearchConstraints, mut root_moves: LegalMoveContainer) -> Result<(), RootSearchError> {
        let max_depth = search_constraints.depth();
        for search_constraints in (0..=max_depth).map(|search_depth| search_constraints.with_depth(search_depth)) {
            if search_constraints.should_halt(&self.state, &self.inputs.halt_flag) {
                break;
            }

            // println!("iterative deepening search to {}", search_constraints.depth());

            {
                // TODO: Sort root_moves to move any previous iterative deepening ordering hints to the front (pv, killers, etc)
                root_moves.sort_by_key(|&c| if self.board.gives_check(c) {
                    i32::MIN
                } else if self.board.piece_on(c.to).is_some() {
                    0
                } else {
                    300_000
                });

                // Move PV from previous iteration to the front
                if let Some(&Some(best_move)) = self.get_search_result().and_then(|result| result.principle_variation.first()) {
                    let best_index = root_moves.iter().position(|&m| m == best_move).unwrap();
                    root_moves.remove(best_index);
                    root_moves.insert(0, best_move);
                }
            }

            let score = match self.alpha_beta_root(PositionEvaluation::MIN, PositionEvaluation::MAX, search_constraints, &root_moves) {
                Err(error) => {
                    match error {
                        SearchError::Halted => break,
                        // Map some search errors to root errors and return
                        SearchError::FailedToReadTT => return Err(RootSearchError::FailedToReadTT),
                        SearchError::FailedToWriteTT => return Err(RootSearchError::FailedToWriteTT),
                        SearchError::FailedToWriteMoveOrdering => return Err(RootSearchError::FailedToWriteMoveOrdering),
                        SearchError::FailedToReadMoveOrdering => return Err(RootSearchError::FailedToReadMoveOrdering),
                    }
                }
                Ok(score) => score,
            };

            // TODO: USE FOR UCI
            // print!("info");
            // print!(" depth {}", search_constraints.depth());
            // print!(" seldepth {}", self.state.max_depth);
            // #[cfg(feature = "multipv")]
            // print!(" multipv {}", self.state.pv_index + 1);
            // print!(" score cp {}", self.get_search_result().evaluation.unwrap().centipawns());
            // print!(" nodes {}", self.state.nodes, );
            // print!(" nps {}", self.state.nodes_per_second());
            // print!(" time {}", self.state.started_at.unwrap().elapsed().as_secs_f64().ceil() as u64);
            // if !self.get_search_result().principle_variation.is_empty() {
            //     print!(" pv {}", self.get_search_result().principle_variation.iter().map(|m| m.to_string()).intersperse(String::from(' ')).collect::<String>());
            // }
            // println!();

            // If we found a mate we don't need to iterative deepen as we shouldn't find a mate in less moves by searching deeper
            if score.is_mating() {
                #[cfg(feature = "tracing")]
                info!("found checkmate exiting search early {}", score);
                break;
            }
        }

        Ok(())
    }

    /// TODO
    #[cfg_attr(feature = "tracing", instrument(level = "debug", name = "root_ab_search", err(Debug), ret(Display), skip_all, fields(depth = search_constraints.depth())))]
    fn alpha_beta_root(&mut self, mut alpha: PositionEvaluation, beta: PositionEvaluation, search_constraints: SearchConstraints, root_moves: &LegalMoveContainer) -> Result<PositionEvaluation, SearchError> {
        let mut best_move = *root_moves.first().unwrap();

        #[cfg(feature = "tracing")]
        debug!(%alpha, %beta, "searching root moves {}", root_moves.iter().map(|m| m.to_string()).intersperse(String::from(", ")).collect::<String>());

        for (pv_index, &root_move) in root_moves.iter().enumerate() {
            halt_check!(search_constraints, &self.state, &self.inputs.halt_flag);

            #[cfg(feature = "tracing")]
            debug!("evaluating root move {root_move} for {}", self.board.get_fen_string());

            self.state.increment_nodes();

            #[cfg(feature = "multipv")]
            { self.state.pv_index = pv_index; }

            let score = self.alpha_beta_next_move(root_move, alpha, beta, search_constraints)?;

            if score > alpha {
                #[cfg(feature = "tracing")]
                debug!("new best move {root_move} ({score})");
                alpha = score;
                best_move = root_move;
            }

            // TODO: Do we need to re-sort the results by highest score? (probably not, that should be done by whatever is printing the results?))
        }

        self.state.pv_index = 0;
        self.update_search_result(best_move, alpha);

        #[cfg(feature = "tracing")]
        {
            #[cfg(feature = "multipv")]
            let results = self.search_results.iter()
                .map(|r| &r.principle_variation)
                .filter(|pv| !pv.is_empty())
                .map(|pv| pv.iter()
                    .filter_map(|&m| m)
                    .map(|m| m.to_string())
                    .intersperse(String::from(" "))
                    .collect::<String>())
                .intersperse(String::from(",    "))
                .collect::<String>();
            #[cfg(not(feature = "multipv"))]
            let results = self.search_results.principle_variation.iter()
                .filter_map(|&m| m)
                .map(|m| m.to_string())
                .intersperse(String::from(" "))
                .collect::<String>();
            info!(results, "finished alpha beta search ({alpha} - {beta}) covering {}nodes in {:.3}s reaching a maximum depth of {}", self.state.nodes, self.state.started_at.unwrap().elapsed().as_secs_f32(), self.state.max_depth);
        }

        Ok(alpha)
    }

    fn alpha_beta(&mut self, mut alpha: PositionEvaluation, mut beta: PositionEvaluation, mut search_constraints: SearchConstraints) -> Result<PositionEvaluation, SearchError> {
        if search_constraints.should_halt(&self.state, &self.inputs.halt_flag) {
            #[cfg(feature = "tracing")]
            info!("halting search");
            return Err(SearchError::Halted);
        }
        self.state.increment_nodes();

        // TODO: Check for threefold repetition

        // Check for 50 move limit
        if self.board.as_ref().halfmove_clock() >= STANDARD_MOVE_CLOCK {
            #[cfg(feature = "tracing")]
            trace!("node evaluated as draw due to 50 move limit");
            return Ok(PositionEvaluation::DRAW);
        }

        // Get each legal move, if no moves are available then it's checkmate if in check otherwise stalemate
        let mut legal_moves = self.board.generate_legal();
        if legal_moves.is_empty() {
            return if self.board.in_check() {
                #[cfg(feature = "tracing")]
                trace!("node evaluated as checkmate");
                Ok(PositionEvaluation::MIN)
            } else {
                #[cfg(feature = "tracing")]
                trace!("node evaluated as stalemate");
                Ok(PositionEvaluation::DRAW)
            };
        }

        if self.board.in_check() {
            #[cfg(feature = "tracing")]
            trace!("in check extending search depth");
            search_constraints = search_constraints.with_extended_depth();
        }

        // Check if we have reached our maximum search depth and evaluate if so
        if search_constraints.depth() == 0 {
            #[cfg(feature = "q_search")]
            {
                self.state.q_search_depth = self.state.q_search_depth.saturating_add(1);
                let quiescent_eval = self.quiescence_search(-beta, -alpha, search_constraints.with_depth(MAX_Q_SEARCH_DEPTH)).map(|opposite_sides_evaluation| -opposite_sides_evaluation);
                self.state.q_search_depth = self.state.q_search_depth.saturating_sub(1);

                return quiescent_eval;
            }
            #[cfg(not(feature = "q_search"))]
            return Ok(PositionEvaluation::new_centipawns(self.board.material_evaluation()));
        }

        // Check for TT entry
        #[cfg(feature = "transposition_table")]
        if let Some(entry) = self.inputs.transposition_table.read().or(Err(SearchError::FailedToReadTT))?.lookup(&self.board) && entry.search_depth >= search_constraints.depth() {
            let unbounded_score;
            match entry.score {
                BoundedEvaluation::Exact(exact_score) => return Ok(exact_score),
                BoundedEvaluation::Lower(lower_bound) => {
                    unbounded_score = lower_bound;
                    alpha = alpha.max(lower_bound);
                },
                BoundedEvaluation::Upper(upper_bound) => {
                    unbounded_score = upper_bound;
                    beta = beta.min(upper_bound);
                }
            }
            if alpha >= beta {
                return Ok(unbounded_score);
            }
        }

        // Sort legal moves
        #[cfg(any(feature = "move_ordering", feature = "killer_heuristic", feature = "history_heuristic", feature = "countermove_heuristic", feature = "piece_square_heuristic", feature = "static_exchange_eval"))]
        self.inputs.move_ordering_state.read().or(Err(SearchError::FailedToReadMoveOrdering))?.sort_moves(&mut legal_moves, &self.board);

        let mut best_move = None;
        // Iterate over each legal move and evaluate the resulting position from the opposite perspective
        for legal_move in legal_moves {
            #[cfg(feature = "tracing")]
            trace!("evaluating {legal_move} for {}", self.board.get_fen_string());

            let score = self.alpha_beta_next_move(legal_move, alpha, beta, search_constraints)?;

            if score >= beta {
                self.beta_cutoff_move(legal_move, #[cfg(feature = "tracing")] beta, #[cfg(feature = "history_heuristic")] &search_constraints, #[cfg(feature = "transposition_table")] score)?;// TODO: FIXME
                return Ok(beta);
            }

            if score > alpha {
                #[cfg(feature = "tracing")]
                trace!(evaluation = %score, %alpha, chess_move = %legal_move, "exceeded alpha");
                alpha = score;
                #[cfg(feature = "transposition_table")]
                {
                    // Insert upper bound TT entry
                    best_move = Some(legal_move);
                    let entry = TranspositionTableEntry::new_upper(&self.board, search_constraints.depth(), score, legal_move);
                    self.inputs.transposition_table.write().or(Err(SearchError::FailedToWriteTT))?.upsert(entry);
                }
            }
        }

        // If a an alpha cutoff was set, and we've search every move update bounds and save pv entry
        if let Some(best_move) = best_move {
            // Use exact-bounds instead of upper-bound as we've searched every move and didn't find a beta cutoff
            #[cfg(feature = "transposition_table")]
            self.inputs.transposition_table.write().or(Err(SearchError::FailedToWriteTT))?.try_make_exact(&self.board, best_move);

            self.insert_pv_move(best_move);
        }

        Ok(alpha)
    }

    fn beta_cutoff_move(&mut self, chess_move: ChessMove, #[cfg(feature = "tracing")] beta: PositionEvaluation, #[cfg(feature = "history_heuristic")] search_constraints: &SearchConstraints, #[cfg(feature = "transposition_table")] score: PositionEvaluation) -> Result<(), SearchError> {
        #[cfg(feature = "tracing")]
        trace!(evaluation = %score, %beta, %chess_move, "beta cutoff");

        #[cfg(feature = "killer_heuristic")]
        self.inputs.move_ordering_state.write().or(Err(SearchError::FailedToWriteMoveOrdering))?.add_killer_move(chess_move);

        // If not a capture update move ordering
        #[cfg(feature = "history_heuristic")]
        if !matches!(chess_move.move_type, MoveType::Capture { .. } | MoveType::EnPassantCapture { .. } | MoveType::PromotingCapture { .. }) {
            self.inputs.move_ordering_state.write().or(Err(SearchError::FailedToWriteMoveOrdering))?.update_history_table(self.board.side_to_move(), chess_move, search_constraints.depth());
        }

        #[cfg(feature = "transposition_table")]
        {
            let entry = TranspositionTableEntry::new_lower(&self.board, search_constraints.depth(), score, chess_move);
            self.inputs.transposition_table.write().or(Err(SearchError::FailedToWriteTT))?.upsert(entry);
        }

        Ok(())
    }

    /// TODO
    fn alpha_beta_next_move(&mut self, chess_move: ChessMove, alpha: PositionEvaluation, beta: PositionEvaluation, search_constraints: SearchConstraints) -> Result<PositionEvaluation, SearchError> {
        let search_constraints = search_constraints.with_reduced_depth();
        #[cfg(feature = "tracing")]
        let span = trace_span!("ab", %chess_move, %alpha, %beta, depth = search_constraints.depth());
        #[cfg(feature = "tracing")]
        let _guard = span.enter();
        self.state.increment_search_depth(false);
        let previous_state = self.board.make_move(chess_move);
        let search_evaluation = self
            .alpha_beta(-beta, -alpha, search_constraints)
            .map(|opposite_sides_pov| -opposite_sides_pov);
        self.board.unmake_move(chess_move, previous_state);
        self.state.decrement_search_depth();

        search_evaluation
    }

    #[cfg(feature = "q_search")]
    pub fn quiescence_search(&mut self, mut alpha: PositionEvaluation, beta: PositionEvaluation, search_constraints: SearchConstraints) -> Result<PositionEvaluation, SearchError> {// TODO: Make private
        halt_check!(search_constraints, &self.state, &self.inputs.halt_flag);

        let evaluation = self.board.material_evaluation();
        self.state.increment_nodes();

        // To avoid extremely deep searches set a maximum search depth
        if self.state.q_search_depth >= MAX_Q_SEARCH_DEPTH {
            #[cfg(feature = "tracing")]
            trace!(%evaluation, "reached maximum quiescent search depth returning evaluation");
            return Ok(evaluation);
        }

        if evaluation >= beta {
            #[cfg(feature = "tracing")]
            trace!(%evaluation, %beta, "beta cutoff");
            return Ok(beta);
        }

        if evaluation > alpha {
            #[cfg(feature = "tracing")]
            trace!(%evaluation, %alpha, "exceeded alpha");
            alpha = evaluation;
        }

        // Get quiescent moves
        let mut quiescent_moves = self.board.generate_quiescent_moves();
        // TODO: Sort quiescent_moves
        {
            type QMoveScore = u8;
            // Captor then capture
            const MVV_LVA: ByPieceType<ByPieceType<QMoveScore>> = ByPieceType::new_with(
                ByPieceType::new_with(20, 30, 40, 40, 50, 0),
                ByPieceType::new_with(15, 25, 35, 35, 45, 0),
                ByPieceType::new_with(10, 20, 30, 30, 40, 0),
                ByPieceType::new_with(5, 15, 25, 25, 35, 0),
                ByPieceType::new_with(0, 10, 20, 20, 10, 0),
                ByPieceType::new_with(0, 5, 15, 15, 0, 0),
            );
            quiescent_moves.sort_by_key(|&m| {
                let check_bonus = if self.board.gives_check(m) { (MAX_Q_SEARCH_DEPTH - self.state.q_search_depth) as QMoveScore * 5 } else { 0 };
                let mvv_lva = match m.move_type {
                    MoveType::Quiet { .. } | MoveType::DoublePawnPush { .. } | MoveType::Castle { .. } | MoveType::Promotion { .. } => 0,
                    MoveType::Capture { piece_type, captured_piece } => *MVV_LVA.get_piece(piece_type).get_piece(captured_piece),
                    MoveType::EnPassantCapture { .. } => *MVV_LVA.get_piece(PieceType::Pawn).get_piece(PieceType::Pawn),
                    MoveType::PromotingCapture { captured_piece, .. } => 200 + *MVV_LVA.get_piece(PieceType::Pawn).get_piece(captured_piece),
                };

                mvv_lva.saturating_add(check_bonus)
            });
        }

        for q_move in quiescent_moves {
            // Make sure we don't need to exit the search early
            halt_check!(search_constraints, &self.state, &self.inputs.halt_flag);

            // Make the move and evaluate it
            let score = {
                // Record tracing information
                #[cfg(feature = "tracing")]
                let span = trace_span!("q", chess_move = %q_move, %alpha, %beta, evaluation = Empty);
                #[cfg(feature = "tracing")]
                let _guard = span.enter();

                // Make the move
                let move_state = self.board.make_move(q_move);
                self.state.increment_search_depth(true);

                // Evaluate the position
                let score = -if self.board.in_check() {
                    self.alpha_beta(-beta, -alpha, search_constraints.with_depth(0))
                } else {
                    self.quiescence_search(-beta, -alpha, search_constraints.with_reduced_depth())
                }?;

                // Unmake the move
                self.state.decrement_search_depth();
                self.board.unmake_move(q_move, move_state);

                // Record tracing information
                #[cfg(feature = "tracing")]
                span.record("evaluation", score.to_string());

                score
            };

            if score >= beta {
                #[cfg(feature = "tracing")]
                trace!(evaluation = %score, %beta, chess_move = %q_move, "beta cutoff");
                return Ok(beta);
            }

            if score > alpha {
                #[cfg(feature = "tracing")]
                trace!(evaluation = %score, %alpha, chess_move = %q_move, "exceeded alpha");
                alpha = score;
            }
        }


        Ok(alpha)
    }















    fn get_search_result(&self) -> Option<&SearchResult> {
        #[cfg(feature = "multipv")]
        let result = self.search_results.get(self.state.pv_index);
        #[cfg(not(feature = "multipv"))]
        let result = Some(&self.search_results);

        result
    }

    fn get_mut_search_result(&mut self) -> Option<&mut SearchResult> {
        #[cfg(feature = "multipv")]
        let result = self.search_results.get_mut(self.state.pv_index);
        #[cfg(not(feature = "multipv"))]
        let result = Some(&mut self.search_results);

        result
    }

    fn update_search_result(&mut self, best_move: ChessMove, score: PositionEvaluation) {
        self.insert_pv_move(best_move);
        if let Some(results) = self.get_mut_search_result() {
            results.evaluation = Some(score);
        }
    }

    /// Save a principle variation (or best [move](ChessMove) for a root line at the current [search depth](PlyCount)) to the [search result](SearchResult) for the current root move
    fn insert_pv_move(&mut self, best_move: ChessMove) {
        let search_depth = self.state.search_depth as usize;
        if let Some(results) = self.get_mut_search_result() {
            let pv_container = &mut results.principle_variation;
            if search_depth < pv_container.len() {
                pv_container.insert(search_depth, Some(best_move));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    use thermite_core::square::Square::*;
    use thermite_core::move_type::MoveType::*;
    use thermite_core::piece_type::PieceType::*;

    // https://www.ideachess.com/chess_tactics_puzzles/checkmate_n/61738
    #[test_case(Board::from_fen("7r/5pk1/2Q2bpp/3p4/P2q3P/1P6/2P5/1K3R2 b - - 0 1").unwrap(), 1, ChessMove { move_type: Quiet { piece_type: Queen }, from: D4, to: A1 })]
    // https://www.ideachess.com/chess_tactics_puzzles/checkmate_n/10519
    #[test_case(Board::from_fen("r7/7k/b1p1p3/1p1pP1Q1/p2P4/2P5/PP6/1K1R4 w - - 0 2").unwrap(), 1, ChessMove { move_type: Quiet { piece_type: Rook }, from: D1, to: H1 })]
    #[test_case(Board::from_fen("r7/7k/b1p1p3/pp1pP1r1/3P4/2P5/PP4Q1/1K1R4 w - - 0 1").unwrap(), 3, ChessMove { move_type: Capture { piece_type: Queen, captured_piece: Rook }, from: G2, to: G5 })]
    // https://www.ideachess.com/chess_tactics_puzzles/checkmate_n/47212
    #[test_case(Board::from_fen("2r1r1k1/1pR4p/5pp1/p4N2/3Q1P2/8/1P4qP/1K1R4 w - - 0 1").unwrap(), 3, ChessMove { move_type: Quiet { piece_type: Knight }, from: F5, to: H6 })]
    fn finds_mate(board: Board, mate_in: PlyCount, expected_mate: ChessMove) {
        let constraints = SearchConstraints::new().with_depth(mate_in);
        let results = Search::create(board, SearchInputs::default(), #[cfg(feature = "multipv")] MAX_PV_MOVES).start(constraints).unwrap();
        #[cfg(feature = "multipv")]
        assert_eq!(results.iter().filter_map(|r| r.principle_variation.first().copied().flatten()).next().unwrap(), expected_mate);
        #[cfg(not(feature = "multipv"))]
        assert_eq!(*results.principle_variation.first().unwrap(), expected_mate);
    }

    // #[test_case(Board::from_fen("r1b2r2/pp1nbpk1/1q2p1B1/3pP2Q/5P2/1P5R/PBPN2PP/R6K b - - 1 1"))]
    // https://www.ideachess.com/chess_tactics_puzzles/checkmate_n/1386
    #[test_case(Board::from_fen("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4").unwrap(), 1)]
    #[test_case(Board::from_fen("1r2r1k1/p4pbp/6p1/8/8/5QPb/PPP2PBP/R1BN2K1 b - - 0 3").unwrap(), 3)]
    #[test_case(Board::from_fen("1r2r1k1/p4pbp/6p1/4n3/5Q2/6Pb/PPP2PBP/R1BN2K1 b - - 0 2").unwrap(), 5)]
    #[test_case(Board::from_fen("1r2r1k1/p2q1pbp/6p1/4n3/5Q2/2N3Pb/PPP2PBP/R1BR2K1 b - - 0 1").unwrap(), 7)]
    fn q_search_finds_mate(board: Board, expected_mate_plies: PlyCount) {
        let results = Search::create(board, SearchInputs::default(), #[cfg(feature = "multipv")] MAX_PV_MOVES)
            .quiescence_search(PositionEvaluation::MIN, PositionEvaluation::MAX, SearchConstraints::new())
            .unwrap();
        assert_eq!(results, PositionEvaluation::new_mating(expected_mate_plies));
    }
}
