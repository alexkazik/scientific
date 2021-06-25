use crate::Scientific;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

impl Shl<isize> for &Scientific {
  type Output = Scientific;

  #[inline]
  fn shl(self, rhs: isize) -> Self::Output {
    let mut result = self.clone();
    result <<= rhs;
    result
  }
}

impl ShlAssign<isize> for Scientific {
  #[inline(always)]
  fn shl_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent += rhs;
    }
  }
}

impl Shr<isize> for &Scientific {
  type Output = Scientific;

  #[inline]
  fn shr(self, rhs: isize) -> Self::Output {
    let mut result = self.clone();
    result >>= rhs;
    result
  }
}

impl ShrAssign<isize> for Scientific {
  #[inline(always)]
  fn shr_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent -= rhs;
    }
  }
}
