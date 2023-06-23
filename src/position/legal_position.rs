use crate::bitboard::BoardMask;
use crate::castles::CastleRights;
use crate::chess_move::castle::Castle;
use crate::half_move_clock::HalfMoveClock;
use crate::pieces::{NonKingPieceType, OwnedPiece, Piece, PieceType, PlacedPiece};
use crate::player_color::PlayerColor;
use crate::position::hash_history::HashHistory;
use crate::position::material_evaluation::MaterialEvaluation;
use crate::position::position_builder::PositionBuilder;
use crate::square::{EnPassantSquare, Square};
use crate::zobrist::ZobristHash;
use derive_more::{AsMut, AsRef};
use enum_iterator::all;
use enum_map::EnumMap;

/// Invalid standard chess position (violates rules)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum IllegalPosition {
    /// Missing a king on the board for a given [player](PlayerColor)
    MissingKing(PlayerColor),
}

/// The hard to compute or irrecoverable/irreversible state
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    // Irrecoverable state
    pub(super) hash: ZobristHash,
    pub(super) halfmove_clock: HalfMoveClock,
    pub(super) en_passant_square: Option<EnPassantSquare>,
    pub(super) castles: CastleRights,
    // Move generation state
    pub(super) checkers: BoardMask,
    pub(super) pinners_for: EnumMap<PlayerColor, BoardMask>,
    pub(super) blockers_for: EnumMap<PlayerColor, BoardMask>,
    pub(super) check_squares: EnumMap<NonKingPieceType, BoardMask>,
}

/// A position known to be valid and legal in standard chess.
/// Keeps track of [`state`](State) to maintain legality as the board is mutated.
#[derive(Clone, PartialEq, Debug, AsRef, AsMut)]
pub struct LegalPosition {
    pub(super) material_eval: MaterialEvaluation,
    pub(super) player_to_move: PlayerColor,
    pub(super) pieces_masks: EnumMap<NonKingPieceType, BoardMask>,
    pub(super) side_masks: EnumMap<PlayerColor, BoardMask>,
    pub(super) king_squares: EnumMap<PlayerColor, Square>,
    pub(super) state: State,
    pub(super) hash_history: HashHistory,
}

impl TryFrom<PositionBuilder> for LegalPosition {
    type Error = IllegalPosition;

