use crate::types::builder::Builder;
use crate::types::precision::Precision;
use crate::types::scientific::Scientific;

pub(crate) fn export_round(value: &Scientific, precision: Precision) -> Scientific {
  let len = match precision {
    Precision::Digits(digits) => digits,
    Precision::Decimals(decimals) => value.exponent0() + decimals,
  };
  if len <= 0 {
    Scientific::ZERO
  } else if len < value.len && value.data[len] >= 5 {
    // actually round to nearest away from zero
    let (result, mut p) = Builder::new(value.sign, len + 1, value.exponent + value.len - len);
    value.data.copy_to_nonoverlapping(len, p, 1);
    let mut pos = len;
    let mut val = p[pos] + 1;
    while val == 10 {
      p[pos] = 0;
      pos -= 1;
      val = p[pos] + 1;
    }
    p[pos] = val;
    result.finish()
  } else {
    // call truncate since it's not rounded up, this will return the same result (without copying the mantissa)
    value.truncate(Precision::Digits(len))
  }
}
