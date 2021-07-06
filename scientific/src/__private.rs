use crate::ptr::Ptr;
use crate::types::owner::Owner;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;

// This function must not change before 0.5 since scientific-macro depends on it.
#[doc(hidden)]
#[inline(always)]
pub const fn unsafe_new(is_negative: bool, mantissa: &'static [u8], exponent: isize) -> Scientific {
  Scientific {
    sign: Sign::new(is_negative),
    data: Ptr::new_const(mantissa),
    len: mantissa.len() as isize,
    exponent,
    owner: Owner::None,
  }
}
