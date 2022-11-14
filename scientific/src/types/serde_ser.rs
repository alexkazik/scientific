use crate::types::sci::Sci;
use crate::types::scientific::Scientific;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Serialize, Serializer};

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl Serialize for Scientific {
  fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
  where
    S: Serializer,
  {
    if serializer.is_human_readable() {
      serializer.serialize_str(&s_display_1e(&self.inner))
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

        &SW(self.to_bytes())
      })
    }
  }
}

fn s_display_1e(value: &Sci) -> String {
  if value.is_zero() {
    return "0".to_string();
  }
  let mut result = String::with_capacity(value.len as usize + 7);
  if value.sign.is_negative() {
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
