use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, RangeFrom, RangeInclusive};

use num_traits::{Pow, Signed};

/// Number of han (big) points.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Han(i32);

impl Han {
    /// Constructs new `Han` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Han;
    ///
    /// let han = Han::new(5);
    /// assert_eq!(han.get(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the integer value for a `Han` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Han;
    ///
    /// let han = Han::new(5);
    /// assert_eq!(han.get(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl<T: Into<i32>> From<T> for Han {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

impl Display for Han {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} han", self.0)
    }
}

/// Number of fu (small) points.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Fu(i32);

impl Fu {
    /// Constructs new `Fu` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Fu;
    ///
    /// let fu = Fu::new(30);
    /// assert_eq!(fu.get(), 30);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the integer value for a `Fu` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Fu;
    ///
    /// let fu = Fu::new(30);
    /// assert_eq!(fu.get(), 30);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl<T: Into<i32>> From<T> for Fu {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

impl Display for Fu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} fu", self.0)
    }
}

/// Number of honbas (counter sticks).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Honbas(i32);

impl Honbas {
    /// A constant meaning zero honbas. `Honbas::ZERO` is also the default
    /// value.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Honbas;
    ///
    /// assert_eq!(Honbas::ZERO.get(), 0);
    /// assert_eq!(Honbas::ZERO, Honbas::default());
    /// ```
    pub const ZERO: Honbas = Honbas::new(0);

    /// Constructs new `Honba` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Honbas;
    ///
    /// let honba = Honbas::new(2);
    /// assert_eq!(honba.get(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Gets the integer value for a `Fu` object.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Honbas;
    ///
    /// let honba = Honbas::new(2);
    /// assert_eq!(honba.get(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

impl<T: Into<i32>> From<T> for Honbas {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

impl Display for Honbas {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} honbas", self.0)
    }
}

impl Default for Honbas {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum PointsMode {
    Calculated { has_tsumo: bool, has_ron: bool },
    Limited,
}

impl PointsMode {
    #[inline]
    #[must_use]
    const fn has_tsumo(&self) -> bool {
        match self {
            PointsMode::Calculated { has_tsumo, .. } => *has_tsumo,
            PointsMode::Limited => true,
        }
    }

    #[inline]
    #[must_use]
    const fn has_ron(&self) -> bool {
        match self {
            PointsMode::Calculated { has_ron, .. } => *has_ron,
            PointsMode::Limited => true,
        }
    }
}

/// Number of (scoring) points.
///
/// This struct can be constructed using so-called base points. Base points are
/// calculated using the following formula: fu × 2^(2 + han). Base points are
/// then multiplied by 1, 2, 4, or 6, and rounded up to the next 100 to
/// get the number of points paid to the winner. Specifically:
/// * non-dealer tsumo: base points × 1 paid by other non-dealers, base points ×
///   2 paid by the dealer,
/// * non-dealer ron: base points × 4 paid by the discarding player,
/// * dealer tsumo: base points × 2 paid by everyone,
/// * dealer ron: base points × 6 paid by the discarding player.
///
/// Each value is rounded up to the next 100.
///
/// This variant uses [`i32`] as its base to store the number of points. This is
/// more than enough for any practical uses, but if you need to use different
/// base data type (including BigInts), you can use [`PointsCustom`].
pub type Points = PointsCustom<i32>;

/// Number of (scoring) points.
///
/// This type allows to specify a base type that stores the number of points.
/// This allows one to e.g. use BigInts and calculate the number of points for
/// absurdly high number of [`Han`].
///
/// Normally, [`Points`] type alias should be used instead of using this type
/// directly.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PointsCustom<T> {
    base_points: T,
    honbas: Honbas,
    mode: PointsMode,
}