    fn try_from(position: PositionBuilder) -> Result<Self, Self::Error> {
        let PositionBuilder {
            halfmove_clock,
            squares,
            starting_player: player_to_move,
            castle_rights: castles,
            en_passant_square,
            ..
        } = position;
        // Construct most of the position fields by iterating over all of the squares with pieces
        let (pieces_masks, side_masks, king_squares, material_eval, mut hash) = squares
            .iter()
            .filter_map(|(square, piece)| piece.map(|owned_piece| owned_piece.placed_on(square)))
            .fold(
                (
                    EnumMap::<NonKingPieceType, BoardMask>::default(),
                    EnumMap::<PlayerColor, BoardMask>::default(),
                    EnumMap::<PlayerColor, Option<Square>>::default(),
                    MaterialEvaluation::new(),
                    ZobristHash::default(),
                ),
                |(
                    mut pieces_masks,
                    mut side_masks,
                    mut king_squares,
                    mut material_eval,
                    mut hash,
                ),
                 placed_piece| {
                    let PlacedPiece {
                        square,
                        owned_piece: OwnedPiece { piece, player },
                    } = placed_piece;
                    let square_mask = BoardMask::new(1) << square as u8;

                    // Update hash using zobrist key lookup
                    hash.toggle_piece_square(placed_piece);
                    // Add the piece to the side mask
                    side_masks[player] |= square_mask;

                    // If the piece is a king add it to the king squares, otherwise add it to the piece masks
                    NonKingPieceType::try_from(piece).map_or_else(
                        |_| {
                            king_squares[player] = Some(square);
                        },
                        |non_king_piece| {
                            pieces_masks[non_king_piece] |= square_mask;
                            material_eval.add_piece(non_king_piece.owned_by(player));
                        },
                    );

                    (pieces_masks, side_masks, king_squares, material_eval, hash)
                },
            );

        // Update hash from non placed piece positional state
        if player_to_move == PlayerColor::Black {
            hash.switch_sides();
        }

        if let Some(en_passant_square) = en_passant_square {
            hash.toggle_en_passant_square(en_passant_square);
        }

        for castle in Castle::all() {
            if castles.can_castle(castle) {
                hash.toggle_castle_ability(castle);
            }
        }

        // Construct state
        let checkers = BoardMask::default();
        let pinners_for = EnumMap::default();
        let blockers_for = EnumMap::default();
        let check_squares = EnumMap::default();
        let en_passant_square = en_passant_square.map(Into::into);
        let state = State {
            hash,
            halfmove_clock,
            en_passant_square,
            castles,
            checkers,
            pinners_for,
            blockers_for,
            check_squares,
        };

        // TODO: Get this from builder (when we have starting moves implemented)
        let hash_history = HashHistory::new();
        // Make sure we have two kings
        let king_squares = king_squares.into_iter().try_fold(
            EnumMap::<PlayerColor, Square>::from_array([Square::E1, Square::E8]),
            |mut king_squares, (player, square)| {
                king_squares[player] = square.ok_or(IllegalPosition::MissingKing(player))?;

                Ok(king_squares)
            },
        )?;

        // Create the position that might still have some invalid states
        let mut pseudo_legal_position = Self {
            material_eval,
            player_to_move,
            pieces_masks,
            side_masks,
            king_squares,
            state,
            hash_history,
        };

        pseudo_legal_position.update_masks();
        // TODO: Check legality (ie. back rank pawns)

        Ok(pseudo_legal_position)
    }
}

impl LegalPosition {
    /// Get the [`PieceType`] on a given [`Square`] if any
    #[must_use]
    pub fn piece_type_on(&self, square: Square) -> Option<PieceType> {
        // Check if either side's king square takes this space
        if all::<PlayerColor>().any(|side| self.king_squares[side] == square) {
            return Some(PieceType::King);
        }

        // Loop through all non-king pieces and see if there's a piece on the given square
        all::<NonKingPieceType>()
            .find(|&piece| self.pieces_masks[piece] & square.to_mask() != BoardMask::EMPTY)
            .map(PieceType::from)
    }

    /// Get the [`PlayerColor`] on a given [`Square`] if any
    #[must_use]
    pub fn player_color_on(&self, square: Square) -> Option<PlayerColor> {
        // Loop through all side masks and see if there's a piece on the given square
        all::<PlayerColor>().find(|&side| {
            self.king_squares[side] == square
                || self.side_masks[side] & square.to_mask() != BoardMask::EMPTY
        })
    }

    /// Get the [`OwnedPiece`] on a given [`Square`] if any
    #[must_use]
    pub fn owned_piece_on(&self, square: Square) -> Option<OwnedPiece> {
        self.piece_type_on(square)
            .zip(self.player_color_on(square))
            .map(|(piece, player)| piece.owned_by(player))
    }

    /// Whether or not the current player is in check
    #[must_use]
    pub const fn in_check(&self) -> bool {
        !(self.state.checkers).is_empty()
    }

    /// Get a mask for given piece (both players).
    pub fn piece_mask(&self, piece: NonKingPieceType) -> BoardMask {
        self.pieces_masks[piece]
    }

    /// Get the player whose turn it is to move
    #[must_use]
    pub const fn player_to_move(&self) -> PlayerColor {
        self.player_to_move
    }

    /// Get a [`BoardMask`] of the pieces for the [`PlayerColor`] moving
    pub fn player_to_move_mask(&self) -> BoardMask {
        self.side_masks[self.player_to_move]
    }

    /// Get a [`BoardMask`] of the pieces for the [`PlayerColor`] **not** moving
    pub fn opposite_player_mask(&self) -> BoardMask {
        self.side_masks[self.player_to_move.switch()]
    }

