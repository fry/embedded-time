use crate::integer::{IntTrait, Integer};
use crate::numerical_duration::NumericalDuration;
use crate::Ratio;
use core::convert::TryInto;
use core::fmt::Formatter;
use core::{convert, fmt, ops};

pub trait Duration<T: IntTrait + NumericalDuration>: Sized + fmt::Display {
    const PERIOD: Period;

    fn new(value: T) -> Self;

    fn count(self) -> T;

    fn period() -> Period {
        Self::PERIOD
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::duration::Seconds;
    /// assert_eq!(Seconds::<i32>::min_value(), i32::MIN);
    /// ```
    fn min_value() -> T {
        T::min_value()
    }

    /// ```rust
    /// # use embedded_time::prelude::*;
    /// # use embedded_time::duration::Seconds;
    /// assert_eq!(Seconds::<i32>::max_value(), i32::MAX);
    /// ```
    fn max_value() -> T {
        T::max_value()
    }

    /// ```rust
    /// # use embedded_time::duration::{Seconds, Milliseconds, Microseconds, Duration};
    /// assert_eq!(Milliseconds::from_dur(Seconds(1_000)), Milliseconds(1_000_000));
    /// assert_eq!(Seconds::from_dur(Milliseconds(1_234)), Seconds(1));
    /// assert_eq!(Microseconds::from_dur(Milliseconds(1_234)), Microseconds(1_234_000));
    /// ```
    fn from_dur<U: Duration<T>>(other: U) -> Self {
        Self::new(*(Integer(other.count()) * (U::PERIOD / Self::PERIOD)))
    }

    /// ```rust
    /// # use embedded_time::duration::{Seconds, Milliseconds, Microseconds, Duration};
    /// assert_eq!(Milliseconds(1_000_000), Seconds(1_000).into_dur());
    /// assert_eq!(Seconds(2), Milliseconds(2_345).into_dur());
    /// ```
    fn into_dur<U: Duration<T>>(self) -> U {
        U::new(*(Integer(self.count()) * (Self::PERIOD / U::PERIOD)))
    }
}

macro_rules! durations {
    ( $( $name:ident, ($numer:expr, $denom:expr) );+ ) => {
        $(
            #[derive(Copy, Clone, Eq, PartialEq, Debug)]
            pub struct $name<T: IntTrait + NumericalDuration>(pub T);

            impl<T: IntTrait + NumericalDuration> Duration<T> for $name<T> {
                const PERIOD: Period = Period::new_raw($numer, $denom);

                fn new(value: T) -> Self {
                    Self(value)
                }

                fn count(self) -> T {
                    self.0
                }
            }

            impl<T: IntTrait + NumericalDuration> fmt::Display for $name<T> {
                fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.count())
                }
            }

         )+
     };
}

durations![Seconds, (1, 1); Milliseconds, (1, 1_000); Microseconds, (1, 1_000_000)];

pub(crate) type Period = Ratio<i32>;

impl<T: IntTrait> ops::Mul<Period> for Integer<T> {
    type Output = Self;

    fn mul(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.numer()).into() / (*rhs.denom()).into())
    }
}

impl<T: IntTrait> ops::Div<Period> for Integer<T> {
    type Output = Self;

    fn div(self, rhs: Period) -> Self::Output {
        Self(self.0 * (*rhs.denom()).into() / (*rhs.numer()).into())
    }
}