impl<T> PointsCustom<T>
where
    T: Clone,
    T: Signed,
    T: From<i32>,
    T: PartialOrd<T>,
    T: Add<i32, Output = T>,
    T: Mul<i32, Output = T>,
    T: Div<i32, Output = T>,
    T: Pow<u32, Output = T>,
{
    /// Constructs an instance of `PointsCustom` by calculating the number of
    /// points for given [`Han`] and [`Fu`] values.
    ///
    /// There are different modes for calculating the points; see
    /// [`PointsCalculationMode`] documentation for more details.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::{Fu, Han, Points, PointsCalculationMode};
    ///
    /// let points_1 = Points::from_calculated(PointsCalculationMode::Default, Han::new(4), Fu::new(30)).unwrap();
    /// assert_eq!(points_1.ko_ron().unwrap(), 7700);
    ///
    /// let points_2 = Points::from_calculated(PointsCalculationMode::Loose, Han::new(1), Fu::new(20)).unwrap();
    /// assert_eq!(points_2.ko_ron().unwrap(), 700);
    ///
    /// let points_3 = Points::from_calculated(PointsCalculationMode::Unlimited, Han::new(15), Fu::new(50)).unwrap();
    /// assert_eq!(points_3.ko_ron().unwrap(), 26214400);
    /// ```
    pub fn from_calculated(
        calculation_mode: PointsCalculationMode,
        han: Han,
        fu: Fu,
        honbas: Honbas,
    ) -> Result<Self, PointCalculationError> {
        if calculation_mode == PointsCalculationMode::Default {
            if han < Han::new(1) {
                return Err(PointCalculationError::InvalidHan(han));
            }
            if !VALID_FU.contains(&fu) {
                return Err(PointCalculationError::InvalidFu(fu));
            }
            if honbas < Honbas::ZERO {
                return Err(PointCalculationError::InvalidHonbas(honbas));
            }
        }

        if calculation_mode != PointsCalculationMode::Unlimited {
            if MANGAN_HAN_RANGE.contains(&han) {
                return Ok(Self::mangan(honbas));
            } else if HANEMAN_HAN_RANGE.contains(&han) {
                return Ok(Self::haneman(honbas));
            } else if BAIMAN_HAN_RANGE.contains(&han) {
                return Ok(Self::baiman(honbas));
            } else if SANBAIMAN_HAN_RANGE.contains(&han) {
                return Ok(Self::sanbaiman(honbas));
            } else if KAZOE_YAKUMAN_HAN_RANGE.contains(&han) {
                return Ok(Self::yakuman(honbas));
            }
        }

        let power = han.0 + 2;
        const MIN_USABLE_HAN: i32 = -(i32::BITS as i32);
        let points_base = if power.is_positive() {
            T::from(2i32).pow(power as u32) * fu.0
        } else {
            // It's fine to operate on i64 here as using very high (as in absolute value)
            // negative han values will result in base points number of less than 1 anyway
            let power = power.max(MIN_USABLE_HAN).neg() as u32;
            let multiplier = 2i64.pow(power);
            let value = if fu.0.is_positive() {
                (fu.0 as i64 + multiplier - 1) / multiplier
            } else {
                fu.0 as i64 / multiplier
            };
            T::from(value as i32)
        };
        if calculation_mode != PointsCalculationMode::Unlimited && points_base >= T::from(7900 / 4)
        {
            Ok(Self::mangan(honbas))
        } else {
            let val_has_tsumo =
                calculation_mode != PointsCalculationMode::Default || has_tsumo(han, fu);
            let val_has_ron =
                calculation_mode != PointsCalculationMode::Default || has_ron(han, fu);

            let value = Self::new_calculated(points_base, val_has_tsumo, val_has_ron, honbas);
            Ok(value)
        }
    }
}

