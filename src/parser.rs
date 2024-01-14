use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::tiles::*;
use crate::{Hand, HandGroup, HandTile, InvalidTileError, Suite, Tile, TilePlacement, TileValue};

const SUITE_MANZU: char = 'm';
const SUITE_PINZU: char = 'p';
const SUITE_SOUZU: char = 's';
const SUITE_HONOR: char = 'z';

const SPECIAL_TON: char = 'E';
const SPECIAL_NAN: char = 'S';
const SPECIAL_SHAA: char = 'W';
const SPECIAL_PEI: char = 'N';
const SPECIAL_HAKU: char = 'w';
const SPECIAL_HATSU: char = 'g';
const SPECIAL_CHUN: char = 'r';
const SPECIAL_ANY: char = '?';

const POSITION_MODIFIER_ASTERISK: char = '*';
const POSITION_MODIFIER_APOSTROPHE: char = '\'';
const GROUP_SEPARATOR: char = '_';

#[derive(Debug)]
/// A parser that converts string representation of a hand to Hand objects
pub struct HandParser {
    groups: Vec<HandGroup>,
    // data of the newly encountered tiles that we may not know the suite of yet
    new_tiles: Vec<(Option<Suite>, TileValue, TilePlacement)>,
}

impl HandParser {
    #[inline]
    /// Parses given hand representation and returns [Hand] instance, or error,
    /// if the hand string is invalid.
    ///
    /// # Format
    /// A hand can be made of following parts:
    /// * `1`, `2`, `3`, ..., `9` - tile values
    /// * `m`, `p`, `s`, `z` (manzu, pinzu, souzu, honor) - tile suites, used as
    ///   tile value prefixes
    /// * `E`, `S`, `W`, `N` - winds
    /// * `w`, `g`, `r` - dragons
    /// * `*` or `'` - tile value prefix that means that a tile is rotated.
    ///   Repeat twice to rotate and shift
    /// * `_` - tile group separator
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::HandTile;
    /// use riichi_hand::tiles::*;
    /// use riichi_hand::parser::HandParser;
    /// use riichi_hand::TilePlacement::{Normal, Rotated, RotatedAndShifted};
    ///
    /// assert_eq!(
    ///     HandParser::parse("123p1m").unwrap().groups(),
    ///     &vec![
    ///         vec![
    ///             HandTile::new(II_PIN, Normal),
    ///             HandTile::new(RYAN_PIN, Normal),
    ///             HandTile::new(SAN_PIN, Normal),
    ///             HandTile::new(II_MAN, Normal),
    ///         ]
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     HandParser::parse("11*1**1m").unwrap().groups(),
    ///     &vec![
    ///         vec![
    ///             HandTile::new(II_MAN, Normal),
    ///             HandTile::new(II_MAN, Rotated),
    ///             HandTile::new(II_MAN, RotatedAndShifted),
    ///             HandTile::new(II_MAN, Normal),
    ///         ]
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     HandParser::parse("E*EE_SS*S").unwrap().groups(),
    ///     &vec![
    ///         vec![
    ///             HandTile::new(TON, Rotated),
    ///             HandTile::new(TON, Normal),
    ///             HandTile::new(TON, Normal),
    ///         ],
    ///         vec![
    ///             HandTile::new(NAN, Normal),
    ///             HandTile::new(NAN, Rotated),
    ///             HandTile::new(NAN, Normal),
    ///         ]
    ///     ]
    /// );
    /// ```
    pub fn parse(hand: &str) -> Result<Hand, HandParseError> {
        Self::new().parse_internal(hand)
    }

    #[inline]
    fn new() -> Self {
        Self {
            groups: vec![vec![]],
            new_tiles: Vec::new(),
        }
    }

