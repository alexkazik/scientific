use crate::math::add::nz_add;
use crate::math::compare::s_compare;
use crate::ptr::Ptr;
use crate::types::builder::Builder;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use core::cmp::Ordering;
use core::ops::Sub;

impl Sub for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    export_sub(self, rhs)
  }
}

fn export_sub(lhs: &Scientific, rhs: &Scientific) -> Scientific {
  if rhs.is_zero() {
    lhs.clone()
  } else if lhs.is_zero() {
    -rhs
  } else if lhs.sign != rhs.sign {
    nz_add(lhs, rhs, lhs.sign)
  } else {
    match s_compare::<false>(lhs, rhs) {
      Ordering::Less => nz_sub(rhs, lhs, !rhs.sign),
      Ordering::Equal => Scientific::ZERO,
      Ordering::Greater => nz_sub(lhs, rhs, lhs.sign),
    }
  }
}

pub(crate) fn nz_sub(lhs: &Scientific, rhs: &Scientific, sign: Sign) -> Scientific {
  let min_exponent = lhs.exponent.min(rhs.exponent);
  let result_len = lhs.exponent0() - rhs.exponent;

  let (result, result_ptr) = Builder::new(sign, result_len.max(lhs.len), min_exponent);

  lhs.data.copy_to_nonoverlapping(lhs.len, result_ptr, 0);
  p_sub(result_ptr, result_len, rhs);
  result.finish()
}

// Subtract two mantissa (the exponent and sign is ignored)
// The first number must be greater or equal to the second
#[inline]
pub(crate) fn p_sub(lhs_ptr: Ptr, lhs_len: isize, rhs: &Scientific) {
  let mut lhs_cur = lhs_ptr.offset(lhs_len);
  let mut rhs_cur = rhs.data.offset(rhs.len);
  let mut carry = 0;
  while rhs.data < rhs_cur {
    lhs_cur.dec();
    rhs_cur.dec();
    let mut value = *lhs_cur - *rhs_cur - carry;
    if value < 0 {
      value += 10;
      carry = 1;
    } else {
      carry = 0;
    }
    *lhs_cur = value;
  }
  while carry != 0 {
    lhs_cur.dec();
    let mut value = *lhs_cur - carry;
    if value < 0 {
      value += 10;
      carry = 1;
    } else {
      carry = 0;
    }
    *lhs_cur = value;
  }
}