// /// A time duration with a fractional period in seconds
// ///
// /// It replicates many of the `as_` and `from_` methods found on the [`core::time::Duration`](https://doc.rust-lang.org/core/time/struct.Duration.html) type.
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
// pub struct Duration<R: IntTrait> {
//     value: R,
//     period: Ratio<R>,
// }
//
// impl<R: IntTrait + NumericalDuration> Duration<R> {
//     /// The number of seconds in one minute.
//     const SECONDS_PER_MINUTE: i16 = 60;
//
//     /// The number of seconds in one hour.
//     const SECONDS_PER_HOUR: i16 = 60 * Self::SECONDS_PER_MINUTE;
//
//     pub const fn new(value: R, period: Ratio<R>) -> Self {
//         Self { value, period }
//     }
//
//     // /// Equivalent to `0.seconds()`.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::zero(), 0.seconds());
//     // /// ```
//     // #[inline(always)]
//     // pub fn zero() -> Self {
//     //     Self::from_secs(R::zero())
//     // }
//
//     // /// The maximum possible duration. Adding any positive duration to this will
//     // /// cause an overflow.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Ratio, Duration};
//     // /// assert_eq!(Duration::max_value(Ratio::new(1,1)).as_secs(), i32::MAX);
//     // /// ```
//     // ///
//     // /// The value returned by this method may change at any time.
//     // #[inline(always)]
//     // pub fn max_value(period: Ratio<R>) -> Self {
//     //     Self {
//     //         value: R::max_value(),
//     //     }
//     // }
//
//     // /// The minimum possible duration. Adding any negative duration to this will
//     // /// cause an overflow.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Ratio, Duration};
//     // /// assert_eq!(Duration::min_value(Ratio::new(1,1)).as_secs(), i32::MIN);
//     // /// ```
//     // ///
//     // /// The value returned by this method may change at any time.
//     // #[inline(always)]
//     // pub fn min_value(period: Ratio<R>) -> Self {
//     //     Self {
//     //         value: R::min_value(),
//     //         period,
//     //     }
//     // }
//     //
//     // /// Create a new `Duration` with the given number of hours. Equivalent to
//     // /// `Duration::seconds(hours * 3_600)`.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_hours(1), 3_600.seconds());
//     // /// ```
//     // #[inline(always)]
//     // pub fn from_hours(hours: R) -> Self {
//     //     Self {
//     //         value: hours,
//     //         period: Ratio::<R>::new(R::from(Self::SECONDS_PER_HOUR).unwrap(), R::one()),
//     //     }
//     // }
//     //
//     // /// Get the number of whole hours in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(1.hours().as_hours(), 1);
//     // /// assert_eq!((-1).hours().as_hours(), -1);
//     // /// assert_eq!(59.minutes().as_hours(), 0);
//     // /// assert_eq!((-59).minutes().as_hours(), 0);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_hours(self) -> R {
//     //     let hours = Ratio::from_integer(self.value)
//     //         / Ratio::new(R::from(Self::SECONDS_PER_HOUR).unwrap(), R::one())
//     //         * self.period;
//     //     hours.to_integer()
//     // }
//     //
//     // /// Create a new `Duration` with the given number of minutes. Equivalent to
//     // /// `Duration::seconds(minutes * 60)`.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_mins(1), 60.seconds());
//     // /// ```
//     // #[inline(always)]
//     // pub fn from_mins(minutes: R) -> Self {
//     //     Self {
//     //         value: minutes,
//     //         period: Ratio::<R>::new(R::from(Self::SECONDS_PER_MINUTE).unwrap(), R::one()),
//     //     }
//     // }
//     //
//     // /// Get the number of whole minutes in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(1.minutes().as_mins(), 1);
//     // /// assert_eq!((-1).minutes().as_mins(), -1);
//     // /// assert_eq!(59.seconds().as_mins(), 0);
//     // /// assert_eq!((-59).seconds().as_mins(), 0);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_mins(self) -> R {
//     //     let mins = Ratio::from_integer(self.value)
//     //         / Ratio::new(R::from(Self::SECONDS_PER_MINUTE).unwrap(), R::one())
//     //         * self.period;
//     //     mins.to_integer()
//     // }
//     //
//     // /// Create a new `Duration` with the given number of seconds.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_secs(1).as_millis(), 1_000.milliseconds().as_millis());
//     // /// ```
//     // #[inline(always)]
//     // pub fn from_secs(seconds: R) -> Self {
//     //     Self {
//     //         value: seconds,
//     //         period: Ratio::new(R::one(), R::one()),
//     //     }
//     // }
//     //
//     // /// Get the number of whole seconds in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(1.seconds().as_secs(), 1);
//     // /// assert_eq!((-1).seconds().as_secs(), -1);
//     // /// assert_eq!(1.minutes().as_secs(), 60);
//     // /// assert_eq!((-1).minutes().as_secs(), -60);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_secs(self) -> R {
//     //     self.value
//     // }
//     //
//     // /// Create a new `Duration` with the given number of milliseconds.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_millis(1_000), 1.seconds());
//     // /// assert_eq!(Duration::from_millis(-1_000), (-1).seconds());
//     // /// ```
//     // #[inline(always)]
//     // #[allow(clippy::cast_possible_truncation)]
//     // pub fn from_millis(milliseconds: R) -> Self {
//     //     Self {
//     //         value: milliseconds,
//     //         period: Ratio::<R>::new(R::one(), R::from(1_000).unwrap()),
//     //     }
//     // }
//     //
//     // /// Get the number of whole milliseconds in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(1.seconds().as_millis(), 1_000);
//     // /// assert_eq!((-1).seconds().as_millis(), -1_000);
//     // /// assert_eq!(1.milliseconds().as_millis(), 1);
//     // /// assert_eq!((-1).milliseconds().as_millis(), -1);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_millis(self) -> R {
//     //     let millis = Ratio::from_integer(self.value)
//     //         / Ratio::new(R::one(), R::from(1_000).unwrap())
//     //         * self.period;
//     //     millis.to_integer()
//     // }
//     //
//     // /// Create a new `Duration` with the given number of microseconds.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_micros(1), 1_000.nanoseconds());
//     // /// assert_eq!(Duration::from_micros(-1), (-1_000).nanoseconds());
//     // /// ```
//     // #[inline(always)]
//     // #[allow(clippy::cast_possible_truncation)]
//     // pub fn from_micros(microseconds: R) -> Self {
//     //     Self {
//     //         value: microseconds,
//     //         period: Ratio::<R>::new(R::one(), R::from(1_000_000).unwrap()),
//     //     }
//     // }
//     //
//     // /// Get the number of whole microseconds in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(1.milliseconds().as_micros(), 1_000);
//     // /// assert_eq!((-1).milliseconds().as_micros(), -1_000);
//     // /// assert_eq!(1.microseconds().as_micros(), 1);
//     // /// assert_eq!((-1).microseconds().as_micros(), -1);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_micros(self) -> R {
//     //     let micros = Ratio::from_integer(self.value)
//     //         / Ratio::new(R::one(), R::from(1_000_000).unwrap())
//     //         * self.period;
//     //     micros.to_integer()
//     // }
//     //
//     // /// Create a new `Duration` with the given number of nanoseconds.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(Duration::from_nanos(1_000), 1.microseconds());
//     // /// assert_eq!(Duration::from_nanos(-1_000), (-1).microseconds());
//     // /// ```
//     // #[inline(always)]
//     // #[allow(clippy::cast_possible_truncation)]
//     // pub fn from_nanos(nanoseconds: R) -> Self {
//     //     Self {
//     //         value: nanoseconds,
//     //         period: Ratio::<R>::new(R::one(), R::from(1_000_000_000).unwrap()),
//     //     }
//     // }
//     //
//     // /// Get the number of nanoseconds in the duration.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(1.microseconds().as_nanos(), 1_000);
//     // /// assert_eq!((-1).microseconds().as_nanos(), -1_000);
//     // /// assert_eq!(1.nanoseconds().as_nanos(), 1);
//     // /// assert_eq!((-1).nanoseconds().as_nanos(), -1);
//     // /// ```
//     // #[inline(always)]
//     // pub fn as_nanos(self) -> R {
//     //     let nanos = Ratio::from_integer(self.value)
//     //         / Ratio::new(R::one(), R::from(1_000_000_000).unwrap())
//     //         * self.period;
//     //     nanos.to_integer()
//     // }
//     //
//     // /// Computes `self + rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*, Ratio};
//     // /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
//     // /// assert_eq!(Duration::max_value(Ratio::new(1,1_000)).checked_add(1.milliseconds()), None);
//     // /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
//     // /// ```
//     // #[inline]
//     // pub fn checked_add(self, rhs: Self) -> Option<Self> {
//     //     let value = self.value.checked_add(&rhs.value)?;
//     //
//     //     Some(Self {
//     //         value,
//     //         period: self.period,
//     //     })
//     // }
//
//     // /// Computes `self - rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
//     // /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
//     // /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_sub(self, rhs: Self) -> Option<Self> {
//     //     self.checked_add(-rhs)
//     // }
//     //
//     // /// Computes `self * rhs`, returning `None` if an overflow occurred.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::{Duration, prelude::*};
//     // /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
//     // /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
//     // /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
//     // /// assert_eq!(Duration::max_value().checked_mul(2), None);
//     // /// assert_eq!(Duration::min_value().checked_mul(2), None);
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_mul(self, rhs: Integer) -> Option<Self> {
//     //     // Multiply nanoseconds as i64, because it cannot overflow that way.
//     //     let value = self.value.checked_mul(rhs)?;
//     //
//     //     Some(Self { value })
//     // }
//     //
//     // /// Computes `self / rhs`, returning `None` if `rhs == 0`.
//     // ///
//     // /// ```rust
//     // /// # use embedded_time::prelude::*;
//     // /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
//     // /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
//     // /// assert_eq!(1.seconds().checked_div(0), None);
//     // /// ```
//     // #[inline(always)]
//     // pub fn checked_div(self, rhs: Integer) -> Option<Self> {
//     //     if rhs == 0 {
//     //         return None;
//     //     }
//     //     let value = self.value / rhs;
//     //
//     //     Some(Self { value })
//     // }
// }

// impl<R: IntTrait> PartialEq for Duration<R> {
//     /// ```rust
//     /// # use embedded_time::{Duration, prelude::*};
//     /// assert_eq!(1_000.milliseconds(), 1.seconds());
//     /// assert_eq!((-1_000).milliseconds(), (-1).seconds());
//     /// ```
//     fn eq(&self, other: &Self) -> bool {
//         (Ratio::from_integer(self.value) * self.period)
//             == (Ratio::from_integer(other.value) * other.period)
//     }
// }

/// ```rust
/// # use embedded_time::prelude::*;
/// use embedded_time::duration::{Seconds, Milliseconds};
/// assert_eq!((Seconds(3_i32) + Seconds(2_i32)).count(), 5_i32);
/// assert_eq!((Seconds(3_i64) + Seconds(2_i64)).count(), 5_i64);
/// ```
impl<T> ops::Add for Seconds<T>
where
    T: IntTrait + NumericalDuration,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// use embedded_time::duration::{Seconds, Milliseconds};
/// assert_eq!((Seconds(3_i32) - Seconds(2_i32)).count(), 1_i32);
/// assert_eq!((Seconds(3_i64) - Seconds(2_i64)).count(), 1_i64);
/// ```
impl<T> ops::Sub for Seconds<T>
where
    T: IntTrait + NumericalDuration,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// ```rust
/// # use embedded_time::prelude::*;
/// use embedded_time::duration::{Seconds, Milliseconds};
/// assert_eq!((Milliseconds(3_i32) - Milliseconds(2_i32)).count(), 1_i32);
/// assert_eq!((Milliseconds(3_i64) - Milliseconds(2_i64)).count(), 1_i64);
/// ```
impl<T> ops::Sub for Milliseconds<T>
where
    T: IntTrait + NumericalDuration,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

// impl<R: IntTrait + NumericalDuration> ops::AddAssign for Duration<R> {
//     #[inline(always)]
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Neg for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     fn neg(self) -> Self::Output {
//         self * R::from(-1).unwrap()
//     }
// }
//
// /// ```rust
// /// # use embedded_time::prelude::*;
// /// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
// /// ```
// impl<R: IntTrait> ops::Sub for Duration<R> {
//     type Output = Self;
//
//     #[inline]
//     fn sub(self, rhs: Self) -> Self::Output {
//         let fraction = (Ratio::from_integer(self.value) * self.period)
//             - (Ratio::from_integer(rhs.value) * rhs.period);
//         let value = (fraction / self.period).to_integer();
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::SubAssign for Duration<R> {
//     #[inline(always)]
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Mul<R> for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     #[allow(trivial_numeric_casts)]
//     fn mul(self, rhs: R) -> Self::Output {
//         let value = self.value * rhs;
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::MulAssign<R> for Duration<R> {
//     #[inline(always)]
//     fn mul_assign(&mut self, rhs: R) {
//         *self = *self * rhs;
//     }
// }
//
// impl<R: IntTrait> ops::Div<R> for Duration<R> {
//     type Output = Self;
//
//     #[inline(always)]
//     fn div(self, rhs: R) -> Self::Output {
//         let value = self.value / rhs;
//
//         Self {
//             value,
//             period: self.period,
//         }
//     }
// }
//
// impl<R: IntTrait> ops::DivAssign<R> for Duration<R> {
//     #[inline(always)]
//     fn div_assign(&mut self, rhs: R) {
//         *self = *self / rhs;
//     }
// }