    fn parse_internal(mut self, hand: &str) -> Result<Hand, HandParseError> {
        for (pos, char) in hand.chars().enumerate() {
            let result = match char {
                '0'..='9' => self.handle_value(char),
                SUITE_MANZU | SUITE_PINZU | SUITE_SOUZU | SUITE_HONOR => self.handle_suite(char),
                SPECIAL_TON | SPECIAL_NAN | SPECIAL_SHAA | SPECIAL_PEI | SPECIAL_HAKU
                | SPECIAL_HATSU | SPECIAL_CHUN | SPECIAL_ANY => self.handle_special_symbol(char),
                POSITION_MODIFIER_ASTERISK | POSITION_MODIFIER_APOSTROPHE => {
                    self.handle_position_modifier()
                }
                GROUP_SEPARATOR => self.handle_group_separator(),
                _ => Err(HandParseErrorType::InvalidCharacter),
            };

            if let Err(err) = result {
                return Err(HandParseError::new(pos, err));
            }
        }

        if let Err(err) = self.add_remaining_tiles() {
            return Err(HandParseError::new(hand.len(), err));
        }

        Ok(Hand::new(self.groups))
    }

    fn handle_value(&mut self, number: char) -> HandParseResult {
        let value = TileValue(number as u8 - b'0');
        let placement = TilePlacement::Normal;
        self.new_tiles.push((None, value, placement));

        Ok(())
    }

    fn handle_suite(&mut self, suite_char: char) -> HandParseResult {
        let suite = match suite_char {
            SUITE_MANZU => Suite::Manzu,
            SUITE_PINZU => Suite::Pinzu,
            SUITE_SOUZU => Suite::Souzu,
            SUITE_HONOR => Suite::Honor,
            _ => unreachable!(),
        };

        for (tile_suite, _, _) in &mut self.new_tiles {
            *tile_suite = Some(tile_suite.unwrap_or(suite));
        }

        self.add_remaining_tiles()?;

        Ok(())
    }

    fn add_remaining_tiles(&mut self) -> HandParseResult {
        let mut new_tiles = Vec::new();
        std::mem::swap(&mut new_tiles, &mut self.new_tiles);

        for (actual_suite, value, placement) in new_tiles {
            let suite = actual_suite.ok_or(HandParseErrorType::UnfinishedSuite)?;
            self.add_tile(Tile::new(suite, value)?, placement)?;
        }

        Ok(())
    }

    fn handle_special_symbol(&mut self, honor: char) -> HandParseResult {
        match honor {
            SPECIAL_TON => self.add_temp_tile(TON, TilePlacement::Normal)?,
            SPECIAL_NAN => self.add_temp_tile(NAN, TilePlacement::Normal)?,
            SPECIAL_SHAA => self.add_temp_tile(SHAA, TilePlacement::Normal)?,
            SPECIAL_PEI => self.add_temp_tile(PEI, TilePlacement::Normal)?,
            SPECIAL_HAKU => self.add_temp_tile(HAKU, TilePlacement::Normal)?,
            SPECIAL_HATSU => self.add_temp_tile(HATSU, TilePlacement::Normal)?,
            SPECIAL_CHUN => self.add_temp_tile(CHUN, TilePlacement::Normal)?,
            SPECIAL_ANY => self.add_temp_tile(ANY, TilePlacement::Normal)?,
            _ => unreachable!(),
        }

        Ok(())
    }

    fn handle_position_modifier(&mut self) -> HandParseResult {
        let last_tile = self.new_tiles.last_mut();

        if let Some(tile) = last_tile {
            let placement = &mut tile.2;
            *placement = placement.next();
            Ok(())
        } else {
            Err(HandParseErrorType::PositionModifierWithNoTile)
        }
    }

    fn handle_group_separator(&mut self) -> HandParseResult {
        self.add_remaining_tiles()?;
        self.groups.push(Vec::new());

        Ok(())
    }

    fn add_temp_tile(&mut self, tile: Tile, placement: TilePlacement) -> HandParseResult {
        self.new_tiles
            .push((Some(tile.suite), tile.value, placement));
        Ok(())
    }

