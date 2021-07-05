use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use core::hash::{Hash, Hasher};

impl Hash for &Scientific {
  fn hash<H: Hasher>(&self, state: &mut H) {
    if !self.is_zero() {
      (self.sign == Sign::Negative).hash(state);
      Hash::hash_slice(self.data.as_slice(self.len), state);
      self.exponent.hash(state);
    }
  }
}
