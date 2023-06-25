use crate::chess_move::quiet::Quiet;
use crate::pieces::{NonKingPieceType, OwnedPiece, Piece, PieceType, PlacedPiece};
use crate::position::LegalPosition;

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

#[cfg(test)]
mod test {
    use crate::fen;
    use crate::pieces::PlacedPiece;
    use crate::pieces::{
        NonKingPieceType, Piece,
        PieceType::{Knight, Queen},
    };
    use crate::player_color::PlayerColor::{Black, White};
    use crate::square::Square::{A6, E4};
    use test_case::test_case;

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
}
