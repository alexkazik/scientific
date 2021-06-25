use crate::types::precision::Precision;
use crate::types::scientific::Scientific;
use crate::util::zeroes::s_mut_make_zero;

pub(crate) fn export_truncate_assign(value: &mut Scientific, precision: Precision) {
  let len = match precision {
    Precision::Digits(digits) => digits,
    Precision::Decimals(decimals) => value.exponent0() + decimals,
  };
  if value.len > len {
    value.exponent += value.len - len;
    value.len = len; // len may be zero or negative

    // remove trailing zeroes
    while value.len > 0 && value.data[value.len - 1] == 0 {
      value.len -= 1;
      value.exponent += 1;
    }

    if value.len <= 0 {
      s_mut_make_zero(value);
    }
  }
}
