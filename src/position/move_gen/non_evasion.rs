use crate::bitboard::BoardMask;
use crate::chess_move::castle::Castle;
use crate::chess_move::ChessMove;
use crate::pieces::{
    NonKingPieceType, NonPawnPieceType, PieceType, PromotablePieceType, SlidingPieceType,
};
use crate::position::LegalPosition;
use crate::square::Square;

impl LegalPosition {
    /// TODO
    pub(super) fn get_non_evasion_moves(&self) -> impl Iterator<Item = ChessMove> + '_ {
        let targets = self.attackable_mask();
        self.generate_non_king_moves(targets)
            .chain(self.generate_king_moves(targets))
    }

    pub(super) fn generate_non_king_moves(
        &self,
        targets: BoardMask,
    ) -> impl Iterator<Item = ChessMove> + '_ {
        self.generate_pawn_moves(targets)
            .chain(self.generate_promotable_piece_type_moves(PromotablePieceType::Knight, targets))
            .chain(self.generate_promotable_piece_type_moves(PromotablePieceType::Bishop, targets))
            .chain(self.generate_promotable_piece_type_moves(PromotablePieceType::Rook, targets))
            .chain(self.generate_promotable_piece_type_moves(PromotablePieceType::Queen, targets))
    }

    fn generate_king_moves(&self, target: BoardMask) -> impl Iterator<Item = ChessMove> + '_ {
        self.generate_non_castling_king_moves(target)
            .chain(self.generate_castle_moves())
    }

    /// Generate pseudo-legal king/rook castle moves
    /// - Could be illegal if the castle path is attacked
    fn generate_castle_moves(&self) -> impl Iterator<Item = ChessMove> + '_ {
        let occupied_mask = self.occupied_mask();
        Castle::all_for_player(self.player_to_move)
            .filter(|castle| self.state.castles.has_rights(castle.required_rights()))
            .filter(move |castle| (occupied_mask & castle.unattacked_mask()).is_empty())
            .map(ChessMove::Castle)
    }

    /// Generate pseudo-legal king moves, without castles
    /// - Could be illegal if the king's destination square is attacked
    pub(super) fn generate_non_castling_king_moves(
        &self,
        target: BoardMask,
    ) -> impl Iterator<Item = ChessMove> + '_ {
        let from = self.king_squares[self.player_to_move];
        let king_attacks = BoardMask::pseudo_attacks_for(NonPawnPieceType::King, from) & target;

        let captures_mask = king_attacks & self.opposite_player_mask();
        let captures = captures_mask
            .into_iter()
            .map(move |king_to| self.create_capture(from, king_to, PieceType::King))
            .map(ChessMove::Capture);

        let quiet_mask = king_attacks & self.empty_mask();
        let quiets = quiet_mask
            .into_iter()
            .map(move |king_to| self.create_quiet(from, king_to, PieceType::King))
            .map(ChessMove::Quiet);

        captures.chain(quiets)
    }

    /// Generate pseudo-legal moves for a promotable-piece (non-pawn/king)
    /// - Could be illegal if the piece is pinned
    fn generate_promotable_piece_type_moves(
        &self,
        piece: PromotablePieceType,
        targets: BoardMask,
    ) -> impl Iterator<Item = ChessMove> + '_ {
        let piece_mask = self
            .piece_mask(NonKingPieceType::try_from(PieceType::from(piece)).unwrap())
            & self.player_to_move_mask();

        piece_mask.into_iter().flat_map(move |from| {
            let attacks_mask = self.get_attacks_for_promotable_piece_type(from, piece) & targets;
            let captures_mask = attacks_mask & self.opposite_player_mask();
            let capture_moves = captures_mask
                .into_iter()
                .map(move |to| self.create_capture(from, to, PieceType::from(piece)))
                .map(ChessMove::Capture);

            let quiet_mask = attacks_mask & self.empty_mask();
            let quiet_moves = quiet_mask
                .into_iter()
                .map(move |to| self.create_quiet(from, to, PieceType::from(piece)))
                .map(ChessMove::Quiet);

            capture_moves.chain(quiet_moves)
        })
    }

    fn get_attacks_for_promotable_piece_type(
        &self,
        from: Square,
        piece: PromotablePieceType,
    ) -> BoardMask {
        match piece {
            PromotablePieceType::Bishop => {
                BoardMask::sliding_attacks_for(SlidingPieceType::Bishop, from, self.occupied_mask())
            }
            PromotablePieceType::Rook => {
                BoardMask::sliding_attacks_for(SlidingPieceType::Rook, from, self.occupied_mask())
            }
            PromotablePieceType::Queen => {
                BoardMask::sliding_attacks_for(SlidingPieceType::Queen, from, self.occupied_mask())
            }
            PromotablePieceType::Knight => {
                BoardMask::pseudo_attacks_for(NonPawnPieceType::Knight, from)
            }
        }
    }
}
