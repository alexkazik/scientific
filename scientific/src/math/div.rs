use crate::ptr::Ptr;
use crate::types::builder::Builder;
use crate::types::error::Error;
use crate::types::mantissa::MANTISSA_1;
use crate::types::precision::Precision;
use crate::types::sci::Sci;
use core::cmp::Ordering;

#[inline(always)]
fn div_results_in_zero(lhs: &Sci, rhs: &Sci, precision: Precision) -> bool {
  match precision {
    Precision::Digits(digits) => digits <= 0,
    Precision::Decimals(decimals) => lhs.exponent0() - rhs.exponent0() + decimals < 0,
  }
}

impl Sci {
  pub(crate) fn div(&self, rhs: &Sci, precision: Precision) -> Result<Sci, Error> {
    if rhs.is_zero() {
      Err(Error::DivisionByZero)
    } else if self.is_zero() || div_results_in_zero(self, rhs, precision) {
      Ok(Sci::ZERO)
    } else if rhs.len == 1 && *rhs.data == 1 {
      let mut r = self.clone();
      r.shr_assign(rhs.exponent);
      r.truncate_assign(precision);
      if rhs.sign.is_negative() {
        r.neg_assign();
      }
      Ok(r)
    } else if self.len == rhs.len && self.nz_compare_mantissa::<false>(rhs) == Ordering::Equal {
      let exponent = self.exponent0() - rhs.exponent0();
      if let Precision::Decimals(decimals) = precision {
        if -decimals > exponent {
          return Ok(Sci::ZERO);
        }
      }
      Ok(Sci::nz_unsafe_static_new(
        self.sign ^ rhs.sign,
        &MANTISSA_1,
        exponent,
      ))
    } else {
      let extra_digits = match precision {
        Precision::Digits(digits) => digits - (self.len - rhs.len),
        Precision::Decimals(decimals) => self.exponent - rhs.exponent + decimals,
      };
      Ok(nz_div(self, rhs, extra_digits, precision))
    }
  }
}

#[inline(always)]
fn nz_div(lhs: &Sci, rhs: &Sci, extra_digits: isize, precision: Precision) -> Sci {
  // Notice: extra_digits can be negative!
  // n1.len + decimals is guaranteed to be >= n2.len
  #[cfg(feature = "debug")]
  assert!(lhs.len + extra_digits >= rhs.len);

  let mut tmp = vec![0; (lhs.len + extra_digits) as usize];
  let mut tmp_ptr = Ptr::new_mut(tmp.as_mut_slice());
  lhs
    .data
    .copy_to_nonoverlapping(lhs.len + extra_digits.min(0), tmp_ptr, 0);
  let mut tmp_len = 0;
  let (result, mut result_ptr) = Builder::new(
    lhs.sign ^ rhs.sign,
    lhs.len + extra_digits,
    lhs.exponent - rhs.exponent - extra_digits,
  );
  let result_end = result_ptr.offset(lhs.len + extra_digits);
  while result_ptr < result_end {
    tmp_len += 1;
    p_trim(&mut tmp_ptr, &mut tmp_len);
    let mut j = 0;
    while p_ge(tmp_ptr, tmp_len, rhs) {
      Sci::p_sub(tmp_ptr, tmp_len, rhs);
      p_trim(&mut tmp_ptr, &mut tmp_len);
      j += 1;
    }
    *result_ptr = j;
    result_ptr.inc();
  }

  let mut result = result.finish();
  result.truncate_assign(precision);
  result
}

// Remove leading zeroes, this is important because p_ge assumes that there are no leading zeroes
#[inline(always)]
fn p_trim(value_ptr: &mut Ptr, value_len: &mut isize) {
  while *value_len > 0 && **value_ptr == 0 {
    value_ptr.inc();
    *value_len -= 1;
  }
}

// Compare two mantissa (the exponent and sign is ignored)
#[inline(always)]
fn p_ge(mut lhs_ptr: Ptr, lhs_len: isize, rhs: &Sci) -> bool {
  if lhs_len != rhs.len {
    lhs_len >= rhs.len
  } else {
    let mut rhs_ptr = rhs.data;
    let rhs_end = rhs.data.offset(rhs.len);
    while rhs_ptr < rhs_end {
      if *lhs_ptr != *rhs_ptr {
        return *lhs_ptr >= *rhs_ptr;
      }
      lhs_ptr.inc();
      rhs_ptr.inc();
    }
    true
  }
}
