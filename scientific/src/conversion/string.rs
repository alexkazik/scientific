use crate::ptr::Ptr;
use crate::types::builder::Builder;
use crate::types::conversion_error::ConversionError;
use crate::types::owner::Owner;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use alloc::string::ToString;
use core::str::FromStr;

impl FromStr for Scientific {
  type Err = ConversionError;

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut s = s.to_string();
    s_parse(s.as_mut_ptr(), s.len(), Owner::new_string(s))
  }
}

pub(crate) fn s_parse(
  data_start: *mut u8,
  len: usize,
  owner: Owner,
) -> Result<Scientific, ConversionError> {
  let len = len as isize;
  let mut data_start = Ptr::new_mut(data_start, len);
  let data_end = data_start.offset(len);

  // check if len > 0
  if data_start == data_end {
    return Err(ConversionError::ParseError);
  }
  // remove sign if any
  let next = *data_start as u8;
  let sign = Sign::new(next == b'-');
  if sign.is_negative() || next == b'+' {
    data_start.inc();
  }

  // check if len > 0 and first char is 0-9 or .
  if data_start == data_end {
    return Err(ConversionError::ParseError);
  }
  let next = *data_start as u8;
  if !((b'0'..=b'9').contains(&next) || next == b'.') {
    return Err(ConversionError::ParseError);
  }

  let mut data_ptr = data_start;
  while data_ptr != data_end {
    let next = *data_ptr as u8;
    if !(b'0'..=b'9').contains(&next) {
      break;
    }
    *data_ptr = (next & 0x0f) as i8; // convert ascii to decimal
    data_ptr.inc();
  }

  let mut mantissa_end = data_ptr;

  let mut dot_len = 0;
  let exponent_start;
  let exponent_len;
  if data_ptr == data_end {
    // end of input = neither dot not exp
    exponent_start = data_end;
    exponent_len = 0;
  } else {
    let next = *data_ptr as u8;
    if next != b'.' {
      // did not found a dot
    } else {
      // found a dot
      data_ptr.inc();
      while data_ptr != data_end {
        let next = *data_ptr as u8;
        if !(b'0'..=b'9').contains(&next) {
          break;
        }
        *mantissa_end = (next & 0x0f) as i8; // convert ascii to decimal
        mantissa_end.inc();
        data_ptr.inc();
        dot_len += 1;
      }
    }
    if data_ptr == data_end {
      // no exp
      exponent_start = data_end;
      exponent_len = 0;
    } else {
      // check for exp
      let next = *data_ptr as u8;
      if !(next == b'e' || next == b'E') {
        return Err(ConversionError::ParseError);
      }
      data_ptr.inc();
      exponent_start = data_ptr;
      exponent_len = data_end.offset_from(data_ptr);
      if exponent_len == 0 {
        // specified 'e' but nothing behind it
        return Err(ConversionError::ParseError);
      }
    }
  }

  let exponent;
  if exponent_len == 0 {
    exponent = 0;
  } else {
    exponent = isize::from_str(unsafe {
      core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        exponent_start.as_slice(exponent_len).as_ptr(),
        exponent_len as usize,
      ))
    })
    .map_err(|_| ConversionError::ParseError)?;
  }

  if data_start == mantissa_end {
    // no digits given (neither before or after the dot)
    return Err(ConversionError::ParseError);
  }

  let len = mantissa_end.offset_from(data_start);

  Ok(Builder::new_with_data(sign, data_start, len, exponent - dot_len, owner).finish())
}
