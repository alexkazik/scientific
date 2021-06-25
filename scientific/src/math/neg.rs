use crate::types::scientific::Scientific;
use core::ops::Neg;

impl Neg for &Scientific {
  type Output = Scientific;

  #[inline(always)]
  fn neg(self) -> Self::Output {
    export_neg(self)
  }
}

#[inline(always)]
fn export_neg(value: &Scientific) -> Scientific {
  let mut result = value.clone();
  result.sign = !result.sign;
  result
}

#[inline(always)]
pub(crate) fn export_neg_assign(value: &mut Scientific) {
  value.sign = !value.sign;
}