impl<T> PointsCustom<T>
where
    T: Clone,
    T: Signed,
    T: From<i32>,
    T: Add<i32, Output = T>,
    T: Mul<i32, Output = T>,
    T: Div<i32, Output = T>,
{
    /// Constructs a new instance of `PointsCustom`, marking it as limited
    /// (i.e. mangan or above) with given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::{Honbas, Points};
    ///
    /// let points = Points::new_limited(2000, Honbas::ZERO);
    /// assert_eq!(points.is_limited(), true);
    /// assert_eq!(points.ko_ron().unwrap(), 8000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new_limited(base_points: T, honbas: Honbas) -> Self {
        Self {
            base_points,
            mode: PointsMode::Limited,
            honbas,
        }
    }

    /// Constructs a new instance of `PointsCustom` with the base points value
    /// of 2000 and given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::{Honbas, Points};
    ///
    /// let points = Points::mangan(Honbas::ZERO);
    /// assert_eq!(points.ko_ron().unwrap(), 8000);
    /// assert_eq!(points.is_limited(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn mangan(honbas: Honbas) -> Self {
        Self::new_limited(2000.into(), honbas)
    }

    /// Constructs a new instance of `PointsCustom` with the base points value
    /// of 3000 and given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::haneman();
    /// assert_eq!(points.ko_ron().unwrap(), 12000);
    /// assert_eq!(points.is_limited(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn haneman(honbas: Honbas) -> Self {
        Self::new_limited(3000.into(), honbas)
    }

    /// Constructs a new instance of `PointsCustom` with the base points value
    /// of 4000 and given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::baiman();
    /// assert_eq!(points.ko_ron().unwrap(), 16000);
    /// assert_eq!(points.is_limited(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn baiman(honbas: Honbas) -> Self {
        Self::new_limited(4000.into(), honbas)
    }

    /// Constructs a new instance of `PointsCustom` with the base points value
    /// of 6000 and given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::sanbaiman();
    /// assert_eq!(points.ko_ron().unwrap(), 24000);
    /// assert_eq!(points.is_limited(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn sanbaiman(honbas: Honbas) -> Self {
        Self::new_limited(6000.into(), honbas)
    }

    /// Constructs a new instance of `PointsCustom` with the base points value
    /// of 8000 and given number of honbas.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::yakuman();
    /// assert_eq!(points.ko_ron().unwrap(), 32000);
    /// assert_eq!(points.is_limited(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn yakuman(honbas: Honbas) -> Self {
        Self::new_limited(8000.into(), honbas)
    }

    /// Constructs a new instance of `PointsCustom`, marking it as non-limited,
    /// or calculated (i.e. below mangan).
    ///
    /// This method allows to specify whether a value for tsumo and ron is
    /// present with `has_tsumo` and `has_ron` parameters, respectively. The
    /// number of honbas is also required.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::{Honbas, Points};
    ///
    /// // 2 han, 20 fu
    /// let points = Points::new_calculated(320, true, false, Honbas::ZERO);
    /// assert_eq!(points.is_calculated(), true);
    /// assert_eq!(points.ko_tsumo().unwrap(), (400, 700));
    /// assert_eq!(points.ko_ron().is_none(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new_calculated(
        base_points: T,
        has_tsumo: bool,
        has_ron: bool,
        honbas: Honbas,
    ) -> Self {
        Self {
            base_points,
            mode: PointsMode::Calculated { has_tsumo, has_ron },
            honbas,
        }
    }

    /// Returns true if the instance was constructed with
    /// [`Points::new_limited`].
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::new_limited(2000);
    /// assert_eq!(points.is_limited(), true);
    /// assert_eq!(points.is_calculated(), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_limited(&self) -> bool {
        match self.mode {
            PointsMode::Calculated { .. } => false,
            PointsMode::Limited => true,
        }
    }

    /// Returns true if the instance was constructed with
    /// [`Points::new_calculated`].
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::new_calculated(640, true, false);
    /// assert_eq!(points.is_calculated(), true);
    /// assert_eq!(points.is_limited(), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_calculated(&self) -> bool {
        match self.mode {
            PointsMode::Calculated { .. } => true,
            PointsMode::Limited => false,
        }
    }

    /// Returns the number of points paid for the dealer on a win by tsumo.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::mangan();
    /// assert_eq!(points.oya_tsumo().unwrap(), 4000);
    /// ```
    #[inline]
    #[must_use]
    pub fn oya_tsumo(&self) -> Option<T> {
        if self.mode.has_tsumo() {
            let value = round_up_points(self.base_points.clone() * 2) + self.tsumo_honba_points();
            Some(value)
        } else {
            None
        }
    }

    /// Returns the number of points paid for the dealer on a win by ron.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::mangan();
    /// assert_eq!(points.oya_ron().unwrap(), 12000);
    /// ```
    #[inline]
    #[must_use]
    pub fn oya_ron(&self) -> Option<T> {
        if self.mode.has_ron() {
            let value = round_up_points(self.base_points.clone() * 6) + self.ron_honba_points();
            Some(value)
        } else {
            None
        }
    }

    /// Returns the number of points paid for the non-dealer on a win by ron.
    /// The first number is the number of points paid by non-dealers, and the
    /// second is paid by the dealer.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::mangan();
    /// assert_eq!(points.ko_tsumo().unwrap(), (2000, 4000));
    /// ```
    #[inline]
    #[must_use]
    pub fn ko_tsumo(&self) -> Option<(T, T)> {
        if self.mode.has_tsumo() {
            let honba_points = self.tsumo_honba_points();
            let value_ko = round_up_points(self.base_points.clone()) + honba_points;
            let value_oya = round_up_points(self.base_points.clone() * 2) + honba_points;
            Some((value_ko, value_oya))
        } else {
            None
        }
    }

    /// Returns the number of points paid for the non-dealer on a win by ron.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::Points;
    ///
    /// let points = Points::mangan();
    /// assert_eq!(points.ko_ron().unwrap(), 8000);
    /// ```
    #[inline]
    #[must_use]
    pub fn ko_ron(&self) -> Option<T> {
        if self.mode.has_ron() {
            let value = round_up_points(self.base_points.clone() * 4) + self.ron_honba_points();
            Some(value)
        } else {
            None
        }
    }

    /// Returns the number of honbas passed when creating the value.
    ///
    /// # Examples
    /// ```
    /// use riichi_hand::points::{Honbas, Points};
    ///
    /// let points = Points::mangan(Honbas::new(3));
    /// assert_eq!(points.ko_ron().unwrap(), 8900);
    /// assert_eq!(points.honbas().get(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn honbas(&self) -> Honbas {
        self.honbas
    }

    #[inline]
    #[must_use]
    fn tsumo_honba_points(&self) -> i32 {
        self.honbas.get() * 100
    }

    #[inline]
    #[must_use]
    fn ron_honba_points(&self) -> i32 {
        self.honbas.get() * 300
    }
}

