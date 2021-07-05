use crate::ptr::Ptr;
use crate::types::owner::Owner;
use crate::types::precision::Precision;
use crate::types::scientific::{s_mut_make_zero, Scientific};
use crate::types::sign::Sign;
use crate::types::trimmer::Trimmer;
use alloc::vec::Vec;
use core::mem::size_of;

pub(crate) struct Builder(Scientific);

impl Builder {
  pub(crate) fn new(sign: Sign, len: isize, exponent: isize) -> (Builder, Ptr) {
    #[cfg(feature = "debug")]
    assert!(len > 0);

    let mut vec = vec![0; len as usize];
    let data = vec.as_mut_ptr();
    (
      Builder(Scientific {
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
  pub(crate) fn finish(mut self, trimmer: Trimmer) -> Scientific {
    s_mut_trim_zeroes(&mut self.0, trimmer);
    self.0
  }
}

pub(crate) fn s_mut_trim_zeroes(value: &mut Scientific, trimmer: Trimmer) {
  // remove leading zeroes
  while value.len > 0 && *value.data == 0 {
    value.data.inc();
    value.len -= 1;
  }

  match trimmer {
    Trimmer::Basic => (),
    Trimmer::Trim(Precision::Digits(digits)) => {
      if value.len > digits {
        value.exponent += value.len - digits;
        value.len = digits;
      }
    }
    Trimmer::Trim(Precision::Decimals(decimals)) => {
      let trim_len = -decimals - value.exponent;
      if trim_len > 0 {
        value.len -= trim_len; // this may result in a negative len
        value.exponent += trim_len;
      }
    }
  }

  // remove trailing zeroes
  while value.len > 0 && value.data[value.len - 1] == 0 {
    value.len -= 1;
    value.exponent += 1;
  }

  if value.len <= 0 {
    // if nothing remains -> set to zero
    s_mut_make_zero(value);
  } else if value.owner.capacity() > 20 * (size_of::<isize>() as isize)
    && value.len * 3 < value.owner.capacity()
  {
    // allocated size is much more than required -> reallocate
    let mut vec = Vec::with_capacity(value.len as usize);
    let mut data = Ptr::new_mut(vec.as_mut_ptr(), value.len);
    value.data.copy_to_nonoverlapping(value.len, data, 0);
    unsafe { vec.set_len(value.len as usize) };
    data.set_immutable();
    *value = Scientific {
      sign: value.sign,
      data,
      len: value.len,
      exponent: value.exponent,
      owner: Owner::new_vec(vec),
    }
  }
}
