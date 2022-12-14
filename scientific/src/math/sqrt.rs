use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::sci::Sci;
use core::cmp::Ordering;

impl Sci {
  pub(crate) fn sqrt(&self, precision: Precision) -> Result<Sci, Error> {
    if self.is_zero() {
      Ok(Sci::ZERO)
    } else if self.sign.is_negative() {
      Err(Error::NumberIsNegative)
    } else {
      match self.compare::<false>(&Sci::ONE) {
        Ordering::Less => nz_sqrt(self, precision, (self.exponent0() - 1) / 2),
        Ordering::Equal => {
          if Sci::ONE.precision_len(precision) >= 1 {
            Ok(Sci::ONE)
          } else {
            Ok(Sci::ZERO)
          }
        }
        Ordering::Greater => nz_sqrt(self, precision, self.exponent0() / 2),
      }
    }
  }
}

// Babylonian method
fn nz_sqrt(value: &Sci, precision: Precision, guess_exponent_adapt: isize) -> Result<Sci, Error> {
  let mut guess = value.clone();
  guess.shr_assign(guess_exponent_adapt);
  limit(&mut guess, precision);
  loop {
    let mut next_guess = Sci::POINT5.mul(
      &(guess.add(&value.div(
        &guess,
        Precision::Digits(limit_div(value, &guess, precision)),
      )?)),
    );
    limit(&mut next_guess, precision);
    if guess.compare::<false>(&next_guess) == Ordering::Equal {
      guess.truncate_assign(precision);
      return Ok(guess);
    }
    guess = next_guess;
  }
}

#[inline]
fn limit(value: &mut Sci, precision: Precision) {
  let len = value.precision_len(precision).max(1);
  // do truncate
  if value.len > len {
    value.exponent += value.len - len;
    value.len = len;
  }
}

#[inline]
fn limit_div(lhs: &Sci, rhs: &Sci, precision: Precision) -> isize {
  match precision {
    Precision::Digits(d) => d,
    Precision::Decimals(d) => lhs.exponent0() - rhs.exponent0() + d + 1,
  }
  .max(2)
}
