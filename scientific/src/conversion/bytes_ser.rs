use crate::types::sign::Sign;
use crate::Scientific;
use alloc::vec::Vec;
use core::convert::TryFrom;

pub(crate) fn s_to_bytes(value: &Scientific) -> Vec<u8> {
  let mut result;
  if value.is_zero() {
    result = Vec::new();
  } else {
    result = Vec::with_capacity((value.len as usize * 5) / 12 + 4);
    let mantissa_sign = if value.sign == Sign::Negative {
      0x80
    } else {
      0
    };
    #[allow(clippy::collapsible_else_if)]
    if value.exponent >= -64 && value.exponent <= 59 {
      result.push(mantissa_sign | (((value.exponent as i8) as u8) & 0x7f));
    } else {
      if let Ok(e) = i8::try_from(value.exponent) {
        result.push(mantissa_sign | 0x3c);
        result.push(e as u8);
      } else {
        #[cfg(target_pointer_width = "16")]
        {
          result.push(mantissa_sign | 0x3d);
          result.extend_from_slice(&(value.exponent as i16).to_be_bytes());
        }

        #[allow(clippy::collapsible_else_if)]
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
        if let Ok(e) = i16::try_from(value.exponent) {
          result.push(mantissa_sign | 0x3d);
          result.extend_from_slice(&e.to_be_bytes());
        } else {
          #[cfg(target_pointer_width = "32")]
          {
            result.push(mantissa_sign | 0x3e);
            result.extend_from_slice(&(value.exponent as i32).to_be_bytes());
          }

          #[cfg(target_pointer_width = "64")]
          if let Ok(e) = i32::try_from(value.exponent) {
            result.push(mantissa_sign | 0x3e);
            result.extend_from_slice(&e.to_be_bytes());
          } else {
            result.push(mantissa_sign | 0x3f);
            result.extend_from_slice(&(value.exponent as i64).to_be_bytes());
          }
        }
      }

      #[cfg(not(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64"
      )))]
      compile_error!("This target_pointer_width is not yet supported, please open a issue.")
    }
    let mut p = value.data;
    let mut buf = 0;
    let mut buf_len = 0;
    let mut len = value.len;

    while len >= 3 {
      let a = *p;
      p.inc();
      let b = *p;
      p.inc();
      let c = *p;
      p.inc();
      buf = (buf << 10) | ((a as u16) * 100 + (b as u16) * 10 + (c as u16));
      buf_len += 10;
      while buf_len >= 8 {
        buf_len -= 8;
        result.push((buf >> buf_len) as u8);
      }
      len -= 3;
    }
    // what to do with the remaining digits?
    if buf_len + len * 4 > 8 {
      // adding the missing digits in 4 bits each would (together with the filler) reach or
      // exceed 10 bits and interpreted by the decoder as a full triplet -> add another triplet
      // this can't be happening with len=0, so the following read is safe
      let a = *p;
      p.inc();
      let b;
      if len == 2 {
        b = *p;
      } else {
        b = 0;
      }
      buf = (buf << 10) | ((a as u16) * 100 + (b as u16) * 10);
      buf_len += 10;
    } else {
      // add all missing digits in 4 bit each
      for _ in 0..len {
        buf = (buf << 4) | (*p as u16);
        p.inc();
        buf_len += 4;
      }
    }
    while buf_len >= 8 {
      buf_len -= 8;
      result.push((buf >> buf_len) as u8);
    }
    if buf_len > 0 {
      result.push(((buf << 8) >> buf_len) as u8);
    }
  }
  result
}
