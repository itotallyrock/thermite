use crate::direction::Direction;
use crate::pieces::PromotablePieceType;
use crate::player_color::PlayerColor;
use crate::square::{
    EastShiftableFile, File, PromotableSquare, PromotionSquare, Rank, Square, WestShiftableFile,
};
use enum_map::EnumMap;

/// A valid pawn promotion (optionally a capture)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[must_use]
pub struct Promotion {
    /// The starting square
    from: PromotableSquare,
    /// The ending square
    to: PromotionSquare,
    /// The player doing the promotion
    player: PlayerColor,
    /// The piece the pawn is promoting to
    pub piece: PromotablePieceType,
}

impl Promotion {
    const FROM_RANK: EnumMap<PlayerColor, Rank> =
        EnumMap::from_array([Rank::Seventh, Rank::Second]);
    const TO_RANK: EnumMap<PlayerColor, Rank> = EnumMap::from_array([Rank::Eighth, Rank::First]);

    /// Create a new push promotion
    #[allow(clippy::missing_panics_doc)]
    pub fn new(piece: PromotablePieceType, file: File, player: PlayerColor) -> Self {
        let from = Square::new(file, Self::FROM_RANK[player])
            .try_into()
            .unwrap();
        let to = Square::new(file, Self::TO_RANK[player]).try_into().unwrap();

        Self {
            from,
            to,
            player,
            piece,
        }
    }

    /// Create a new east capture promotion
    #[allow(clippy::missing_panics_doc)]
    pub fn new_east_capture(
        piece: PromotablePieceType,
        starting_file: EastShiftableFile,
        player: PlayerColor,
    ) -> Self {
        let from = Square::new(File::from(starting_file), Self::FROM_RANK[player])
            .try_into()
            .unwrap();
        let to = Square::new(File::from(starting_file), Self::TO_RANK[player])
            .shift(Direction::East)
            .unwrap()
            .try_into()
            .unwrap();

        Self {
            from,
            to,
            player,
            piece,
        }
    }

    /// Create a new west capture promotion
    #[allow(clippy::missing_panics_doc)]
    pub fn new_west_capture(
        piece: PromotablePieceType,
        starting_file: WestShiftableFile,
        player: PlayerColor,
    ) -> Self {
        let from = Square::new(File::from(starting_file), Self::FROM_RANK[player])
            .try_into()
            .unwrap();
        let to = Square::new(File::from(starting_file), Self::TO_RANK[player])
            .shift(Direction::West)
            .unwrap()
            .try_into()
            .unwrap();

        Self {
            from,
            to,
            player,
            piece,
        }
    }

    /// Get the _ [square](PromotableSquare)
    #[must_use]
    pub const fn from(&self) -> PromotableSquare {
        self.from
    }

    /// Get the target [square](PromotionSquare) the promotion ends on
    #[must_use]
    pub const fn to(&self) -> PromotionSquare {
        self.to
    }

    /// Get the [player](PlayerColor) making the promotion
    #[must_use]
    pub const fn player(&self) -> PlayerColor {
        self.player
    }
}
