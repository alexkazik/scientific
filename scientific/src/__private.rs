use crate::types::scientific::{s_unsafe_static_new, Scientific};
use crate::types::sign::Sign;

// This function must not change before 0.5 since scientific-macro depends on it.
#[doc(hidden)]
#[inline(always)]
pub const fn unsafe_new(is_negative: bool, mantissa: &'static [u8], exponent: isize) -> Scientific {
  s_unsafe_static_new(Sign::new(is_negative), mantissa, exponent)
}
