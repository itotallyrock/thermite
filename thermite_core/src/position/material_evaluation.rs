use crate::evaluation::PawnEvaluation;
use crate::pieces::{NonKingPieceType, OwnedPiece};
use crate::player_color::PlayerColor;
use enum_map::EnumMap;
use std::sync::LazyLock;

/// A [board](position::LegalPosition)'s material [`PawnEvaluation`]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct MaterialEvaluation(PawnEvaluation);

/// Piece approximate valuations
static PIECE_VALUES: LazyLock<EnumMap<NonKingPieceType, PawnEvaluation>> = LazyLock::new(|| {
    EnumMap::from_array([
        PawnEvaluation(1.26),  // Pawn
        PawnEvaluation(7.81),  // Knight
        PawnEvaluation(8.25),  // Bishop
        PawnEvaluation(12.76), // Rook
        PawnEvaluation(25.38), // Queen
    ])
});

/// The side relative weight for distinguishing white vs black material
const PLAYER_WEIGHT: EnumMap<PlayerColor, PawnEvaluation> =
    EnumMap::from_array([PawnEvaluation(1.0), PawnEvaluation(-1.0)]);

impl MaterialEvaluation {
    /// Get a neutral or empty evaluation
    pub const fn new() -> Self {
        Self(PawnEvaluation::new(0.0))
    }

    /// Add a [piece](NonKingPieceType) from the material evaluation for a given [player](PlayerColor)
    pub fn add_piece(&mut self, owned_piece: OwnedPiece<NonKingPieceType>) {
        let OwnedPiece { player, piece } = owned_piece;
        self.0 += PLAYER_WEIGHT[player] * *PIECE_VALUES[piece];
    }

    /// Remove a [piece](NonKingPieceType) from the material evaluation for a given [player](PlayerColor)
    pub fn remove_piece(&mut self, owned_piece: OwnedPiece<NonKingPieceType>) {
        let OwnedPiece { player, piece } = owned_piece;
        self.0 -= PLAYER_WEIGHT[player] * *PIECE_VALUES[piece];
    }

    /// Get the [evaluation](PawnEvaluation) from a given [player](PlayerColor)'s perspective
    pub fn for_player(self, player: PlayerColor) -> PawnEvaluation {
        PawnEvaluation::new(PLAYER_WEIGHT[player].0 * self.0 .0)
    }
}

impl Default for MaterialEvaluation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use crate::pieces::{NonKingPieceType, Piece};
    use crate::player_color::PlayerColor;
    use crate::position::material_evaluation::MaterialEvaluation;
    use enum_iterator::all;

    #[test]
    fn add_remove_piece_is_symmetrical() {
        let original_eval = MaterialEvaluation::new();
        let mut eval = original_eval;
        for piece in all::<PlayerColor>()
            .flat_map(|player| all::<NonKingPieceType>().map(move |piece| piece.owned_by(player)))
        {
            for _ in 1..fastrand::usize(..32) {
                eval.add_piece(piece);
                eval.remove_piece(piece);
            }
        }

        assert_eq!(eval, original_eval);
    }
}
