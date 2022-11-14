use crate::ptr::Ptr;
use crate::types::owner::Owner;
use crate::types::precision::Precision;
use crate::types::rounding::{Rounding, Truncate};
use crate::types::sci::Sci;
use crate::types::sign::Sign;
use alloc::vec::Vec;
use core::mem::size_of;

pub(crate) struct Builder(Sci);

impl Builder {
  pub(crate) fn new(sign: Sign, len: isize, exponent: isize) -> (Builder, Ptr) {
    #[cfg(feature = "debug")]
    assert!(len > 0);

    let mut vec = vec![0; len as usize];
    let data = vec.as_mut_ptr();
    (
      Builder(Sci {
        sign,
        data: Ptr::new(data, len),
        len,
        exponent,
        owner: Owner::new_vec(vec),
      }),
      Ptr::new_mut(data, len),
    )
  }

  #[inline(always)]
  pub(crate) fn new_with_data(
    sign: Sign,
    data: Ptr,
    len: isize,
    exponent: isize,
    owner: Owner,
  ) -> Builder {
    Builder(Sci {
      sign,
      data,
      len,
      exponent,
      owner,
    })
  }

  #[inline(always)]
  pub(crate) fn finish(mut self) -> Sci {
    // there might be no leading zero -> `Truncate` must return true for `is_truncate`
    #[cfg(feature = "debug")]
    assert!(Truncate::is_truncate());

    b_mut_trim_zeroes(&mut self.0, None, Truncate);
    self.0
  }

  #[inline(always)]
  pub(crate) fn round<R: Rounding>(mut self, precision: Precision, rounding: R) -> Sci {
    if !R::is_truncate() {
      // the first digit must be zero in order to be able to propagate the carry for rounding away from zero
      #[cfg(feature = "debug")]
      assert_eq!(*self.0.data, 0);
    }

    b_mut_trim_zeroes(&mut self.0, Some(precision), rounding);
    self.0
  }
}

fn b_mut_trim_zeroes<R: Rounding>(value: &mut Sci, precision: Option<Precision>, rounding: R) {
  // remove leading zeroes
  while value.len > 0 && *value.data == 0 {
    value.data.inc();
    value.len -= 1;
  }

  let mut do_round = false;

  match precision {
    None => (),
    Some(Precision::Digits(digits)) => {
      let digits = digits + isize::from(!<R>::is_truncate());
      if value.len >= digits {
        value.exponent += value.len - digits;
        value.len = digits;
        do_round = true;
      }
    }
    Some(Precision::Decimals(decimals)) => {
      let decimals = decimals + isize::from(!<R>::is_truncate());
      let trim_len = -decimals - value.exponent;
      if trim_len >= 0 {
        value.len -= trim_len; // this may result in a negative len
        value.exponent += trim_len;
        do_round = true;
      }
    }
  }

  if !<R>::is_truncate() && do_round && value.len > 0 {
    // do the rounding (only if mantissa is left)
    if value.data[value.len - 1] != 0
      && rounding.round_away_from_zero(
        value.sign.is_negative(),
        value.data[value.len - 2],
        value.data[value.len - 1],
      )
    {
      // the digit after the cut-off is not zero
      // and the rounding does result in an (absolute) increase
      let mut ptr = value.data.offset(value.len - 2);
      ptr.set_mutable(); // it is safe to so since we still have full control over the mantissa
      let mut val = *ptr + 1;
      while val == 10 {
        *ptr = 0;
        ptr.dec();
        val = *ptr + 1;
      }
      *ptr = val;
      if ptr < value.data {
        // overflow to the spare space
        value.len += 1;
        value.data.dec();
      }
    }
  }

  if !<R>::is_truncate() {
    // remove the digit behind the cut-off
    value.len -= 1;
    value.exponent += 1;
  }

  // remove trailing zeroes
  while value.len > 0 && value.data[value.len - 1] == 0 {
    value.len -= 1;
    value.exponent += 1;
  }

  if value.len <= 0 {
    // if nothing remains -> set to zero
    value.assign_zero();
  } else if value.owner.capacity() > 20 * (size_of::<isize>() as isize)
    && value.len * 3 < value.owner.capacity()
  {
    // allocated size is much more than required -> reallocate
    let mut vec = Vec::with_capacity(value.len as usize);
    let mut data = Ptr::new_mut(vec.as_mut_ptr(), value.len);
    value.data.copy_to_nonoverlapping(value.len, data, 0);
    unsafe { vec.set_len(value.len as usize) };
    data.set_immutable();
    *value = Sci {
      sign: value.sign,
      data,
      len: value.len,
      exponent: value.exponent,
      owner: Owner::new_vec(vec),
    }
  }
}
