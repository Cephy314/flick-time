//! Fixed-point time representation.
//!
//! Units of time in a Flick represent `1/705_600_000`th of a second.
//! which allows common Hz rates used in media such as 24 or 60 frames per second.
//! Additionally this can support NTSC broadcast frame rates and common audio sample rates.
use core::ops::Add;

/// There are `705_600_000` flicks per second.
pub const FLICKS_PER_SECOND: i64 = 705_600_000;

/// Integral time representation divided into [`FLICKS_PER_SECOND`]
///
/// Represents +/- 414 years.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Flick(i64);

impl IntoIterator for Flick {
    type IntoIter = FlickIter;
    type Item = i64;

    fn into_iter(self) -> Self::IntoIter {
        FlickIter(self.0)
    }
}

impl Add<Flick> for Flick {
    type Output = Flick;
    fn add(self, rhs: Self) -> Flick {
        Flick(self.0.saturating_add(rhs.0))
    }
}

impl AsRef<i64> for Flick {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Flick {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

macro_rules! impl_from_flick {

    ($($ty:ty),* $(,)?) => {
        $(
        impl From<Flick> for $ty {
            fn from(flick: Flick) -> Self {
                flick.0 as _
            }
        }

        impl From<&Flick> for $ty {
            fn from(flick: &Flick) -> Self {
                flick.0 as _
            }
        }
        )*
    };
}

impl_from_flick!(u64, u128, usize, i64, i128, isize, f32, f64);

macro_rules! impl_flick_from {

    ($($ty:ty),* $(,)?) => {
        $(
        impl From<$ty> for Flick {
            fn from(value: $ty) -> Self {
                Self(value as _)
            }
        }
        )*
    };
}

impl_flick_from!(u8, u16, u32, i8, i16, i32, i64);

impl Flick {}
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
pub enum Rate {
    /// NTSC 24 Fps (24/1001)
    Hz23_98,
    /// 24 Fps
    Hz24,
    /// 25 Fps,
    Hz25,
    /// NTSC 30 Fps (30/1001)
    Hz29_97,
    /// 30 Fps
    Hz30,
    /// 48 Fps
    Hz48,
    /// 50 Fps
    Hz50,
    /// NTSC 59.94 Fps (60/1001)
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

impl Rate {
    /// Retrieve the number of [`Flick`]s which represents the given Hz per second.
    pub const fn flicks_per_second(self) -> i64 {
        match self {
            Self::Hz23_98 => 29_429_400,
            Self::Hz24 => 29_400_000,
            Self::Hz25 => 28_224_000,
            Self::Hz29_97 => 23_543_520,
            Self::Hz30 => 23_520_000,
            Self::Hz48 => 14_700_000,
            Self::Hz50 => 14_122_000,
            Self::Hz59_94 => 11_771_760,
            Self::Hz60 => 11_760_000,
            Self::Hz90 => 7_840_000,
            Self::Hz120 => 5_880_000,
            Self::Hz144 => 4_900_000,
            Self::Hz240 => 2_940_000,
            Self::Hz44_100 => 16_000,
            Self::Hz48_000 => 14_700,
            Self::Hz88_200 => 8_000,
            Self::Hz96_000 => 7_350,
            Self::Hz176_400 => 4_000,
            Self::Hz192_000 => 3_675,
        }
    }
}
