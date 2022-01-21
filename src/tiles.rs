use crate::{Suite, Tile, TileValue};

// Manzu
/// Akadora man (red 5man) tile.
pub const AKADORA_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(0),
};
/// Ii man (1man) tile.
pub const II_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(1),
};
/// Ryan man (2man) tile.
pub const RYAN_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(2),
};
/// San man (3man) tile.
pub const SAN_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(3),
};
/// Suu man (4man) tile.
pub const SUU_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(4),
};
/// Uu man (5man) tile.
pub const UU_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(5),
};
/// Rou man (6man) tile.
pub const ROU_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(6),
};
/// Chii man (7man) tile.
pub const CHII_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(7),
};
/// Paa man (8man) tile.
pub const PAA_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(8),
};
/// Kyuu man (9man) tile.
pub const KYUU_MAN: Tile = Tile {
    suite: Suite::Manzu,
    value: TileValue(9),
};

// Pinzu
/// Akadora pin (red 5pin) tile.
pub const AKADORA_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(0),
};
/// Ii pin (1pin) tile.
pub const II_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(1),
};
/// Ryan pin (2pin) tile.
pub const RYAN_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(2),
};
/// San pin (3pin) tile.
pub const SAN_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(3),
};
/// Suu pin (4pin) tile.
pub const SUU_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(4),
};
/// Uu pin (5pin) tile.
pub const UU_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(5),
};
/// Rou pin (6pin) tile.
pub const ROU_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(6),
};
/// Chii pin (7pin) tile.
pub const CHII_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(7),
};
/// Paa pin (8pin) tile.
pub const PAA_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(8),
};
/// Kyuu pin (9pin) tile.
pub const KYUU_PIN: Tile = Tile {
    suite: Suite::Pinzu,
    value: TileValue(9),
};

// Souzu
/// Akadora sou (red 5sou) tile.
pub const AKADORA_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(0),
};
/// Ii sou (1sou) tile.
pub const II_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(1),
};
/// Ryan sou (2sou) tile.
pub const RYAN_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(2),
};
/// San sou (3sou) tile.
pub const SAN_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(3),
};
/// Suu sou (4sou) tile.
pub const SUU_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(4),
};
/// Uu sou (5sou) tile.
pub const UU_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(5),
};
/// Rou sou (6sou) tile.
pub const ROU_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(6),
};
/// Chii sou (7sou) tile.
pub const CHII_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(7),
};
/// Paa sou (8sou) tile.
pub const PAA_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(8),
};
/// Kyuu sou (9sou) tile.
pub const KYUU_SOU: Tile = Tile {
    suite: Suite::Souzu,
    value: TileValue(9),
};

// Honors
/// Ton (east) tile.
pub const TON: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(1),
};
/// Nan (south) tile.
pub const NAN: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(2),
};
/// Shaa (west) tile.
pub const SHAA: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(3),
};
/// Pei (north) tile.
pub const PEI: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(4),
};
/// Haku (white dragon) tile.
pub const HAKU: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(5),
};
/// Hatsu (green dragon) tile.
pub const HATSU: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(6),
};
/// Chun (red dragon) tile.
pub const CHUN: Tile = Tile {
    suite: Suite::Honor,
    value: TileValue(7),
};

// Any
/// Any tile.
pub const ANY: Tile = Tile {
    suite: Suite::Any,
    value: TileValue(0),
};

// All
/// A list containing all valid tiles.
pub const ALL_TILES: [Tile; 38] = [
    AKADORA_MAN,
    II_MAN,
    RYAN_MAN,
    SAN_MAN,
    SUU_MAN,
    UU_MAN,
    ROU_MAN,
    CHII_MAN,
    PAA_MAN,
    KYUU_MAN,
    AKADORA_PIN,
    II_PIN,
    RYAN_PIN,
    SAN_PIN,
    SUU_PIN,
    UU_PIN,
    ROU_PIN,
    CHII_PIN,
    PAA_PIN,
    KYUU_PIN,
    AKADORA_SOU,
    II_SOU,
    RYAN_SOU,
    SAN_SOU,
    SUU_SOU,
    UU_SOU,
    ROU_SOU,
    CHII_SOU,
    PAA_SOU,
    KYUU_SOU,
    TON,
    NAN,
    SHAA,
    PEI,
    HAKU,
    HATSU,
    CHUN,
    ANY,
];
