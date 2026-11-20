//! Fixed-point time representation.
//!
//! Units of time in a Flick represent `1/705_600_000`th of a second.
//! which allows common Hz rates used in media such as 24 or 60 frames per second.
//! Additionally this can support NTSC broadcast frame rates and common audio sample rates.
use core::fmt::Debug;
use core::ops::Add;

type Result<T> = core::result::Result<T, FlickError>;
/// There are `705_600_000` flicks per second.
pub const FLICKS_PER_SECOND: i64 = 705_600_000;

/// Integral time representation divided into [`FLICKS_PER_SECOND`]
///
/// Represents +/- 414 years.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Flicks(i64);

impl IntoIterator for Flicks {
    type IntoIter = FlickIter;
    type Item = i64;

    fn into_iter(self) -> Self::IntoIter {
        FlickIter(self.0)
    }
}

impl Add<Flicks> for Flicks {
    type Output = Flicks;
    fn add(self, rhs: Self) -> Flicks {
        Flicks(self.0.saturating_add(rhs.0))
    }
}

impl AsRef<i64> for Flicks {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Flicks {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

macro_rules! impl_from_flick {

    ($($ty:ty),* $(,)?) => {
        $(
        impl From<Flicks> for $ty {
            fn from(flicks: Flicks) -> Self {
                flicks.0 as _
            }
        }

        impl From<&Flicks> for $ty {
            fn from(flicks: &Flicks) -> Self {
                flicks.0 as _
            }
        }
        )*
    };
}

impl_from_flick!(u64, u128, usize, i64, i128, isize, f32, f64);

macro_rules! impl_flick_from {

    ($($ty:ty),* $(,)?) => {
        $(
        impl From<$ty> for Flicks {
            fn from(value: $ty) -> Self {
                Self(value as _)
            }
        }
        )*
    };
}

impl_flick_from!(u8, u16, u32, i8, i16, i32, i64);

impl Flicks {
    /// Create new [`Flicks`] with given i64
    pub const fn new(value: i64) -> Self {
        Flicks(value)
    }

    /// Calculate the number of flicks per second.
    ///
    /// Accepts a `numerator` and `denominator`.
    ///
    /// Returns Some(i64) flicks per second value or None if result is 0 or invalid.
    ///
    /// # Example
    ///
    /// ```
    /// use flick_time::{Flicks,FlickError};
    ///
    /// assert_eq!(Flicks::per_cycle(24,1),Ok(Flicks::new(29_400_000)));
    /// assert_eq!(Flicks::per_cycle(30_000,1001),Ok(Flicks::new(23_543_520)));
    /// assert!(Flicks::per_cycle(705_600_001,1).is_err());
    /// ```
    pub const fn per_cycle(num: i64, den: i64) -> Result<Flicks> {
        if num == 0 || den == 0 {
            return Err(FlickError::DivideByZero(num, den));
        }

        if FLICKS_PER_SECOND * den % num > 0 {
            return Err(FlickError::Remainder(num, den));
        }
        let Some(top) = FLICKS_PER_SECOND.checked_mul(den) else {
            return Err(FlickError::Overflow);
        };

        let Some(flicks) = top.checked_div(num) else {
            return Err(FlickError::Overflow);
        };
        Ok(Flicks(flicks))
    }
}

/// Error types for [`Flick`] type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FlickError {
    /// An attempt to use a ratio with 0 as denominator or numerator.
    #[non_exhaustive]
    DivideByZero(i64, i64),
    /// Does not divide evenly into flicks.
    #[non_exhaustive]
    Remainder(i64, i64),
    /// Ratios must be greater than zero.
    #[non_exhaustive]
    InvalidRatio,
    /// Overflow occurred on operation
    #[non_exhaustive]
    Overflow,
}

impl Debug for FlickError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivideByZero(num, den) => {
                write!(f, "No division by zero: {0} / {1}", num, den)
            }
            Self::Remainder(num, den) => {
                write!(
                    f,
                    "Does not divide cleaning into flicks (705_600_000): {0} / {1}",
                    num, den
                )
            }
            Self::InvalidRatio => {
                write!(f, "Ratio is invalid, must be greater than zero.")
            }
            Self::Overflow => {
                write!(f, "Operation overflowed.")
            }
        }
    }
}

/// An iterator oveer [`Flick`]s,
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FlickIter(i64);

impl Iterator for FlickIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if i64::MAX == self.0 {
            None
        } else {
            let value = self.0;
            self.0 += 1;

            Some(value)
        }
    }
}

/// Common rates (Hz) which divide cleanly into FLICKS.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FlicksPerCycle {
    /// NTSC 24 Fps (24000/1001)
    Hz23_98,
    /// 24 Fps
    Hz24,
    /// 25 Fps,
    Hz25,
    /// NTSC 30 Fps (30000/1001)
    Hz29_97,
    /// 30 Fps
    Hz30,
    /// 48 Fps
    Hz48,
    /// 50 Fps
    Hz50,
    /// NTSC 59.94 Fps (600000/1001)
    Hz59_94,
    /// 60 Fps
    Hz60,
    /// 90 Fps
    Hz90,
    /// 120 Fps
    Hz120,
    /// 144 Hz
    Hz144,
    /// 240 Hz
    Hz240,
    /// 44.1 kHz
    Hz44_100,
    /// 48 kHz
    Hz48_000,
    /// 88.2 kHz
    Hz88_200,
    /// 96 kHz
    Hz96_000,
    /// 176.4 kHz
    Hz176_400,
    /// 192 kHz
    Hz192_000,
}

impl FlicksPerCycle {
    /// Retrieve the number of [`Flick`]s which represents the given Hz per second.
    pub const fn flicks_per_second(self) -> Flicks {
        match self {
            Self::Hz23_98 => Flicks(29_429_400),
            Self::Hz24 => Flicks(29_400_000),
            Self::Hz25 => Flicks(28_224_000),
            Self::Hz29_97 => Flicks(23_543_520),
            Self::Hz30 => Flicks(23_520_000),
            Self::Hz48 => Flicks(14_700_000),
            Self::Hz50 => Flicks(14_122_000),
            Self::Hz59_94 => Flicks(11_771_760),
            Self::Hz60 => Flicks(11_760_000),
            Self::Hz90 => Flicks(7_840_000),
            Self::Hz120 => Flicks(5_880_000),
            Self::Hz144 => Flicks(4_900_000),
            Self::Hz240 => Flicks(2_940_000),
            Self::Hz44_100 => Flicks(16_000),
            Self::Hz48_000 => Flicks(14_700),
            Self::Hz88_200 => Flicks(8_000),
            Self::Hz96_000 => Flicks(7_350),
            Self::Hz176_400 => Flicks(4_000),
            Self::Hz192_000 => Flicks(3_675),
        }
    }
}
