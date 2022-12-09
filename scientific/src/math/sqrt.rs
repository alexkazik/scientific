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
        Ordering::Equal => Ok(Sci::ONE),
        Ordering::Greater => nz_sqrt(self, precision, self.exponent0() / 2),
      }
    }
  }
}

// Babylonian method
fn nz_sqrt(value: &Sci, precision: Precision, guess_exponent_adapt: isize) -> Result<Sci, Error> {
  let mut guess = value.clone();
  guess.shr_assign(guess_exponent_adapt);
  guess.truncate_assign(precision);
  loop {
    let mut next_guess = Sci::POINT5.mul(&(guess.add(&value.div(&guess, precision)?)));
    next_guess.truncate_assign(precision);

    if guess.compare::<false>(&next_guess) == Ordering::Equal {
      return Ok(guess);
    }
    guess = next_guess;
  }
}
