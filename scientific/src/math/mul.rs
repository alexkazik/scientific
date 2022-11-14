use crate::types::builder::Builder;
use crate::types::sci::Sci;
use crate::types::sign::Sign;

impl Sci {
  pub(crate) fn mul(&self, rhs: &Sci) -> Sci {
    if self.is_zero() || rhs.is_zero() {
      Sci::ZERO
    } else {
      nz_mul(
        self.sign ^ rhs.sign,
        self,
        rhs,
        self.exponent + rhs.exponent,
      )
    }
  }
}

#[inline(always)]
fn nz_mul(sign: Sign, lhs: &Sci, rhs: &Sci, exponent: isize) -> Sci {
  let result_len = lhs.len + rhs.len + 1;

  let (result, result_ptr) = Builder::new(sign, result_len, exponent);

  let lhs_end = lhs.data.offset(lhs.len - 1);
  let rhs_end = rhs.data.offset(rhs.len - 1);
  let mut result_end = result_ptr.offset(result_len - 1);
  let mut sum = 0;

  for index in 0..result_len - 1 {
    let mut lhs_ptr = lhs_end.offset(-((index - rhs.len + 1).max(0)));
    let mut rhs_ptr = rhs_end.offset(-(index.min(rhs.len - 1)));
    while lhs_ptr >= lhs.data && rhs_ptr <= rhs_end {
      sum += (*lhs_ptr * *rhs_ptr) as usize;
      lhs_ptr.dec();
      rhs_ptr.inc();
    }
    *result_end = (sum % 10) as i8;
    result_end.dec();
    sum /= 10;
  }
  *result_end = sum as i8;

  result.finish()
}
