use crate::math::compare::nz_compare_mantissa;
use crate::math::sub::p_sub;
use crate::ptr::Ptr;
use crate::types::builder::Builder;
use crate::types::error::Error;
use crate::types::mantissa::MANTISSA_1;
use crate::types::owner::Owner;
use crate::types::precision::Precision;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use crate::types::trimmer::Trimmer;
use core::cmp::Ordering;

#[inline(always)]
fn div_results_in_zero(lhs: &Scientific, rhs: &Scientific, precision: Precision) -> bool {
  match precision {
    Precision::Digits(digits) => digits <= 0,
    Precision::Decimals(decimals) => lhs.exponent0() - rhs.exponent0() + decimals < 0,
  }
}

pub(crate) fn export_div(
  lhs: &Scientific,
  rhs: &Scientific,
  precision: Precision,
) -> Result<Scientific, Error> {
  if rhs.is_zero() {
    Err(Error::DivisionByZero)
  } else if lhs.is_zero() || div_results_in_zero(lhs, rhs, precision) {
    Ok(Scientific::ZERO)
  } else if rhs.len == 1 && *rhs.data == 1 {
    let mut r = lhs.clone();
    if rhs.sign == Sign::Negative {
      r.neg_assign();
    }
    r >>= rhs.exponent;
    r.truncate_assign(precision);
    Ok(r)
  } else if lhs.len == rhs.len && nz_compare_mantissa::<false>(lhs, rhs) == Ordering::Equal {
    let exponent = lhs.exponent0() - rhs.exponent0();
    if let Precision::Decimals(decimals) = precision {
      if -decimals > exponent {
        return Ok(Scientific::ZERO);
      }
    }
    Ok(Scientific {
      sign: lhs.sign ^ rhs.sign,
      data: Ptr::new_const(&MANTISSA_1),
      len: 1,
      exponent,
      owner: Owner::None,
    })
  } else {
    let extra_digits = match precision {
      Precision::Digits(digits) => (digits - (lhs.len - rhs.len)),
      Precision::Decimals(decimals) => (lhs.exponent - rhs.exponent + decimals),
    };
    Ok(nz_div(lhs, rhs, extra_digits, precision))
  }
}

#[inline(always)]
fn nz_div(
  lhs: &Scientific,
  rhs: &Scientific,
  extra_digits: isize,
  precision: Precision,
) -> Scientific {
  // Notice: extra_digits can be negative!
  // n1.len + decimals is guaranteed to be >= n2.len
  #[cfg(feature = "debug")]
  assert!(lhs.len + extra_digits >= rhs.len);

  let mut tmp = vec![0; (lhs.len + extra_digits) as usize];
  let mut tmp_ptr = Ptr::new_mut(tmp.as_mut_ptr(), lhs.len + extra_digits);
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
      p_sub(tmp_ptr, tmp_len, rhs);
      p_trim(&mut tmp_ptr, &mut tmp_len);
      j += 1;
    }
    *result_ptr = j;
    result_ptr.inc();
  }

  result.finish(Trimmer::Trim(precision))
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
fn p_ge(mut lhs_ptr: Ptr, lhs_len: isize, rhs: &Scientific) -> bool {
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
