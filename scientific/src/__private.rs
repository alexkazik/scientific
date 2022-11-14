use crate::types::sci::Sci;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;

// This function must not change before 0.5 since scientific-macro depends on it.
#[doc(hidden)]
#[inline(always)]
pub const fn unsafe_new(is_negative: bool, mantissa: &'static [u8], exponent: isize) -> Scientific {
  Scientific {
    inner: Sci::nz_unsafe_static_new(Sign::new(is_negative), mantissa, exponent),
  }
}