    /// Get a [`BoardMask`] of the attack-able squares (empty or opposite side) of the [`PlayerColor`] moving
    pub fn attackable_mask(&self) -> BoardMask {
        !self.player_to_move_mask()
    }

    /// Get a [`BoardMask`] of all of the pieces on the board
    ///
    /// ```
    ///
    /// use thermite::bitboard::BoardMask;
    /// use thermite::fen;
    ///
    /// assert_eq!(fen!("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4").occupied_mask(), BoardMask::new(0x42e1400000e0a77d));
    /// assert_eq!(fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").occupied_mask(), BoardMask::new(0xffff00000000ffff));
    /// assert_eq!(fen!("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/1r5P/8 b - - 0 1").occupied_mask(), BoardMask::new(0xc4401820718200));
    /// ```
    pub fn occupied_mask(&self) -> BoardMask {
        self.side_masks[PlayerColor::White] | self.side_masks[PlayerColor::Black]
    }

    /// Get a [`BoardMask`] of all the empty squares on the board
    ///
    /// ```
    /// use thermite::bitboard::BoardMask;
    /// use thermite::fen;
    /// assert_eq!(fen!("1r4k1/p4pbp/6p1/8/8/5QPb/PPP2P1P/R1BNrBK1 b - - 2 4").empty_mask(), BoardMask::new(0xbd1ebfffff1f5882));
    /// assert_eq!(fen!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").empty_mask(), BoardMask::new(0xffffffff0000));
    /// assert_eq!(fen!("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/1r5P/8 b - - 0 1").empty_mask(), BoardMask::new(0xff3bbfe7df8e7dff));
    /// ```
    pub fn empty_mask(&self) -> BoardMask {
        !self.occupied_mask()
    }

    /// Compute a [mask](BoardMask) of attackers for all [sides](PlayerColor) that can target a given [`Square`], given a set of [blockers](BoardMask)
    ///
    /// ```
    /// use thermite::bitboard::BoardMask;
    /// use thermite::fen;
    /// use thermite::square::Square;
    ///
    /// let position = fen!("8/2q3kp/6p1/3Bp3/5n2/Q3BPK1/6rP/8 w - - 1 2");
    /// assert_eq!(position.attackers_to(Square::G3, position.occupied_mask()), BoardMask::new(0xc000));
    /// ```
    pub fn attackers_to(&self, target: Square, occupied_mask: BoardMask) -> BoardMask {
        let square_mask: BoardMask = target.to_mask();

        let pawns = self.piece_mask(NonKingPieceType::Pawn);
        let white_pawns = pawns & self.side_masks[PlayerColor::White];
        let black_pawns = pawns & self.side_masks[PlayerColor::Black];
        let knights = self.piece_mask(NonKingPieceType::Knight);
        let queens = self.piece_mask(NonKingPieceType::Queen);
        let rooks_and_queens = self.piece_mask(NonKingPieceType::Rook) | queens;
        let bishops_and_queens = self.piece_mask(NonKingPieceType::Bishop) | queens;
        let kings = self
            .king_squares
            .values()
            .fold(BoardMask::EMPTY, |mask, king_square| {
                mask | king_square.to_mask()
            });

        let white_pawn_attacks = square_mask.pawn_attacks(PlayerColor::Black) & white_pawns;
        let black_pawn_attacks = square_mask.pawn_attacks(PlayerColor::White) & black_pawns;
        let knight_attacks = square_mask.knight_attacks() & knights;
        let rook_attacks = square_mask.cardinal_sliding_attacks(occupied_mask) & rooks_and_queens;
        let bishop_attacks =
            square_mask.ordinal_sliding_attacks(occupied_mask) & bishops_and_queens;
        let king_attacks = square_mask.king_attacks() & kings;

        white_pawn_attacks
            | black_pawn_attacks
            | knight_attacks
            | rook_attacks
            | bishop_attacks
            | king_attacks
    }
}
