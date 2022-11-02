use crate::types::conversion_error::ConversionError;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use alloc::string::{String, ToString};
use core::convert::TryFrom;
use core::str::FromStr;

impl TryFrom<f64> for Scientific {
  type Error = ConversionError;

  fn try_from(value: f64) -> Result<Self, Self::Error> {
    if value.is_finite() {
      Self::from_string(value.to_string())
    } else {
      Err(ConversionError::FloatIsNotFinite)
    }
  }
}

impl From<&Scientific> for f64 {
  fn from(value: &Scientific) -> Self {
    if value.is_zero() {
      0f64
    } else if value.exponent1() > f64::MAX_10_EXP as isize {
      match value.sign {
        Sign::POSITIVE => f64::INFINITY,
        Sign::NEGATIVE => f64::NEG_INFINITY,
      }
    } else if value.exponent1() < f64::MIN_10_EXP as isize {
      0f64
    } else {
      const DIGITS: isize = 18;
      let mut str = String::with_capacity(6 + DIGITS as usize);
      if value.sign.is_negative() {
        str.push('-');
      }
      for i in 0..value.len.min(DIGITS) {
        str.push((b'0' + value.data[i] as u8) as char);
      }
      str.push('e');
      str.push_str(&(value.exponent + (value.len - DIGITS).max(0)).to_string());
      f64::from_str(&str).unwrap()
    }
  }
}

impl TryFrom<f32> for Scientific {
  type Error = ConversionError;

  fn try_from(value: f32) -> Result<Self, Self::Error> {
    if value.is_finite() {
      Self::from_string(value.to_string())
    } else {
      Err(ConversionError::FloatIsNotFinite)
    }
  }
}

impl From<&Scientific> for f32 {
  fn from(value: &Scientific) -> Self {
    if value.is_zero() {
      0f32
    } else if value.exponent1() > f32::MAX_10_EXP as isize {
      match value.sign {
        Sign::POSITIVE => f32::INFINITY,
        Sign::NEGATIVE => f32::NEG_INFINITY,
      }
    } else if value.exponent1() < f32::MIN_10_EXP as isize {
      0f32
    } else {
      const DIGITS: isize = 10;
      let mut str = String::with_capacity(6 + DIGITS as usize);
      if value.sign.is_negative() {
        str.push('-');
      }
      for i in 0..value.len.min(DIGITS) {
        str.push((b'0' + value.data[i] as u8) as char);
      }
      str.push('e');
      str.push_str(&(value.exponent + (value.len - DIGITS).max(0)).to_string());
      f32::from_str(&str).unwrap()
    }
  }
}
