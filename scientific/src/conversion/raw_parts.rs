use crate::ptr::Ptr;
use crate::types::builder::s_mut_trim_zeroes;
use crate::types::conversion_error::ConversionError;
use crate::types::mantissa::MANTISSA_0;
use crate::types::owner::Owner;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use crate::types::trimmer::Trimmer;
use alloc::vec::Vec;

pub fn s_as_raw_mantissa(value: &Scientific) -> &[u8] {
  if value.is_zero() {
    // value.data is undefined for zero
    // should a pointer to an empty slice be returned?
    &MANTISSA_0
  } else {
    value.data.as_slice(value.len)
  }
}

pub fn s_from_raw_parts(
  negative: bool,
  mantissa: Vec<u8>,
  exponent: isize,
) -> Result<Scientific, ConversionError> {
  for v in mantissa.iter() {
    if *v > 9 {
      return Err(ConversionError::ParseError);
    }
  }

  let len = mantissa.len() as isize;
  let mut result = Scientific {
    sign: if negative {
      Sign::Negative
    } else {
      Sign::Positive
    },
    data: Ptr::new(mantissa.as_ptr(), len),
    len,
    exponent,
    owner: Owner::new_vec(mantissa),
  };
  s_mut_trim_zeroes(&mut result, Trimmer::Basic);
  Ok(result)
}
