use crate::conversion::bytes_de::s_from_bytes;
use crate::conversion::bytes_ser::s_to_bytes;
use crate::conversion::raw_parts::{s_as_raw_mantissa, s_from_raw_parts};
use crate::conversion::string::s_parse;
use crate::math::div::export_div;
use crate::math::div_rem::export_div_rem;
use crate::math::neg::export_neg_assign;
use crate::math::powi::export_powi;
use crate::math::round::export_round;
use crate::math::sqrt::export_sqrt;
use crate::math::truncate::export_truncate_assign;
use crate::ptr::Ptr;
use crate::types::conversion_error::ConversionError;
use crate::types::error::Error;
use crate::types::mantissa::{MANTISSA_1, MANTISSA_5};
use crate::types::owner::Owner;
use crate::types::precision::Precision;
use crate::types::rounding::{Round, Rounding, Truncate};
use crate::types::sign::Sign;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter, Write};

/// Arbitrary precision scientific number
///
/// See the [module-level documentation](crate) for more details.
// len == 0 <=> value 0
#[derive(Clone)]
#[must_use]
pub struct Scientific {
  pub(crate) sign: Sign,      // ignored for value 0, can be changed at will
  pub(crate) data: Ptr,       // should never be used for value 0
  pub(crate) len: isize,      // must be 0 for value 0, greater than 0 otherwise
  pub(crate) exponent: isize, // must be 1 for value 0
  pub(crate) owner: Owner,
}

impl Scientific {
  // This constant must not change before 0.5 since scientific-macro depends on it.
  pub const ZERO: Scientific = Scientific {
    sign: Sign::POSITIVE,
    data: Ptr::new_invalid(),
    len: 0,
    exponent: 1, // required for exponent() to work
    owner: Owner::None,
  };
  pub const ONE: Scientific = s_unsafe_static_new(Sign::POSITIVE, &MANTISSA_1, 0);
  pub(crate) const POINT5: Scientific = s_unsafe_static_new(Sign::POSITIVE, &MANTISSA_5, -1);

  #[inline(always)]
  pub fn from_string(mut source: String) -> Result<Scientific, ConversionError> {
    s_parse(source.as_mut_ptr(), source.len(), Owner::new_string(source))
  }

  #[inline(always)]
  #[must_use]
  pub fn to_bytes(&self) -> Vec<u8> {
    s_to_bytes(self)
  }

