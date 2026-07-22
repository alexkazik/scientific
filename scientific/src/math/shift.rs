use crate::types::limited::Exponent;
use crate::types::sci::Sci;

impl Sci {
  #[inline]
  pub(crate) fn shl_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent = Exponent::new(self.exponent.saturating_add(rhs));
    }
  }

  #[inline]
  pub(crate) fn shr_assign(&mut self, rhs: isize) {
    if !self.is_zero() {
      self.exponent = Exponent::new(self.exponent.saturating_sub(rhs));
    }
  }
}
