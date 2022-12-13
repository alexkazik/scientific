use crate::types::builder::Builder;
use crate::types::precision::Precision;
use crate::types::rounding::Rounding;
use crate::types::sci::Sci;

impl Sci {
  pub(crate) fn round<R: Rounding>(&self, precision: Precision, rounding: R) -> Sci {
    if <R>::is_truncate() {
      let mut result = self.clone();
      result.truncate_assign(precision);
      result
    } else {
      let len = match precision {
        Precision::Digits(digits) => digits,
        Precision::Decimals(decimals) => self.exponent0() + decimals,
      };
      if len <= 0 {
        Sci::ZERO
      } else if len >= self.len {
        // more precision requested as available: just return the number
        self.clone()
      } else if self.data[len] == 0
        || !rounding.round_away_from_zero(
          self.sign.is_negative(),
          self.data[len - 1],
          self.data[len],
        )
      {
        // the digit after the cutoff is zero and thus there is no rounding
        // or the rounding would result in no change
        let mut result = self.clone();
        result.truncate_assign(precision);
        result
      } else {
        let (result, result_ptr) =
          Builder::new(self.sign, len + 2, self.exponent + self.len - (len + 1));
        self.data.copy_to_nonoverlapping(len + 1, result_ptr, 1);
        result.round(Precision::Digits(len), rounding)
      }
    }
  }
}
