use crate::types::sci::Sci;

impl Sci {
  #[inline]
  pub(crate) fn shl_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent = (self.exponent + rhs).into();
    }
  }

  #[inline]
  pub(crate) fn shr_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent = (self.exponent - rhs).into();
    }
  }
}