#[inline]
#[must_use]
fn round_up_points<T>(num: T) -> T
where
    T: Signed,
    T: Add<i32, Output = T>,
    T: Mul<i32, Output = T>,
    T: Div<i32, Output = T>,
{
    round_up_to(num, 100)
}

#[inline]
#[must_use]
fn round_up_to<T>(num: T, divisor: i32) -> T
where
    T: Signed,
    T: Add<i32, Output = T>,
    T: Mul<i32, Output = T>,
    T: Div<i32, Output = T>,
{
    if num.is_positive() {
        (num + (divisor - 1)) / divisor * divisor
    } else {
        num / divisor * divisor
    }
}

/// The range of [`Han`] points for a Mangan hand, no matter what the Fu value
/// is. In other words, this only includes 5 han.
pub const MANGAN_HAN_RANGE: RangeInclusive<Han> = Han::new(5)..=Han::new(5);
/// The range of [`Han`] points for a Haneman hand.
pub const HANEMAN_HAN_RANGE: RangeInclusive<Han> = Han::new(6)..=Han::new(7);
/// The range of [`Han`] points for a Baiman hand.
pub const BAIMAN_HAN_RANGE: RangeInclusive<Han> = Han::new(8)..=Han::new(10);
/// The range of [`Han`] points for a Sanbaiman hand.
pub const SANBAIMAN_HAN_RANGE: RangeInclusive<Han> = Han::new(11)..=Han::new(12);
/// The range of [`Han`] points for a Kazoe yakuman hand.
pub const KAZOE_YAKUMAN_HAN_RANGE: RangeFrom<Han> = Han::new(13)..;

