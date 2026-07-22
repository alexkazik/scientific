use crate::types::limited::{Exponent, Length};
use crate::types::precision::Precision;
use crate::types::sci::Sci;

impl Sci {
  pub(crate) fn truncate_assign(&mut self, precision: Precision) {
    let mut len = self.precision_len(precision);
    if self.len > len {
      let mut exponent = self.exponent + self.len - len;

      // remove trailing zeroes
      while len > 0 && self.data[len - 1] == 0 {
        len -= 1;
        exponent += 1;
      }

      if len <= 0 {
        self.assign_zero();
      } else {
        self.len = Length::new(len);
        self.exponent = Exponent::new(exponent);
      }
    }
  }
}
