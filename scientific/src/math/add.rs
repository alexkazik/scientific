use crate::math::compare::s_compare;
use crate::math::sub::nz_sub;
use crate::types::builder::Builder;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use core::cmp::Ordering;
use core::mem::swap;
use core::ops::Add;

impl Add for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn add(self, rhs: Self) -> Self::Output {
    export_add(self, rhs)
  }
}

fn export_add(lhs: &Scientific, rhs: &Scientific) -> Scientific {
  if lhs.is_zero() {
    return rhs.clone();
  } else if rhs.is_zero() {
    return lhs.clone();
  }
  if lhs.sign == rhs.sign {
    nz_add(lhs, rhs, lhs.sign)
  } else {
    match s_compare::<false>(lhs, rhs) {
      Ordering::Less => nz_sub(rhs, lhs, rhs.sign),
      Ordering::Equal => Scientific::ZERO,
      Ordering::Greater => nz_sub(lhs, rhs, lhs.sign),
    }
  }
}

pub(crate) fn nz_add<'a>(
  mut lhs: &'a Scientific,
  mut rhs: &'a Scientific,
  sign: Sign,
) -> Scientific {
  let mut lhs_exponent0 = lhs.exponent0();
  let mut rhs_exponent0 = rhs.exponent0();

  if lhs_exponent0 < rhs_exponent0 {
    swap(&mut lhs, &mut rhs);
    swap(&mut lhs_exponent0, &mut rhs_exponent0);
  }

  let min_exponent = lhs.exponent.min(rhs.exponent);
  let result_len = 1 + (lhs_exponent0.max(rhs_exponent0) - min_exponent);

  let (result, mut result_ptr) = Builder::new(sign, result_len, min_exponent);

  let mut carry = 0;

  lhs.data.copy_to_nonoverlapping(lhs.len, result_ptr, 1);
  result_ptr.mut_offset(result_len - (rhs.exponent - min_exponent));
  let mut rhs_ptr = rhs.data.offset(rhs.len);
  while rhs.data < rhs_ptr {
    rhs_ptr.dec();
    result_ptr.dec();
    let mut value = *result_ptr + *rhs_ptr + carry;
    if value >= 10 {
      value -= 10;
      carry = 1;
    } else {
      carry = 0;
    }
    *result_ptr = value;
  }

  while carry != 0 {
    result_ptr.dec();
    let mut value = *result_ptr + carry;
    if value >= 10 {
      value -= 10;
      carry = 1;
    } else {
      carry = 0;
    }
    *result_ptr = value;
  }

  result.finish()
}
