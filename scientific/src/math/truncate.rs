use crate::types::limited::{Exponent, Length, Unchecked};
use crate::types::precision::Precision;
use crate::types::sci::Sci;

impl Sci {
  pub(crate) fn truncate_assign(&mut self, precision: Precision) {
    let len = self.precision_len(precision);
    if self.len > len {
      // Safety: len is less than self.len
      let mut len = Length::from_isize_unchecked(len);
      let mut exponent = self.exponent + self.len - len;

      // remove trailing zeroes
      while len > 0 && self.data[len - 1] == 0 {
        len -= Unchecked(1);
        // Safety: exponent uses only 3/8 and has at most 1/8 new additions, thus always fits
        exponent += Unchecked(1);
      }

      if len <= 0 {
        self.assign_zero();
      } else {
        self.len = len;
        self.exponent = Exponent::new(exponent);
      }
    }
  }
}