    fn add_tile(&mut self, tile: Tile, placement: TilePlacement) -> HandParseResult {
        self.groups
            .last_mut()
            .expect("List of groups is empty")
            .push(HandTile::new(tile, placement));

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Represents an error that occurred when parsing a hand.
pub struct HandParseError {
    /// Position (starting from character 0) where the error occurred.
    position: usize,
    /// Type of the error.
    error_type: HandParseErrorType,
}

impl Error for HandParseError {}

impl HandParseError {
    /// Creates a new [HandParseError] instance.
    pub fn new(position: usize, error_type: HandParseErrorType) -> Self {
        Self {
            position,
            error_type,
        }
    }
}

impl Display for HandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error when parsing hand at position {}: {}",
            self.position, self.error_type
        )
    }
}

type HandParseResult = Result<(), HandParseErrorType>;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
/// Type of an error occurred when parsing hand.
pub enum HandParseErrorType {
    /// Invalid character provided (e.g. `X`, or `@`).
    InvalidCharacter,
    /// Invalid tile value provided (e.g. `9z`).
    InvalidValue,
    /// Some tile values without any suite are left at the end of the string
    /// (e.g. `123m456`).
    UnfinishedSuite,
    /// Position modifier was used without any tile to modify (e.g. `**123m`).
    PositionModifierWithNoTile,
}

impl Error for HandParseErrorType {}

impl From<InvalidTileError> for HandParseErrorType {
    fn from(_: InvalidTileError) -> Self {
        HandParseErrorType::InvalidValue
    }
}

