use crate::bitboard::BoardMask;
use crate::chess_move::capture::Capture;
use crate::chess_move::double_pawn_push::DoublePawnPush;
use crate::chess_move::en_passant_capture::EnPassantCapture;
use crate::chess_move::promoting_capture::PromotingCapture;
use crate::chess_move::promotion::Promotion;
use crate::chess_move::quiet::Quiet;
use crate::chess_move::ChessMove;
use crate::direction::{Direction, PawnCaptureDirection, PawnPushDirection};
use crate::pieces::{NonKingPieceType, Piece, PieceType, PromotablePieceType};
use crate::player_color::PlayerColor;
use crate::position::LegalPosition;
use crate::square::{EastShiftableFile, File, Rank, Square, WestShiftableFile};
use enum_iterator::all;

impl LegalPosition {
    /// TODO
    pub(super) fn generate_pawn_moves(
        &self,
        targets: BoardMask,
    ) -> impl Iterator<Item = ChessMove> + '_ {
        self.generate_pawn_pushes(targets)
            .chain(self.get_all_pawn_captures(targets))
    }

    fn generate_pawn_pushes(&self, targets: BoardMask) -> impl Iterator<Item = ChessMove> + '_ {
        let targets = self.empty_mask() & targets;
        let push_direction: Direction = PawnPushDirection::for_player(self.player_to_move()).into();
        let opposite_push_direction = push_direction.opposite();
        let pawns = self.piece_mask(NonKingPieceType::Pawn) & self.player_to_move_mask();
        let pushed_pawns = pawns.shift(push_direction) & targets;
        let (double_pawn_push_destination_rank, promotion_destination_rank) =
            match self.player_to_move {
                PlayerColor::White => (Rank::Fourth, Rank::Eighth),
                PlayerColor::Black => (Rank::Fifth, Rank::First),
            };
        let double_pushed_pawns = pushed_pawns.shift(push_direction)
            & BoardMask::RANKS[double_pawn_push_destination_rank]
            & targets;
        let promotion_destination_rank_mask = BoardMask::RANKS[promotion_destination_rank];
        let promoting_pushes = pushed_pawns & promotion_destination_rank_mask;

        let promoting_push_moves = promoting_pushes
            .into_iter()
            .map(Square::file)
            .flat_map(move |to_file| {
                all::<PromotablePieceType>().map(move |promotion_piece| {
                    Promotion::new(promotion_piece, to_file, self.player_to_move)
                })
            })
            .map(ChessMove::Promotion);

        let non_promoting_pushes = pushed_pawns & !promotion_destination_rank_mask;
        let push_moves = non_promoting_pushes
            .into_iter()
            .map(move |to| {
                let from = to.shift(opposite_push_direction).unwrap();
                self.create_quiet(from, to, PieceType::Pawn)
            })
            .map(ChessMove::Quiet);

        let double_push_moves = double_pushed_pawns
            .into_iter()
            .map(|to| {
                let file = to.file();
                let player = self.player_to_move();
                DoublePawnPush::new(player, file)
            })
            .map(ChessMove::DoublePawnPush);

        promoting_push_moves
            .chain(push_moves)
            .chain(double_push_moves)
    }

    fn get_all_pawn_captures(&self, targets: BoardMask) -> impl Iterator<Item = ChessMove> + '_ {
        all::<PawnCaptureDirection>().flat_map(move |direction| {
            self.generate_pawn_captures_for_direction(targets, direction)
        })
    }

    fn generate_pawn_captures_for_direction(
        &self,
        targets: BoardMask,
        direction: PawnCaptureDirection,
    ) -> impl Iterator<Item = ChessMove> + '_ {
        let targets = self.opposite_player_mask() & targets;
        let pawns = self.piece_mask(NonKingPieceType::Pawn) & self.player_to_move_mask();
        let attacks = match direction {
            PawnCaptureDirection::East => pawns.pawn_east_attacks(self.player_to_move),
            PawnCaptureDirection::West => pawns.pawn_west_attacks(self.player_to_move),
        };

        let en_passant_captures = self
            .state
            .en_passant_square
            .into_iter()
            .filter(move |&en_passant_square| {
                !(Square::from(en_passant_square).to_mask() & attacks).is_empty()
            })
            .map(move |en_passant_square| {
                EnPassantCapture::new_en_passant_square(
                    en_passant_square,
                    direction,
                    self.player_to_move,
                )
                .expect(
                    "shifting the opposite direction of previous shift will always be on the board",
                )
            })
            .map(ChessMove::EnPassantCapture)
            .take(1);

        let opposite_pawn_capture_direction =
            direction.to_sided_direction(self.player_to_move).opposite();
        let opposite_pawn_push_direction =
            Direction::from(PawnPushDirection::for_player(self.player_to_move)).opposite();
        let non_promoting_mask = BoardMask::FULL.shift(opposite_pawn_push_direction);
        let pawn_captures_mask = attacks & targets;
        let pawn_promoting_captures_mask = pawn_captures_mask & !non_promoting_mask;
        let pawn_promoting_captures = pawn_promoting_captures_mask
            .into_iter()
            .map(move |to| {
                let from = to.shift(opposite_pawn_capture_direction).unwrap();
                from.file()
            })
            .flat_map(move |from_file| {
                all::<PromotablePieceType>().map(move |promotion_piece| {
                    self.create_promoting_capture_promotion(promotion_piece, from_file, direction)
                })
            })
            .map(|promotion| {
                let captured_piece = self.get_captured_piece(promotion.to().into());
                PromotingCapture::new(promotion, captured_piece)
            })
            .map(ChessMove::PromotingCapture);

        let pawn_captures_mask = pawn_captures_mask & non_promoting_mask;
        let pawn_captures = pawn_captures_mask
            .into_iter()
            .map(move |to| {
                let from = to.shift(opposite_pawn_capture_direction).expect(
                    "shifting the opposite direction of previous shift will always be on the board",
                );
                let captured_piece = NonKingPieceType::try_from(
                    self.piece_type_on(to)
                        .expect("known capture will have piece on to"),
                )
                .expect("attempting to capture king");
                Capture::new(
                    Quiet::new(from, to, PieceType::Pawn.owned_by(self.player_to_move))
                        .expect("shifted squares cannot equal"),
                    captured_piece,
                )
            })
            .map(ChessMove::Capture);

        pawn_promoting_captures
            .chain(en_passant_captures)
            .chain(pawn_captures)
    }

    fn create_promoting_capture_promotion(
        &self,
        piece: PromotablePieceType,
        file: File,
        direction: PawnCaptureDirection,
    ) -> Promotion {
        match direction {
            PawnCaptureDirection::East => {
                let starting_file = EastShiftableFile::try_from(file).expect("direction is east");
                Promotion::new_east_capture(piece, starting_file, self.player_to_move)
            }
            PawnCaptureDirection::West => {
                let starting_file = WestShiftableFile::try_from(file).expect("direction is west");
                Promotion::new_west_capture(piece, starting_file, self.player_to_move)
            }
        }
    }
}
