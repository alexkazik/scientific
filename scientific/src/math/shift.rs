use crate::types::sci::Sci;

impl Sci {
  #[inline(always)]
  pub(crate) fn shl_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent += rhs;
    }
  }

  #[inline(always)]
  pub(crate) fn shr_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent -= rhs;
    }
  }
}