/// Point calculation mode for use with [`PointsCustom::from_calculated`].
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PointsCalculationMode {
    /// Default, most strict mode. The point table is strictly followed
    /// (including missing ron/tsumo values e.g for 1 han, 20 fu), and only
    /// valid fu values can be provided.
    Default,
    /// Loose mode. Contrary to the Default mode, this allows fu values to be
    /// invalid (e.g. 10 or 21), and it returns values for all possible han/fu
    /// combinations.
    Loose,
    /// Unlimited mode, also known as [Aotenjou](https://riichi.wiki/Aotenjou) rules.
    /// This disables limiting the hands to mangan and above, possibly producing
    /// absurdly high score numbers.
    ///
    /// Using this mode, it might make sense to use data types from
    /// the [num-bigint](https://crates.io/crates/num-bigint) crate.
    Unlimited,
}

impl Default for PointsCalculationMode {
    fn default() -> Self {
        Self::Default
    }
}

const VALID_FU: [Fu; 11] = [
    Fu::new(20),
    Fu::new(25),
    Fu::new(30),
    Fu::new(40),
    Fu::new(50),
    Fu::new(60),
    Fu::new(70),
    Fu::new(80),
    Fu::new(90),
    Fu::new(100),
    Fu::new(110),
];

const NO_TSUMO: [(Han, Fu); 3] = [
    (Han::new(1), Fu::new(20)),
    (Han::new(1), Fu::new(25)),
    (Han::new(2), Fu::new(25)),
];
#[inline]
#[must_use]
fn has_tsumo(han: Han, fu: Fu) -> bool {
    !NO_TSUMO.contains(&(han, fu))
}

const NO_RON: [(Han, Fu); 5] = [
    (Han::new(1), Fu::new(20)),
    (Han::new(1), Fu::new(25)),
    (Han::new(2), Fu::new(20)),
    (Han::new(3), Fu::new(20)),
    (Han::new(4), Fu::new(20)),
];
#[inline]
#[must_use]
fn has_ron(han: Han, fu: Fu) -> bool {
    !NO_RON.contains(&(han, fu))
}

/// Error type returned when point calculation in
/// [`PointsCustom::from_calculated`] fails.
#[derive(Debug, Copy, Clone)]
pub enum PointCalculationError {
    /// Invalid han value provided (below 1).
    /// Only returned with [`PointsCalculationMode::Default`].
    InvalidHan(Han),
    /// Invalid fu value provided (below 20, above 110, or not divisible by 5).
    /// Only returned with [`PointsCalculationMode::Default`].
    InvalidFu(Fu),
    /// Invalid honba counter provided (below 0).
    /// Only returned with [`PointsCalculationMode::Default`].
    InvalidHonbas(Honbas),
}

impl Display for PointCalculationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PointCalculationError::InvalidHan(han) => {
                write!(f, "Han cannot be less than 1: {}", han)
            }
            PointCalculationError::InvalidFu(fu) => {
                write!(f, "Invalid fu value: {}", fu)
            }
            PointCalculationError::InvalidHonbas(honbas) => {
                write!(f, "Invalid honba count: {}", honbas)
            }
        }
    }
}

