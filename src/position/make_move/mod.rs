use enum_iterator::all;
use enum_map::EnumMap;

use crate::bitboard::BoardMask;
use crate::chess_move::castle::Castle;
use crate::chess_move::quiet::Quiet;
use crate::pieces::{NonKingPieceType, OwnedPiece, Piece, PieceType, PlacedPiece};
use crate::player_color::PlayerColor;
use crate::position::hash_history::HashHistory;
use crate::position::legal_position::State;
use crate::position::material_evaluation::MaterialEvaluation;
use crate::position::{IllegalPosition, LegalPosition, PositionBuilder};
use crate::square::Square;
use crate::zobrist::ZobristHash;

mod make;
mod unmake;

/// Common make/unmake methods
impl LegalPosition {
    /// Simply change the player to move and its key from the hash
    fn switch_player_to_move(&mut self) {
        self.player_to_move = self.player_to_move.switch();
        self.state.hash.switch_sides();
    }

    /// Place [a piece](PlacedPiece) on the board
    ///
    /// # Panics
    /// Panics in debug mode when attempting to add a piece to a non-empty square
    fn add_piece(&mut self, placed_piece: PlacedPiece) {
        let PlacedPiece {
            owned_piece:
                OwnedPiece {
                    player,
                    piece: piece_type,
                },
            square: to,
        } = placed_piece;

        // Make sure we aren't adding a piece where we shouldn't (avoids really nasty bugs)
        debug_assert_eq!(
            self.piece_type_on(to),
            None,
            "attempting to `add_piece` to a non-empty square"
        );
        debug_assert_eq!(
            self.player_color_on(to),
            None,
            "attempting to `add_piece` to a non-empty square"
        );

        // Update the hash
        self.state.hash.toggle_piece_square(placed_piece);
        // Update the side mask
        let to_mask = to.to_mask();
        self.side_masks[player] |= to_mask;
        // Update the piece masks or king square
        if let Ok(piece_type) = NonKingPieceType::try_from(piece_type) {
            self.pieces_masks[piece_type] |= to_mask;
            // Update the material evaluation
            self.material_eval.add_piece(piece_type.owned_by(player));
        } else {
            self.king_squares[player] = to;
        }

        // TODO: Update piece square evaluation
    }

    /// Clear a piece from the board
    ///
    /// # Panics
    /// - Will panic in debug mode when removing a piece from an empty square
    /// - Will panic in debug mode when removing a king
    fn remove_piece(&mut self, placed_piece: PlacedPiece<NonKingPieceType>) {
        let PlacedPiece {
            square,
            owned_piece:
                OwnedPiece {
                    player,
                    piece: piece_type,
                },
        } = placed_piece;
        // Make sure we aren't removing a piece where we shouldn't (avoids really nasty bugs)
        debug_assert_eq!(
            self.piece_type_on(square),
            Some(PieceType::from(piece_type)),
            "attempting to `remove_piece` from a `Square` not occupied by the specified `PieceType`"
        );
        debug_assert_eq!(
            self.player_color_on(square),
            Some(player),
            "attempting to `remove_piece` from a `Square` not occupied by the specified `PlayerColor`"
        );

        // Update the hash
        self.state.hash.toggle_piece_square(
            PieceType::from(piece_type)
                .owned_by(player)
                .placed_on(square),
        );
        // Update the side mask
        let square_mask = square.to_mask();
        self.side_masks[player] ^= square_mask;
        // Update the piece mask
        self.pieces_masks[piece_type] ^= square_mask;

        // Update the material evaluation
        self.material_eval.remove_piece(placed_piece.owned_piece);

        // TODO: Update piece square evaluation
    }

    /// Move a piece on the board
    ///
    /// # Panics
    /// Will panic in debug mode if moving from a square not owned by the side or to an occupied square.
    fn move_piece(&mut self, quiet: Quiet) {
        let from = quiet.from();
        let to = quiet.to();
        let owned_piece = quiet.piece();
        let OwnedPiece {
            piece: piece_type,
            player,
        } = owned_piece;

        // Make sure we aren't removing a piece where we shouldn't (avoids really nasty bugs)
        debug_assert_eq!(
            self.piece_type_on(from),
            Some(piece_type),
            "attempting to `move_piece` from a `Square` not occupied by the specified `PieceType`"
        );
        debug_assert_eq!(self.player_color_on(from), Some(player), "attempting to `move_piece` from a `Square` not occupied by the specified `PlayerColor`");
        debug_assert_eq!(
            self.piece_type_on(to),
            None,
            "attempting to `move_piece` to a non-empty square"
        );
        debug_assert_eq!(
            self.player_color_on(to),
            None,
            "attempting to `move_piece` to a non-empty square"
        );

        // Update the hash
        self.state
            .hash
            .toggle_piece_square(owned_piece.placed_on(from));
        self.state
            .hash
            .toggle_piece_square(owned_piece.placed_on(to));
        // Update side mask
        let to_mask = to.to_mask();
        let from_mask = from.to_mask();
        let from_to_mask = from_mask | to_mask;
        self.side_masks[player] ^= from_to_mask;
        // Update piece mask
        if let Ok(piece_type) = NonKingPieceType::try_from(piece_type) {
            self.pieces_masks[piece_type] ^= from_to_mask;
        } else {
            self.king_squares[player] = to;
        }

        // TODO: Update piece square evaluation
    }
}

