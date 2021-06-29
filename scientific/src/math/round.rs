use crate::types::builder::Builder;
use crate::types::precision::Precision;
use crate::types::precision::Precision::Digits;
use crate::types::rounding::Rounding;
use crate::types::scientific::Scientific;

pub(crate) fn export_round<R: Rounding>(
  value: &Scientific,
  precision: Precision,
  rounding: R,
) -> Scientific {
  if <R>::is_truncate() {
    value.truncate(precision)
  } else {
    let len = match precision {
      Precision::Digits(digits) => digits,
      Precision::Decimals(decimals) => value.exponent0() + decimals,
    };
    if len <= 0 {
      Scientific::ZERO
    } else if len >= value.len {
      // more precision requested as available: just return the number
      value.clone()
    } else if value.data[len] == 0
      || !rounding.round_away_from_zero(
        value.sign.is_negative(),
        value.data[len - 1],
        value.data[len],
      )
    {
      // the digit after the cutoff is zero and thus there is no rounding
      // or the rounding would result in no change
      value.truncate(Precision::Digits(len))
    } else {
      let (result, result_ptr) =
        Builder::new(value.sign, len + 2, value.exponent + value.len - (len + 1));
      value.data.copy_to_nonoverlapping(len + 1, result_ptr, 1);
      result.round(Digits(len), rounding)
    }
  }
}
