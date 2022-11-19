use crate::ptr::Ptr;
use crate::types::mantissa::{MANTISSA_1, MANTISSA_5};
use crate::types::owner::Owner;
use crate::types::sign::Sign;

// len == 0 <=> value 0
#[derive(Clone)]
#[must_use]
pub(crate) struct Sci {
  pub(crate) sign: Sign,      // ignored for value 0, can be changed at will
  pub(crate) data: Ptr,       // should never be used for value 0
  pub(crate) len: isize,      // must be 0 for value 0, greater than 0 otherwise
  pub(crate) exponent: isize, // must be 1 for value 0
  pub(crate) owner: Owner,
}

impl Sci {
  // This constant must not change before 0.5 since scientific-macro depends on it.
  pub(crate) const ZERO: Sci = Sci {
    sign: Sign::POSITIVE,     // does not matter
    data: Ptr::new_invalid(), // a pointer to nowhere (is never used for zero)
    len: 0,                   // required for is_zero() to work
    exponent: 1,              // required for exponent() to work
    owner: Owner::None,
  };
  pub(crate) const ONE: Sci = Sci::nz_unsafe_static_new(Sign::POSITIVE, &MANTISSA_1, 0);
  pub(crate) const POINT5: Sci = Sci::nz_unsafe_static_new(Sign::POSITIVE, &MANTISSA_5, -1);

  #[inline(always)]
  pub(crate) fn assign_zero(&mut self) {
    self.data.invalidate();
    self.len = 0; // required for is_zero() to work
    self.exponent = 1; // required for exponent() to work
    self.owner = Owner::None;
  }

  #[inline(always)]
  pub(crate) const fn nz_unsafe_static_new(
    sign: Sign,
    mantissa: &'static [u8],
    exponent: isize,
  ) -> Sci {
    Sci {
      sign,
      data: Ptr::new(mantissa),
      len: mantissa.len() as isize,
      exponent,
      owner: Owner::None,
    }
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub(crate) fn is_zero(&self) -> bool {
    self.len == 0
  }

  #[inline(always)]
  pub(crate) fn exponent0(&self) -> isize {
    self.exponent + self.len
  }

  #[inline(always)]
  pub(crate) fn exponent1(&self) -> isize {
    self.exponent + self.len - 1
  }
}
