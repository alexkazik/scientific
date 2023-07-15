use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::rounding_mode::RoundingMode;
use crate::types::rounding_rpsp::RPSP;
use crate::types::sci::Sci;
use core::cmp::Ordering;

impl Sci {
  pub(crate) fn sqrt(&self, precision: Precision, use_rpsp: bool) -> Result<Sci, Error> {
    if self.is_zero() {
      Ok(Sci::ZERO)
    } else if self.sign.is_negative() {
      Err(Error::NumberIsNegative)
    } else {
      match self.compare::<false>(&Sci::ONE) {
        Ordering::Less => nz_sqrt(self, precision, (self.exponent0() - 1) / 2, use_rpsp),
        Ordering::Equal => {
          if Sci::ONE.precision_len(precision) >= 1 {
            Ok(Sci::ONE)
          } else if use_rpsp {
            let mut result = Sci::ONE;
            result.round_assign(precision, RoundingMode::RPSP(RPSP));
            Ok(result)
          } else {
            Ok(Sci::ZERO)
          }
        }
        Ordering::Greater => nz_sqrt(self, precision, self.exponent0() / 2, use_rpsp),
      }
    }
  }
}

// Babylonian method
fn nz_sqrt(
  value: &Sci,
  precision: Precision,
  guess_exponent_adapt: isize,
  use_rpsp: bool,
) -> Result<Sci, Error> {
  let mut guess = value.clone();
  guess.shr_assign(guess_exponent_adapt);
  limit(&mut guess, precision, use_rpsp);

  loop {
    let mut next_guess = Sci::POINT5.mul(
      &(guess.add(&value.div(
        &guess,
        Precision::Digits(limit_div(value, &guess, precision)),
        use_rpsp,
      )?)),
    );
    limit(&mut next_guess, precision, use_rpsp);
    if guess.compare::<false>(&next_guess) == Ordering::Equal {
      break;
    }
    guess = next_guess;
  }

  if use_rpsp {
    guess.round_assign(precision, RoundingMode::RPSP(RPSP));
  } else {
    guess.truncate_assign(precision);
  }

  Ok(guess)
}

#[inline]
fn limit(value: &mut Sci, precision: Precision, use_rpsp: bool) {
  let len = value.precision_len(precision).max(1);
  if use_rpsp {
    value.round_assign(Precision::Digits(len), RoundingMode::RPSP(RPSP));
  } else {
    // do truncate
    if value.len > len {
      value.exponent += value.len - len;
      value.len = len;
    }
  }
}

#[inline]
fn limit_div(lhs: &Sci, rhs: &Sci, precision: Precision) -> isize {
  match precision {
    Precision::Digits(d) => d,
    Precision::Decimals(d) => lhs.exponent0() - rhs.exponent0() + d + 1,
  }
  .max(1)
    + 1
}