impl TryFrom<PositionBuilder> for LegalPosition {
    type Error = IllegalPosition;

    fn try_from(position: PositionBuilder) -> Result<Self, Self::Error> {
        let PositionBuilder {
            halfmove_clock,
            halfmove_count: _,
            squares,
            starting_player: player_to_move,
            castle_rights: castles,
            en_passant_square,
        } = position;
        let (king_squares, side_masks, mut hash) = all::<PlayerColor>().try_fold(
            (
                EnumMap::from_array([Square::E1, Square::E8]),
                EnumMap::default(),
                ZobristHash::default(),
            ),
            |(mut king_squares, mut side_masks, mut hash), player_color| {
                let player_king = PieceType::King.owned_by(player_color);
                let king_square = squares
                    .iter()
                    .find_map(|(s, &p)| p.filter(|&p| p == player_king).map(|_| s))
                    .ok_or(IllegalPosition::MissingKing(player_color))?;
                king_squares[player_color] = king_square;
                side_masks[player_color] = king_square.to_mask();
                hash.toggle_piece_square(player_king.placed_on(king_square));
                Ok((king_squares, side_masks, hash))
            },
        )?;

        // Update hashed fields that we include in the initial `pseudo_legal_position`
        if player_to_move != PlayerColor::White {
            hash.switch_sides();
        }
        if let Some(en_passant_square) = en_passant_square {
            hash.toggle_en_passant_square(en_passant_square);
        }
        Castle::all()
            .filter(|castle| castles.has_rights(castle.required_rights()))
            .for_each(|castle| hash.toggle_castle_ability(castle));

        let mut pseudo_legal_position = Self {
            material_eval: MaterialEvaluation::default(),
            player_to_move,
            pieces_masks: EnumMap::default(),
            side_masks,
            king_squares,
            state: State {
                hash,
                halfmove_clock,
                en_passant_square,
                castles,
                checkers: BoardMask::default(),
                pinners_for: EnumMap::default(),
                blockers_for: EnumMap::default(),
                check_squares: EnumMap::default(),
            },
            hash_history: HashHistory::default(),
        };

        // Add each piece
        squares
            .iter()
            .filter_map(|(s, p)| {
                p.filter(|p| p.piece != PieceType::King)
                    .map(|p| p.placed_on(s))
            })
            .for_each(|p| pseudo_legal_position.add_piece(p));

        pseudo_legal_position.update_masks();
        // TODO: Check legality (ie. back rank pawns)

        Ok(pseudo_legal_position)
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::chess_move::double_pawn_push::DoublePawnPush;
    use crate::chess_move::quiet::Quiet;
    use crate::fen;
    use crate::pieces::PlacedPiece;
    use crate::pieces::{NonKingPieceType, Piece, PieceType::*};
    use crate::player_color::PlayerColor::*;
    use crate::square::{File, Square::*};

    #[test_case("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4")]
    fn switch_sides_is_symmetrical(fen: &str) {
        let original_position = fen!(fen);
        let mut position = original_position.clone();
        position.switch_player_to_move();
        position.switch_player_to_move();
        assert_eq!(position, original_position);
    }

    #[test_case("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4", Knight.owned_by(White).placed_on(E4))]
    #[test_case("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/1r5P/8 b - - 4 8", Queen.owned_by(Black).placed_on(A6))]
    fn add_piece_remove_piece_is_symmetrical(fen: &str, piece: PlacedPiece) {
        let original_position = fen!(fen);
        let mut position = original_position.clone();
        position.add_piece(piece);
        assert_eq!(
            position.owned_piece_on(piece.square),
            Some(piece.owned_piece)
        );
        let placed_piece = NonKingPieceType::try_from(piece.owned_piece.piece)
            .unwrap()
            .owned_by(piece.owned_piece.player)
            .placed_on(piece.square);
        position.remove_piece(placed_piece);
        assert_eq!(position, original_position);
    }

    #[test_case("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/1r5P/8 b - - 4 8", Quiet::new(C7, C2, Queen.owned_by(Black)).unwrap())]
    #[test_case("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/1r5P/8 w - - 4 8", Quiet::new(A3, E7, Queen.owned_by(White)).unwrap())]
    #[test_case("2r5/p7/1kp2R2/4nQ1P/8/1P3P2/P1Rq2r1/1K6 b - - 2 4", DoublePawnPush::new(Black, File::A).into())]
    #[test_case("k7/ppr5/P3ppn1/2Pp4/Q1n1b1p1/2P3Pr/4PKBq/R1B1NR2 b - - 2 4", Quiet::new(G6, E5, Knight.owned_by(Black)).unwrap())]
    fn move_piece_is_symmetrical(fen: &str, quiet: Quiet) {
        let original_position = fen!(fen);
        let mut position = original_position.clone();
        position.move_piece(quiet);
        assert_eq!(position.owned_piece_on(quiet.to()), Some(quiet.piece()));
        position.move_piece(quiet.reverse());
        assert_eq!(position, original_position);
    }
}
