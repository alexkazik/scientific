use crate::math::compare::s_compare;
use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::scientific::Scientific;
use core::cmp::Ordering;

pub(crate) fn export_sqrt(value: &Scientific, precision: Precision) -> Result<Scientific, Error> {
  if value.is_sign_negative() {
    Err(Error::NumberIsNegative)
  } else if value.is_zero() {
    Ok(Scientific::ZERO)
  } else {
    match s_compare::<false>(value, &Scientific::ONE) {
      Ordering::Less => nz_sqrt(value, precision, (value.exponent0() - 1) / 2),
      Ordering::Equal => Ok(Scientific::ONE),
      Ordering::Greater => nz_sqrt(value, precision, value.exponent0() / 2),
    }
  }
}

// Babylonian method
fn nz_sqrt(
  value: &Scientific,
  precision: Precision,
  guess_exponent_adapt: isize,
) -> Result<Scientific, Error> {
  let mut guess = value >> guess_exponent_adapt;
  guess.truncate_assign(precision);
  loop {
    let mut next_guess = &Scientific::POINT5 * &(&guess + &value.div(&guess, precision)?);
    next_guess.truncate_assign(precision);

    if guess == next_guess {
      return Ok(guess);
    }
    guess = next_guess;
  }
}