impl Display for HandParseErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HandParseErrorType::InvalidCharacter => "invalid character",
            HandParseErrorType::InvalidValue => "invalid tile value",
            HandParseErrorType::UnfinishedSuite => "tile suite not finished",
            HandParseErrorType::PositionModifierWithNoTile => {
                "position modifier does not have any tile to modify"
            }
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{HandParseError, HandParseErrorType, HandParser};
    use crate::tiles::*;
    use crate::{HandTile, Tile, TilePlacement};

    #[test]
    fn should_return_empty_hand() {
        let hand = HandParser::parse("");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(hand.tiles().collect::<Vec<Tile>>(), vec![]);
    }

    #[test]
    fn should_parse_single_suite() {
        let hand = HandParser::parse("1m");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(hand.tiles().collect::<Vec<Tile>>(), vec![II_MAN]);

        let hand = HandParser::parse("123456789m");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![II_MAN, RYAN_MAN, SAN_MAN, SUU_MAN, UU_MAN, ROU_MAN, CHII_MAN, PAA_MAN, KYUU_MAN]
        );

        let hand = HandParser::parse("123p");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![II_PIN, RYAN_PIN, SAN_PIN]
        );

        let hand = HandParser::parse("123s");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![II_SOU, RYAN_SOU, SAN_SOU]
        );

        let hand = HandParser::parse("1234567z");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![TON, NAN, SHAA, PEI, HAKU, HATSU, CHUN]
        );
    }

    #[test]
    fn should_parse_special_symbols() {
        let hand = HandParser::parse("ESWNwgr?");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![TON, NAN, SHAA, PEI, HAKU, HATSU, CHUN, ANY]
        );
    }

    #[test]
    fn should_parse_multiple_suites() {
        let hand = HandParser::parse("123m456p789s1122z");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.tiles().collect::<Vec<Tile>>(),
            vec![
                II_MAN, RYAN_MAN, SAN_MAN, SUU_PIN, UU_PIN, ROU_PIN, CHII_SOU, PAA_SOU, KYUU_SOU,
                TON, TON, NAN, NAN
            ]
        );
    }

    #[test]
    fn should_parse_position_modifiers() {
        let hand = HandParser::parse("11*1**1m");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.hand_tiles().collect::<Vec<HandTile>>(),
            vec![
                HandTile::new(II_MAN, TilePlacement::Normal),
                HandTile::new(II_MAN, TilePlacement::Rotated),
                HandTile::new(II_MAN, TilePlacement::RotatedAndShifted),
                HandTile::new(II_MAN, TilePlacement::Normal),
            ]
        );

        let hand = HandParser::parse("EE*E**E");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.hand_tiles().collect::<Vec<HandTile>>(),
            vec![
                HandTile::new(TON, TilePlacement::Normal),
                HandTile::new(TON, TilePlacement::Rotated),
                HandTile::new(TON, TilePlacement::RotatedAndShifted),
                HandTile::new(TON, TilePlacement::Normal),
            ]
        );

        let hand = HandParser::parse("11'1''1m");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.hand_tiles().collect::<Vec<HandTile>>(),
            vec![
                HandTile::new(II_MAN, TilePlacement::Normal),
                HandTile::new(II_MAN, TilePlacement::Rotated),
                HandTile::new(II_MAN, TilePlacement::RotatedAndShifted),
                HandTile::new(II_MAN, TilePlacement::Normal),
            ]
        );

        let hand = HandParser::parse("EE'E*'E");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(hand.groups().len(), 1);
        assert_eq!(
            hand.hand_tiles().collect::<Vec<HandTile>>(),
            vec![
                HandTile::new(TON, TilePlacement::Normal),
                HandTile::new(TON, TilePlacement::Rotated),
                HandTile::new(TON, TilePlacement::RotatedAndShifted),
                HandTile::new(TON, TilePlacement::Normal),
            ]
        );
    }

    #[test]
    fn should_parse_multiple_groups() {
        let hand = HandParser::parse("123m_4*56p__7s");
        assert!(hand.is_ok());
        let hand = hand.unwrap();
        assert_eq!(
            hand.groups(),
            &vec![
                vec![
                    HandTile {
                        tile: II_MAN,
                        placement: TilePlacement::Normal
                    },
                    HandTile {
                        tile: RYAN_MAN,
                        placement: TilePlacement::Normal
                    },
                    HandTile {
                        tile: SAN_MAN,
                        placement: TilePlacement::Normal
                    }
                ],
                vec![
                    HandTile {
                        tile: SUU_PIN,
                        placement: TilePlacement::Rotated
                    },
                    HandTile {
                        tile: UU_PIN,
                        placement: TilePlacement::Normal
                    },
                    HandTile {
                        tile: ROU_PIN,
                        placement: TilePlacement::Normal
                    }
                ],
                vec![],
                vec![HandTile {
                    tile: CHII_SOU,
                    placement: TilePlacement::Normal
                }]
            ]
        );
    }

    #[test]
    fn should_return_invalid_character_error() {
        let result = HandParser::parse("XD");
        assert_eq!(
            result,
            Err(HandParseError::new(0, HandParseErrorType::InvalidCharacter))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 0: invalid character"
        );
    }

    #[test]
    fn should_return_invalid_value_error() {
        let result = HandParser::parse("0z");
        assert_eq!(
            result,
            Err(HandParseError::new(1, HandParseErrorType::InvalidValue))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 1: invalid tile value"
        );

        let result = HandParser::parse("8z");
        assert_eq!(
            result,
            Err(HandParseError::new(1, HandParseErrorType::InvalidValue))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 1: invalid tile value"
        );
    }

    #[test]
    fn should_return_unfinished_suite_error() {
        let result = HandParser::parse("123");
        assert_eq!(
            result,
            Err(HandParseError::new(3, HandParseErrorType::UnfinishedSuite))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 3: tile suite not finished"
        );

        let result = HandParser::parse("123_456p");
        assert_eq!(
            result,
            Err(HandParseError::new(3, HandParseErrorType::UnfinishedSuite))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 3: tile suite not finished"
        );
    }

    #[test]
    fn should_return_position_modifier_error() {
        let result = HandParser::parse("**");
        assert_eq!(
            result,
            Err(HandParseError::new(
                0,
                HandParseErrorType::PositionModifierWithNoTile
            ))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 0: position modifier does not have any tile to modify"
        );

        let result = HandParser::parse("123p_*");
        assert_eq!(
            result,
            Err(HandParseError::new(
                5,
                HandParseErrorType::PositionModifierWithNoTile
            ))
        );
        assert_eq!(
            format!("{}", result.err().unwrap()),
            "error when parsing hand at position 5: position modifier does not have any tile to modify"
        );
    }
}
