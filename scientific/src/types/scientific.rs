use crate::types::conversion_error::ConversionError;
use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::rounding::Rounding;
use crate::types::sci::Sci;
use crate::types::sign::Sign;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Add, Mul, Neg, Shl, ShlAssign, Shr, ShrAssign, Sub};
use core::str::FromStr;

/// Arbitrary precision scientific number
///
/// See the [module-level documentation](crate) for more details.
#[derive(Clone)]
#[must_use]
pub struct Scientific {
  pub(crate) inner: Sci,
}

impl Scientific {
  pub const ZERO: Scientific = Scientific { inner: Sci::ZERO };
  pub const ONE: Scientific = Scientific { inner: Sci::ONE };

  #[inline(always)]
  pub fn from_string(source: String) -> Result<Scientific, ConversionError> {
    Ok(Scientific {
      inner: Sci::from_string(source)?,
    })
  }

  #[inline(always)]
  #[must_use]
  pub fn to_bytes(&self) -> Vec<u8> {
    self.inner.to_bytes()
  }

  #[inline(always)]
  pub fn from_bytes(bytes: &[u8]) -> Result<Scientific, ConversionError> {
    Ok(Scientific {
      inner: Sci::from_bytes(bytes)?,
    })
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  #[must_use]
  pub fn as_raw_mantissa(&self) -> &[u8] {
    self.inner.as_raw_mantissa()
  }

  #[inline(always)]
  pub fn from_raw_parts(
    negative: bool,
    mantissa: Vec<u8>,
    exponent: isize,
  ) -> Result<Scientific, ConversionError> {
    Ok(Scientific {
      inner: Sci::from_raw_parts(negative, mantissa, exponent)?,
    })
  }

  #[inline(always)]
  pub fn sqrt_truncate(&self, precision: Precision) -> Result<Scientific, Error> {
    Ok(Scientific {
      inner: self.inner.sqrt(precision)?,
    })
  }

  #[inline(always)]
  pub fn abs(&self) -> Scientific {
    let mut result = self.clone();
    result.inner.sign = Sign::POSITIVE;
    result
  }

  #[inline(always)]
  pub fn abs_assign(&mut self) {
    self.inner.sign = Sign::POSITIVE;
  }

  #[inline(always)]
  pub fn neg_assign(&mut self) {
    self.inner.neg_assign();
  }

  #[inline(always)]
  pub fn div_truncate(&self, rhs: &Scientific, precision: Precision) -> Result<Scientific, Error> {
    Ok(Scientific {
      inner: self.inner.div(&rhs.inner, precision)?,
    })
  }

  #[inline(always)]
  pub fn div_rem(&self, rhs: &Scientific) -> Result<(Scientific, Scientific), Error> {
    let (d, r) = self.inner.div_rem(&rhs.inner)?;
    Ok((Scientific { inner: d }, Scientific { inner: r }))
  }

  #[inline(always)]
  pub fn truncate_assign(&mut self, precision: Precision) {
    self.inner.truncate_assign(precision);
  }

  #[inline(always)]
  pub fn truncate(&self, precision: Precision) -> Scientific {
    let mut r = self.clone();
    r.inner.truncate_assign(precision);
    r
  }

  #[inline(always)]
  pub fn round<R: Rounding>(&self, precision: Precision, rounding: R) -> Scientific {
    Scientific {
      inner: self.inner.round(precision, rounding),
    }
  }

  #[allow(clippy::len_without_is_empty)]
  #[inline(always)]
  #[must_use]
  pub fn len(&self) -> isize {
    self.inner.len
  }

  #[inline(always)]
  #[must_use]
  pub fn decimals(&self) -> isize {
    -self.inner.exponent
  }

  #[inline(always)]
  #[must_use]
  pub fn exponent0(&self) -> isize {
    self.inner.exponent0()
  }

  #[inline(always)]
  #[must_use]
  pub fn exponent1(&self) -> isize {
    self.inner.exponent1()
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  #[must_use]
  pub fn exponent(&self) -> isize {
    self.inner.exponent
  }

  #[inline(always)]
  pub fn powi(&self, exponent: usize) -> Scientific {
    Scientific {
      inner: self.inner.powi(exponent),
    }
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  #[must_use]
  pub fn is_zero(&self) -> bool {
    self.inner.is_zero()
  }

  /// Returns true if self has a positive sign, this excludes 0.
  #[inline(always)]
  #[must_use]
  pub fn is_sign_positive(&self) -> bool {
    self.inner.len > 0 && !self.inner.sign.is_negative()
  }

  /// Returns true if self has a negative sign, this excludes 0.
  #[inline(always)]
  #[must_use]
  pub fn is_sign_negative(&self) -> bool {
    self.inner.len > 0 && self.inner.sign.is_negative()
  }
}

#[cfg(feature = "arc")]
#[cfg_attr(docsrs, doc(cfg(feature = "arc")))]
unsafe impl Send for Scientific {}
#[cfg(feature = "arc")]
#[cfg_attr(docsrs, doc(cfg(feature = "arc")))]
unsafe impl Sync for Scientific {}

impl PartialEq for Scientific {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl Eq for Scientific {}

impl PartialOrd for Scientific {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Scientific {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    self.inner.compare::<true>(&other.inner)
  }
}

impl Add for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    Scientific {
      inner: self.inner.add(&rhs.inner),
    }
  }
}

impl Mul for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn mul(self, rhs: Self) -> Self::Output {
    Scientific {
      inner: self.inner.mul(&rhs.inner),
    }
  }
}

impl Neg for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn neg(self) -> Self::Output {
    let mut result = self.clone();
    result.inner.neg_assign();
    result
  }
}

impl Shl<isize> for &Scientific {
  type Output = Scientific;

  #[inline]
  fn shl(self, rhs: isize) -> Self::Output {
    let mut result = self.clone();
    result.inner.shl_assign(rhs);
    result
  }
}

impl ShlAssign<isize> for Scientific {
  #[inline(always)]
  fn shl_assign(&mut self, rhs: isize) {
    self.inner.shl_assign(rhs);
  }
}

impl Shr<isize> for &Scientific {
  type Output = Scientific;

  #[inline]
  fn shr(self, rhs: isize) -> Self::Output {
    let mut result = self.clone();
    result.inner.shr_assign(rhs);
    result
  }
}

impl ShrAssign<isize> for Scientific {
  #[inline(always)]
  fn shr_assign(&mut self, rhs: isize) {
    self.inner.shr_assign(rhs);
  }
}

impl Sub for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    Scientific {
      inner: self.inner.sub(&rhs.inner),
    }
  }
}

impl FromStr for Scientific {
  type Err = ConversionError;

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Scientific {
      inner: Sci::from_string(s.to_string())?,
    })
  }
}

impl Debug for Scientific {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    self.inner.debug(f)
  }
}

impl Display for Scientific {
  #[inline(always)]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    self.inner.display(f)
  }
}

impl Hash for &Scientific {
  #[inline(always)]
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.inner.hash(state);
  }
}