impl Error for PointCalculationError {}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use crate::points::{Fu, Han, Honbas, Points, PointsCalculationMode, PointsCustom};

    #[derive(Debug, serde::Deserialize)]
    struct PointsRecord {
        han: i32,
        fu: i32,
        ko_tsumo_1: i32,
        ko_tsumo_2: i32,
        ko_ron: i32,
        oya_ron: i32,
    }

    #[test]
    fn should_fail_for_invalid_fu() {
        // Valid fu
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(20);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(2);
        let fu = Fu::new(110);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());

        // Loose mode
        let calculation_mode = PointsCalculationMode::Loose;
        let han = Han::new(1);
        let fu = Fu::new(13);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Loose;
        let han = Han::new(1);
        let fu = Fu::new(35);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Loose;
        let han = Han::new(1);
        let fu = Fu::new(150);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Loose;
        let han = Han::new(1);
        let fu = Fu::new(10);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());

        // Unlimited mode
        let calculation_mode = PointsCalculationMode::Unlimited;
        let han = Han::new(1);
        let fu = Fu::new(13);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Unlimited;
        let han = Han::new(1);
        let fu = Fu::new(35);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Unlimited;
        let han = Han::new(1);
        let fu = Fu::new(150);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());
        let calculation_mode = PointsCalculationMode::Unlimited;
        let han = Han::new(1);
        let fu = Fu::new(10);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_ok());

        // Invalid fu
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(13);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_err());
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(35);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_err());
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(150);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_err());
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(10);
        assert!(Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).is_err());
    }

    #[test]
    fn should_display_invalid_fu_error() {
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(1);
        let fu = Fu::new(35);
        let invalid_fu = Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO);
        let invalid_fu_error = invalid_fu.unwrap_err();
        assert_eq!(invalid_fu_error.to_string(), "Invalid fu value: 35 fu");
    }

    #[test]
    fn should_display_invalid_han_error() {
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(-5);
        let fu = Fu::new(30);
        let invalid_han = Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO);
        let invalid_han_error = invalid_han.unwrap_err();
        assert_eq!(
            invalid_han_error.to_string(),
            "Han cannot be less than 1: -5 han"
        );
    }

    #[test]
    fn should_display_invalid_honbas_error() {
        let calculation_mode = PointsCalculationMode::Default;
        let han = Han::new(3);
        let fu = Fu::new(30);
        let honbas = Honbas::new(-1);
        let invalid_honbas = Points::from_calculated(calculation_mode, han, fu, honbas);
        let invalid_honbas_error = invalid_honbas.unwrap_err();
        assert_eq!(
            invalid_honbas_error.to_string(),
            "Invalid honba count: -1 honbas"
        );
    }

    #[test]
    fn should_return_limited() {
        let mangan = (2000, 4000, 8000, 12000);
        check_points_default_limited(5, 40, mangan);
        check_points_default_limited(4, 40, mangan);
        check_points_default_limited(4, 60, mangan);
        check_points_default_limited(3, 100, mangan);
        check_points_default_limited(3, 110, mangan);

        let haneman = (3000, 6000, 12000, 18000);
        check_points_default_limited(6, 30, haneman);
        check_points_default_limited(7, 30, haneman);

        let baiman = (4000, 8000, 16000, 24000);
        check_points_default_limited(8, 30, baiman);
        check_points_default_limited(9, 30, baiman);
        check_points_default_limited(10, 30, baiman);

        let sanbaiman = (6000, 12000, 24000, 36000);
        check_points_default_limited(11, 30, sanbaiman);
        check_points_default_limited(12, 30, sanbaiman);

        let kazoe_yakuman = (8000, 16000, 32000, 48000);
        check_points_default_limited(13, 30, kazoe_yakuman);
        check_points_default_limited(14, 30, kazoe_yakuman);
        check_points_default_limited(17, 30, kazoe_yakuman);
        check_points_default_limited(25, 30, kazoe_yakuman);
        check_points_default_limited(100, 30, kazoe_yakuman);
    }

    #[test]
    fn should_handle_honbas() {
        check_points_loose_with_honbas(1, 30, 1, (400, 600, 1300, 1800));
        check_points_loose_with_honbas(1, 30, 5, (800, 1000, 2500, 3000));
        check_points_loose_with_honbas(1, 30, -3, (0, 200, 100, 600));
        check_points_loose_with_honbas(1, 30, -5, (-200, 0, -500, 0));
        check_points_loose_with_honbas(3, 30, 10, (2000, 3000, 6900, 8800));
        check_points_loose_with_honbas(5, 30, 1, (2100, 4100, 8300, 12300));
    }

    #[test]
    fn should_return_calculated() {
        let points_table = include_bytes!("points/points_table.csv");
        let mut csv_reader = csv::Reader::from_reader(&points_table[..]);
        for result in csv_reader.deserialize() {
            let record: PointsRecord = result.unwrap();
            let han = Han::new(record.han);
            let fu = Fu::new(record.fu);

            let calculation_mode = PointsCalculationMode::Default;
            let points = Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).unwrap();
            let ko_tsumo = points.ko_tsumo().unwrap_or_default();
            let ko_ron = points.ko_ron().unwrap_or_default();
            let oya_ron = points.oya_ron().unwrap_or_default();

            let actual = (ko_tsumo.0, ko_tsumo.1, ko_ron, oya_ron);
            let expected = (
                record.ko_tsumo_1,
                record.ko_tsumo_2,
                record.ko_ron,
                record.oya_ron,
            );

            assert_eq!(
                actual, expected,
                "Points for {} and {} are different",
                han, fu
            );
        }
    }

    #[test]
    fn should_work_with_loose_mode() {
        check_points_loose(1, 1, (100, 100, 100, 100));
        check_points_loose(1, 13, (200, 300, 500, 700));
        check_points_loose(1, 150, (1200, 2400, 4800, 7200));
        check_points_loose(3, 150, (2000, 4000, 8000, 12000));
        check_points_loose(15, 150, (8000, 16000, 32000, 48000));
    }

    #[test]
    fn should_work_with_unlimited_mode() {
        check_points_unlimited(1, 1, (100, 100, 100, 100));
        check_points_unlimited(1, 13, (200, 300, 500, 700));
        check_points_unlimited(1, 150, (1200, 2400, 4800, 7200));
        check_points_unlimited(3, 150, (4800, 9600, 19200, 28800));
        check_points_unlimited(15, 150, (19660800, 39321600, 78643200, 117964800));
        check_points_unlimited(4, 1500, (96000, 192000, 384000, 576000));
        check_points_unlimited(20, 40, (167772200, 335544400, 671088700, 1006633000));
    }

    #[test]
    fn should_work_with_non_positive_numbers() {
        check_points_unlimited(0, 30, (200, 300, 500, 800));
        check_points_unlimited(-1, 30, (100, 200, 300, 400));
        check_points_unlimited(-1, 70, (200, 300, 600, 900));
        check_points_unlimited(-2, 30, (100, 100, 200, 200));
        check_points_unlimited(-5, 30, (100, 100, 100, 100));
        check_points_unlimited(-10, 30, (100, 100, 100, 100));
        check_points_unlimited(4, -30, (-1900, -3800, -7600, -11500));
        check_points_unlimited(4, -50, (-3200, -6400, -12800, -19200));
        check_points_unlimited(-4, -100, (0, 0, -100, -100));
        check_points_unlimited(-10000, i32::MAX, (100, 100, 100, 100));
        check_points_unlimited(-6, i32::MAX, (134217800, 268435500, 536871000, 805306400));
    }

    #[test]
    fn should_work_with_bigints_and_unlimited_mode() {
        check_points_unlimited_bigint(
            20,
            40,
            ("167772200", "335544400", "671088700", "1006633000"),
        );
        check_points_unlimited_bigint(
            160,
            250,
            (
                "1461501637330902918203684832716283019655932542976000",
                "2923003274661805836407369665432566039311865085952000",
                "5846006549323611672814739330865132078623730171904000",
                "8769009823985417509222108996297698117935595257856000",
            ),
        );
        // Beginning of the cosmos from Koizumi
        check_points_unlimited_bigint(
            105,
            140,
            (
                "22716298756089870874820921440338000",
                "45432597512179741749641842880675900",
                "90865195024359483499283685761351700",
                "136297792536539225248925528642027600",
            ),
        );
    }

    fn check_points_default_limited(han: i32, fu: i32, expected_points: (i32, i32, i32, i32)) {
        let han = Han::new(han);
        let fu = Fu::new(fu);
        let calculation_mode = PointsCalculationMode::Default;
        let points = Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).unwrap();
        assert!(points.is_limited());
        assert!(!points.is_calculated());

        check_points(&points, han, fu, &expected_points);
    }

    fn check_points_loose(han: i32, fu: i32, expected_points: (i32, i32, i32, i32)) {
        check_points_loose_with_honbas(han, fu, 0, expected_points);
    }

    fn check_points_loose_with_honbas(
        han: i32,
        fu: i32,
        honbas: i32,
        expected_points: (i32, i32, i32, i32),
    ) {
        let han = Han::new(han);
        let fu = Fu::new(fu);
        let honbas = Honbas::new(honbas);
        let calculation_mode = PointsCalculationMode::Loose;
        let points = Points::from_calculated(calculation_mode, han, fu, honbas).unwrap();
        check_points(&points, han, fu, &expected_points);
    }

    fn check_points_unlimited(han: i32, fu: i32, expected_points: (i32, i32, i32, i32)) {
        let han = Han::new(han);
        let fu = Fu::new(fu);
        let calculation_mode = PointsCalculationMode::Unlimited;
        let points = Points::from_calculated(calculation_mode, han, fu, Honbas::ZERO).unwrap();
        check_points(&points, han, fu, &expected_points);
    }

    fn check_points_unlimited_bigint(han: i32, fu: i32, expected_points: (&str, &str, &str, &str)) {
        let han = Han::new(han);
        let fu = Fu::new(fu);
        let calculation_mode = PointsCalculationMode::Unlimited;
        let points =
            PointsCustom::from_calculated(calculation_mode, han, fu, Honbas::ZERO).unwrap();
        check_points_bigint(&points, han, fu, &expected_points);
    }

    fn check_points(points: &Points, han: Han, fu: Fu, expected_points: &(i32, i32, i32, i32)) {
        let ko_tsumo = points.ko_tsumo().unwrap_or_default();
        let ko_ron = points.ko_ron().unwrap_or_default();
        let oya_tsumo = points.oya_tsumo().unwrap_or_default();
        let oya_ron = points.oya_ron().unwrap_or_default();

        let actual_points = (ko_tsumo.0, ko_tsumo.1, ko_ron, oya_ron);

        assert!(points.is_limited() ^ points.is_calculated());
        assert_eq!(ko_tsumo.1, oya_tsumo);
        assert_eq!(
            actual_points, *expected_points,
            "Points for {} and {} are different",
            han, fu
        );
    }

    fn check_points_bigint(
        points: &PointsCustom<BigInt>,
        han: Han,
        fu: Fu,
        expected_points: &(&str, &str, &str, &str),
    ) {
        let ko_tsumo = points.ko_tsumo().unwrap_or_default();
        let ko_ron = points.ko_ron().unwrap_or_default();
        let oya_tsumo = points.oya_tsumo().unwrap_or_default();
        let oya_ron = points.oya_ron().unwrap_or_default();

        let actual_points = (
            ko_tsumo.0.to_string(),
            ko_tsumo.1.to_string(),
            ko_ron.to_string(),
            oya_ron.to_string(),
        );
        let actual_points_ref = (
            actual_points.0.as_ref(),
            actual_points.1.as_ref(),
            actual_points.2.as_ref(),
            actual_points.3.as_ref(),
        );

        assert_eq!(ko_tsumo.1, oya_tsumo);
        assert_eq!(
            actual_points_ref, *expected_points,
            "Points for {} and {} are different",
            han, fu
        );
    }
}
