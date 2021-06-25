use crate::types::scientific::Scientific;

pub(crate) fn export_powi(base: &Scientific, exponent: usize) -> Scientific {
  if exponent == 0 {
    Scientific::ONE
  } else if base.is_zero() {
    Scientific::ZERO
  } else {
    nz_powi(base, exponent)
  }
}

#[inline(always)]
fn nz_powi(base: &Scientific, mut exponent: usize) -> Scientific {
  // base is not zero, exponent is greater than zero
  let mut power = base.clone();
  while exponent & 1 == 0 {
    power = &power * &power;
    exponent >>= 1;
  }
  let mut result = power.clone();
  exponent >>= 1;

  while exponent > 0 {
    power = &power * &power;
    if exponent & 1 != 0 {
      result = &result * &power;
    }
    exponent >>= 1;
  }
  result
}
