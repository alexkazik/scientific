use crate::conversion::bytes_ser::s_to_bytes;
use crate::types::sign::Sign;
use crate::Scientific;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Serialize, Serializer};

impl Serialize for Scientific {
  fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
  where
    S: Serializer,
  {
    if serializer.is_human_readable() {
      serializer.serialize_str(&s_display_1e(self))
    } else {
      serializer.serialize_newtype_struct("Scientific", {
        struct SW(Vec<u8>);

        impl Serialize for SW {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
            S: Serializer,
          {
            serializer.serialize_bytes(&self.0)
          }
        }

        &SW(s_to_bytes(self))
      })
    }
  }
}

pub fn s_display_1e(value: &Scientific) -> String {
  if value.is_zero() {
    return "0".to_string();
  }
  let mut result = String::with_capacity(value.len as usize + 7);
  if value.sign == Sign::Negative {
    result.push('-');
  }
  if value.exponent >= 0 && value.exponent <= 3 {
    for i in 0..value.len {
      result.push((b'0' + value.data[i] as u8).into());
    }
    for _ in 0..value.exponent {
      result.push('0');
    }
  } else if value.exponent < 0 && -value.exponent <= value.len {
    let dot = value.len + value.exponent;
    if -value.exponent == value.len {
      result.push('0');
    }
    for i in 0..value.len {
      if i == dot {
        result.push('.');
      }
      result.push((b'0' + value.data[i] as u8).into());
    }
  } else {
    result.push((b'0' + *value.data as u8).into());
    if value.len > 1 {
      result.push('.');
      for i in 1..value.len {
        result.push((b'0' + value.data[i] as u8).into());
      }
    }
    if value.exponent0() != 1 {
      result.push('e');
      result.push_str(&value.exponent0().to_string());
    }
  }
  result
}
