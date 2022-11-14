use crate::types::sci::Sci;
use core::hash::{Hash, Hasher};

impl Sci {
  pub(crate) fn hash<H: Hasher>(&self, state: &mut H) {
    if !self.is_zero() {
      (self.sign.is_negative()).hash(state);
      Hash::hash_slice(self.data.as_slice(self.len), state);
      self.exponent.hash(state);
    }
  }
}
