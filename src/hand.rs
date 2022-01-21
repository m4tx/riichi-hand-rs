use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Tile suite, or Any (if used with an unknown tile).
pub enum Suite {
    /// Manzu (characters).
    Manzu,
    /// Pinzu (circles).
    Pinzu,
    /// Souzu (bamboos).
    Souzu,
    /// Honors (winds and dragons).
    Honor,
    /// Any tile (no suite).
    Any,
}

impl Display for Suite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Suite::Manzu => "Manzu",
            Suite::Pinzu => "Pinzu",
            Suite::Souzu => "Souzu",
            Suite::Honor => "Honor",
            Suite::Any => "Any",
        };

        write!(f, "{}", name)
    }
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// A value of a tile:
/// * 1..9 for number tiles (manzu, pinzu, souzu), or 0 (which means red five),
/// * 1..7 for honor tiles (winds, then dragons),
/// * 0 for Any tile.
pub struct TileValue(pub u8);

impl Display for TileValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<TileValue> for usize {
    fn from(value: TileValue) -> Self {
        value.0 as usize
    }
}

const TILE_NUMERALS: [&str; 10] = [
    "Akadora", "Ii", "Ryan", "San", "Suu", "Uu", "Rou", "Chii", "Paa", "Kyuu",
];
const HONOR_NAMES: [&str; 7] = ["Ton", "Nan", "Shaa", "Pei", "Haku", "Hatsu", "Chun"];

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Tile representation (suite and value).
pub struct Tile {
    /// Suite of the tile.
    pub suite: Suite,
    /// Value of the tile.
    pub value: TileValue,
}

#[derive(Debug, Copy, Clone)]
/// Error that denotes that a user tried to create an invalid tile.
pub struct InvalidTileError {
    /// Requested tile suite.
    pub suite: Suite,
    /// Requested tile value.
    pub value: TileValue,
}

impl Error for InvalidTileError {}

impl InvalidTileError {
    #[inline]
    /// Returns a new [InvalidTileError] object.
    pub fn new(suite: Suite, value: TileValue) -> Self {
        Self { suite, value }
    }
}

impl Display for InvalidTileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid value: {} for suite: {}", self.value, self.suite)
    }
}

impl Tile {
    /// Creates a new tile with given suite and value.
    ///
    /// Returns an error if the provided suite-value pair is not valid.
    pub fn new(suite: Suite, value: TileValue) -> Result<Tile, InvalidTileError> {
        let range: RangeInclusive<usize> = match suite {
            Suite::Manzu | Suite::Pinzu | Suite::Souzu => 0..=9,
            Suite::Honor => 1..=7,
            Suite::Any => 0..=0,
        };
        if range.contains(&usize::from(value)) {
            Ok(Self { suite, value })
        } else {
            Err(InvalidTileError::new(suite, value))
        }
    }