  #[inline(always)]
  pub fn from_bytes(bytes: &[u8]) -> Result<Scientific, ConversionError> {
    s_from_bytes(bytes).map_err(From::from)
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn as_raw_mantissa(&self) -> &[u8] {
    s_as_raw_mantissa(self)
  }

  #[inline(always)]
  pub fn from_raw_parts(
    negative: bool,
    mantissa: Vec<u8>,
    exponent: isize,
  ) -> Result<Scientific, ConversionError> {
    s_from_raw_parts(negative, mantissa, exponent)
  }

  #[inline(always)]
  pub fn sqrt(&self, precision: Precision) -> Result<Scientific, Error> {
    export_sqrt(self, precision)
  }

  #[inline(always)]
  pub fn abs(&self) -> Scientific {
    let mut result = self.clone();
    result.sign = Sign::POSITIVE;
    result
  }

  #[inline(always)]
  pub fn abs_assign(&mut self) {
    self.sign = Sign::POSITIVE;
  }

  #[inline(always)]
  pub fn neg_assign(&mut self) {
    export_neg_assign(self);
  }

  /// Alias to `self.div_r(rhs, precision, Truncate)`.
  #[inline(always)]
  pub fn div(&self, rhs: &Scientific, precision: Precision) -> Result<Scientific, Error> {
    export_div(self, rhs, precision, Truncate)
  }

  #[inline(always)]
  pub fn div_r<R: Rounding>(
    &self,
    rhs: &Scientific,
    precision: Precision,
    rounding: R,
  ) -> Result<Scientific, Error> {
    export_div(self, rhs, precision, rounding)
  }

  #[inline(always)]
  pub fn div_rem(&self, rhs: &Scientific) -> Result<(Scientific, Scientific), Error> {
    export_div_rem(self, rhs)
  }

  #[inline(always)]
  pub fn rem(&self, rhs: &Scientific) -> Result<Scientific, Error> {
    Ok(export_div_rem(self, rhs)?.1)
  }

  #[inline(always)]
  pub fn truncate_assign(&mut self, precision: Precision) {
    export_truncate_assign(self, precision)
  }

  #[inline(always)]
  pub fn truncate(&self, precision: Precision) -> Scientific {
    let mut r = self.clone();
    export_truncate_assign(&mut r, precision);
    r
  }

  /// round to nearest half away from zero
  ///
  /// Alias to `self.round_r(precision, RoundHalfAwayFromZero)`.
  ///
  /// 0.4, -0.4 => 0.0
  ///
  /// 0.5, 0.6 => 1.0
  ///
  /// -0.5, -0.6 => -1.0
  #[inline(always)]
  pub fn round(&self, precision: Precision) -> Scientific {
    export_round(self, precision, Round::RoundHalfAwayFromZero)
  }

  #[inline(always)]
  pub fn round_r<R: Rounding>(&self, precision: Precision, rounding: R) -> Scientific {
    export_round(self, precision, rounding)
  }

  #[allow(clippy::len_without_is_empty)]
  #[inline(always)]
  pub fn len(&self) -> isize {
    self.len
  }

  #[inline(always)]
  pub fn decimals(&self) -> isize {
    -self.exponent
  }

  #[inline(always)]
  pub fn exponent0(&self) -> isize {
    self.exponent + self.len
  }

  #[inline(always)]
  pub fn exponent1(&self) -> isize {
    self.exponent + self.len - 1
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn exponent(&self) -> isize {
    self.exponent
  }

  #[inline(always)]
  pub fn powi(&self, exponent: usize) -> Scientific {
    export_powi(self, exponent)
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn is_zero(&self) -> bool {
    self.len == 0
  }

  /// Returns true if self has a positive sign, this excludes 0.
  #[inline(always)]
  pub fn is_sign_positive(&self) -> bool {
    self.len > 0 && !self.sign.is_negative()
  }

  /// Returns true if self has a negative sign, this excludes 0.
  #[inline(always)]
  pub fn is_sign_negative(&self) -> bool {
    self.len > 0 && self.sign.is_negative()
  }
}

#[inline(always)]
pub(crate) fn s_mut_make_zero(value: &mut Scientific) {
  value.data.invalidate();
  value.len = 0; // required for is_zero() to work
  value.exponent = 1; // required for exponent() to work
  value.owner = Owner::None;
}

#[inline(always)]
pub(crate) const fn s_unsafe_static_new(
  sign: Sign,
  mantissa: &'static [u8],
  exponent: isize,
) -> Scientific {
  Scientific {
    sign,
    data: Ptr::new_const(mantissa),
    len: mantissa.len() as isize,
    exponent,
    owner: Owner::None,
  }
}

#[cfg(feature = "arc")]
unsafe impl Send for Scientific {}
#[cfg(feature = "arc")]
unsafe impl Sync for Scientific {}

impl Display for Scientific {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      return f.write_char('0');
    }
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    let exp = self.exponent0();
    #[allow(clippy::manual_range_contains)]
    if exp >= -1 && exp <= 0 {
      f.write_char('0')?;
      f.write_char('.')?;
      for _ in exp..0 {
        f.write_char('0')?;
      }
      for i in 0..self.len {
        f.write_char((b'0' + self.data[i] as u8).into())?;
      }
    } else if exp > 1 && exp <= 7 {
      let mid = exp.min(self.len);
      for i in 0..mid {
        f.write_char((b'0' + self.data[i] as u8).into())?;
      }
      for _ in mid..exp {
        f.write_char('0')?;
      }
      if self.len > exp {
        f.write_char('.')?;
        for i in exp..self.len {
          f.write_char((b'0' + self.data[i] as u8).into())?;
        }
      }
    } else {
      f.write_char((b'0' + *self.data as u8).into())?;
      if self.len > 1 {
        f.write_char('.')?;
        for i in 1..self.len {
          f.write_char((b'0' + self.data[i] as u8).into())?;
        }
      }
      if exp != 1 {
        write!(f, "e{}", exp - 1)?;
      }
    }
    Ok(())
  }
}

impl Debug for Scientific {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      return f.write_char('0');
    }
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    for i in 0..self.len {
      f.write_char((b'0' + self.data[i] as u8).into())?;
    }
    if self.exponent != 0 {
      write!(f, "e{}", self.exponent)?;
    }
    Ok(())
  }
}
