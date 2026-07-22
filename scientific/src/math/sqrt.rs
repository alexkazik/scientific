use crate::types::error::Error;
use crate::types::limited::{Exponent, Length, Unchecked};
use crate::types::precision::Precision;
use crate::types::rounding_mode::RoundingMode;
use crate::types::rounding_rpsp::RPSP;
use crate::types::sci::Sci;
use crate::types::sign::Sign;
use core::cmp::Ordering;

impl Sci {
  pub(crate) fn sqrt(&self, precision: Precision, use_rpsp: bool) -> Result<Sci, Error> {
    if self.is_zero() {
      Ok(Sci::ZERO)
    } else if self.sign.is_negative() {
      Err(Error::NumberIsNegative)
    } else if self.compare::<false>(&Sci::ONE) == Ordering::Equal {
      if Sci::ONE.precision_len(precision) >= 1 {
        Ok(Sci::ONE)
      } else if use_rpsp {
        let mut result = Sci::ONE;
        result.round_assign(precision, RoundingMode::RPSP(RPSP));
        Ok(result)
      } else {
        Ok(Sci::ZERO)
      }
    } else {
      nz_sqrt(self, precision, use_rpsp)
    }
  }
}

// Babylonian method
fn nz_sqrt(value: &Sci, precision: Precision, use_rpsp: bool) -> Result<Sci, Error> {
  let mut guess = value.clone();
  guess.shr_assign(value.exponent1() / 2 - 1);
  limit(&mut guess, precision);

  #[cfg(all(feature = "debug", feature = "std"))]
  {
    let f_guess = guess.to_f64();
    let f_value = value.to_f64();
    let f_sqrt_value = f_value.sqrt();
    if f_guess.is_finite() && f_value.is_finite() && f_sqrt_value.is_finite() {
      assert!(
        f_guess > f_sqrt_value,
        "{}",
        format!("initial guess {f_guess} should be larger than sqrt({f_value})={f_sqrt_value}")
      );
    }
  }

  loop {
    let mut next_guess = Sci::POINT5.mul(
      &(guess.add(&value.div(
        &guess,
        Precision::Digits(limit_div(value, &guess, precision)),
        false,
      )?)),
    );
    limit(&mut next_guess, precision);
    if guess.compare::<false>(&next_guess) != Ordering::Greater {
      break;
    }
    guess = next_guess;
  }

  if use_rpsp {
    if value.compare::<false>(&guess.mul(&guess)) == Ordering::Greater {
      // value is bigger than result*result => increase the result by a tiny bit, if necessary
      if guess.len >= guess.precision_len(precision) {
        // there is enough precision to simply adapt the last digit, if necessary
        // (may be behind the actual limit, will be fixed with round_assign afterwards)
        if guess.data[guess.len - 1] == 0 || guess.data[guess.len - 1] == 5 {
          guess.data[guess.len - 1] += 1;
        }
      } else {
        // there is not enough space to add it in place, add a 1 at the actual limit
        guess = guess.add(&Sci::one(
          Sign::POSITIVE,
          match precision {
            Precision::Digits(d) => Exponent::new(guess.exponent0().saturating_sub(d)),
            Precision::Decimals(d) => Exponent::new(-d),
          },
        ));
      }
    }
    guess.round_assign(precision, RoundingMode::RPSP(RPSP));
  } else {
    guess.truncate_assign(precision);
  }

  // fix limit (there may be trailing zeroes due to how limit works)
  let mut exponent = guess.exponent;
  while guess.data[guess.len - 1] == 0 {
    guess.len -= Unchecked(1);
    exponent += Unchecked(1);
  }
  guess.exponent = Exponent::new(exponent);

  Ok(guess)
}

#[inline]
fn limit(value: &mut Sci, precision: Precision) {
  let len = Length::new(value.precision_len(precision).max(1));
  if value.len > len {
    value.exponent = Exponent::new(value.exponent + value.len - len);
    value.len = len;
  }
}

#[inline]
fn limit_div(lhs: &Sci, rhs: &Sci, precision: Precision) -> isize {
  match precision {
    Precision::Digits(d) => d,
    Precision::Decimals(d) => (lhs.exponent0() - rhs.exponent0() + 1).saturating_add(d),
  }
  .max(1)
}
