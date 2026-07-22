use crate::types::limited::{
  Exponent, ExponentOutOfRangeError, Length, Unchecked, UnwrapOutOfRange,
};
use crate::types::owner::Owner;
use crate::types::ptr::Ptr;
use crate::types::sci::Sci;
use crate::types::sign::Sign;

pub(crate) struct Builder(Sci);

impl Builder {
  pub(crate) fn new(sign: Sign, len: Length, exponent: Exponent) -> (Builder, Ptr) {
    #[cfg(feature = "debug")]
    assert!(len > 0);

    let mut vec = vec![0; len.to_usize()];
    let data = Ptr::new_mut(vec.as_mut_slice());
    (
      Builder(Sci {
        sign,
        data,
        len,
        exponent,
        owner: Owner::new(vec),
      }),
      data,
    )
  }

  #[inline]
  pub(crate) fn from_data(
    sign: Sign,
    data: Ptr,
    len: Length,
    exponent: Exponent,
    owner: Owner,
  ) -> Sci {
    Builder::try_from_data(sign, data, len, exponent, owner).unwrap_out_of_range()
  }

  #[inline]
  pub(crate) fn finish(self) -> Sci {
    self.try_finish().unwrap_out_of_range()
  }

  #[inline]
  pub(crate) fn try_from_data(
    sign: Sign,
    data: Ptr,
    len: Length,
    exponent: Exponent,
    owner: Owner,
  ) -> Result<Sci, ExponentOutOfRangeError> {
    Builder(Sci {
      sign,
      data,
      len,
      exponent,
      owner,
    })
    .try_finish()
  }

  #[inline]
  pub(crate) fn try_finish(mut self) -> Result<Sci, ExponentOutOfRangeError> {
    b_mut_trim_zeroes(&mut self.0)?;
    Ok(self.0)
  }
}

fn b_mut_trim_zeroes(value: &mut Sci) -> Result<(), ExponentOutOfRangeError> {
  let mut exponent = value.exponent;

  // remove leading zeroes
  while value.len > 0 && *value.data == 0 {
    value.data.inc();
    value.len -= Unchecked(1);
  }

  // remove trailing zeroes
  while value.len > 0 && value.data[value.len - 1] == 0 {
    value.len -= Unchecked(1);
    // Safety: exponent is at max expanded by length, and there is no overflow
    exponent += Unchecked(1);
  }

  if value.len <= 0 {
    // if nothing remains -> set to zero
    value.assign_zero();
  } else {
    value.exponent = Exponent::try_new(exponent)?;
  }

  Ok(())
}