    /// Returns human-readable name of the tile.
    pub fn name(&self) -> String {
        match self.suite {
            Suite::Manzu => format!("{} man", TILE_NUMERALS[usize::from(self.value)]),
            Suite::Pinzu => format!("{} pin", TILE_NUMERALS[usize::from(self.value)]),
            Suite::Souzu => format!("{} sou", TILE_NUMERALS[usize::from(self.value)]),
            Suite::Honor => HONOR_NAMES[usize::from(self.value) - 1].to_owned(),
            Suite::Any => "Any".to_owned(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name())
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Representation of a tile placement:
/// * `Normal`, for closed groups and tiles in open groups that are not rotated,
/// * `Rotated` for the rotated tiles in open groups,
/// * `RotatedAndShifted` for shouminkans ("added kans").
pub enum TilePlacement {
    /// A tile that is not rotated.
    Normal,
    /// A tile that is rotated in an open group.
    Rotated,
    /// A rotated and shifted tile that is a part of a shouminkan.
    RotatedAndShifted,
}

impl TilePlacement {
    #[inline]
    /// Returns next placement option in order, i.e. normal => rotated => rotated and shifted
    /// => normal.
    pub fn next(&self) -> TilePlacement {
        match self {
            TilePlacement::Normal => TilePlacement::Rotated,
            TilePlacement::Rotated => TilePlacement::RotatedAndShifted,
            TilePlacement::RotatedAndShifted => TilePlacement::Normal,
        }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Representation of a tile on a hand (tile and rotation).
pub struct HandTile {
    /// Specific tile.
    pub tile: Tile,
    /// Whether the tile is rotated or not.
    pub placement: TilePlacement,
}

impl HandTile {
    #[inline]
    /// Returns new hand tile using given tile object and placement.
    pub fn new(tile: Tile, placement: TilePlacement) -> Self {
        Self { tile, placement }
    }
}

/// A group consists of a list of hand tiles (tiles and their placements).
pub type HandGroup = Vec<HandTile>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
/// Hand object representation.
///
/// A hand consists of a number of tile groups. Note that an empty group is valid.
pub struct Hand {
    groups: Vec<HandGroup>,
}

impl Hand {
    #[inline]
    /// Returns a new hand representation object using given groups.
    pub fn new(groups: Vec<HandGroup>) -> Self {
        Self { groups }
    }

    #[inline]
    /// Return groups stored in this hand.
    pub fn groups(&self) -> &Vec<HandGroup> {
        &self.groups
    }

    #[inline]
    /// Returns an iterable over hand tile objects in this hand (ignoring groups).
    pub fn hand_tiles(&self) -> impl Iterator<Item = HandTile> + '_ {
        self.groups.iter().flatten().copied()
    }

    #[inline]
    /// Returns an iterable over tiles in this hand (ignoring groups and tile placements).
    pub fn tiles(&self) -> impl Iterator<Item = Tile> + '_ {
        self.groups.iter().flatten().map(|x| x.tile)
    }
}

#[cfg(test)]
mod tests {
    use crate::tiles::ALL_TILES;
    use crate::{Suite, Tile, TileValue};

    #[test]
    fn should_return_valid_suite_names() {
        let suites = [
            Suite::Manzu,
            Suite::Pinzu,
            Suite::Souzu,
            Suite::Honor,
            Suite::Any,
        ];
        let names = suites.map(|suite| format!("{}", suite));
        let expected = ["Manzu", "Pinzu", "Souzu", "Honor", "Any"].map(|x| x.to_owned());

        assert_eq!(names, expected);
    }

    #[test]
    fn should_create_valid_tiles() {
        assert!(Tile::new(Suite::Manzu, TileValue(1)).is_ok());
        assert!(Tile::new(Suite::Manzu, TileValue(0)).is_ok());
        assert!(Tile::new(Suite::Manzu, TileValue(2)).is_ok());
        assert!(Tile::new(Suite::Manzu, TileValue(9)).is_ok());
        assert!(Tile::new(Suite::Pinzu, TileValue(0)).is_ok());
        assert!(Tile::new(Suite::Pinzu, TileValue(9)).is_ok());
        assert!(Tile::new(Suite::Souzu, TileValue(0)).is_ok());
        assert!(Tile::new(Suite::Souzu, TileValue(9)).is_ok());
        assert!(Tile::new(Suite::Honor, TileValue(1)).is_ok());
        assert!(Tile::new(Suite::Honor, TileValue(2)).is_ok());
        assert!(Tile::new(Suite::Honor, TileValue(7)).is_ok());
        assert!(Tile::new(Suite::Any, TileValue(0)).is_ok());
    }

    #[test]
    fn should_return_error_on_invalid_tiles() {
        assert!(Tile::new(Suite::Manzu, TileValue(10)).is_err());
        assert!(Tile::new(Suite::Pinzu, TileValue(10)).is_err());
        assert!(Tile::new(Suite::Souzu, TileValue(10)).is_err());
        assert!(Tile::new(Suite::Honor, TileValue(0)).is_err());
        assert!(Tile::new(Suite::Honor, TileValue(8)).is_err());
        assert!(Tile::new(Suite::Any, TileValue(1)).is_err());
        assert!(Tile::new(Suite::Any, TileValue(5)).is_err());
    }

    #[test]
    fn should_return_valid_tile_names() {
        let names = ALL_TILES.map(|tile| format!("{}", tile));
        let expected = [
            "Akadora man",
            "Ii man",
            "Ryan man",
            "San man",
            "Suu man",
            "Uu man",
            "Rou man",
            "Chii man",
            "Paa man",
            "Kyuu man",
            "Akadora pin",
            "Ii pin",
            "Ryan pin",
            "San pin",
            "Suu pin",
            "Uu pin",
            "Rou pin",
            "Chii pin",
            "Paa pin",
            "Kyuu pin",
            "Akadora sou",
            "Ii sou",
            "Ryan sou",
            "San sou",
            "Suu sou",
            "Uu sou",
            "Rou sou",
            "Chii sou",
            "Paa sou",
            "Kyuu sou",
            "Ton",
            "Nan",
            "Shaa",
            "Pei",
            "Haku",
            "Hatsu",
            "Chun",
            "Any",
        ]
        .map(|x| x.to_owned());

        assert_eq!(names, expected);
    }
}
