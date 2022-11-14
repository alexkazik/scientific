use crate::types::sci::Sci;

impl Sci {
  #[inline(always)]
  pub(crate) fn neg_assign(&mut self) {
    self.sign = !self.sign;
  }
}
