use crate::ptr::Ptr;
use crate::types::owner::Owner;
use crate::types::sign::Sign;
use core::fmt::{Debug, Display, Formatter, Write};

/// Arbitrary precision scientific number
///
/// See the [module-level documentation](crate) for more details.
// len == 0 <=> value 0
#[derive(Clone)]
#[must_use]
pub struct Scientific {
  pub(crate) sign: Sign,      // ignored for value 0, can be changed at will
  pub(crate) data: Ptr,       // should never be used for value 0
  pub(crate) len: isize,      // must be 0 for value 0, greater than 0 otherwise
  pub(crate) exponent: isize, // must be 1 for value 0
  pub(crate) owner: Owner,
}

#[inline(always)]
pub(crate) fn s_mut_make_zero(value: &mut Scientific) {
  value.data.invalidate();
  value.len = 0; // required for is_zero() to work
  value.exponent = 1; // required for exponent() to work
  value.owner = Owner::None;
}

#[cfg(any(doc, feature = "arc"))]
unsafe impl Send for Scientific {}
#[cfg(any(doc, feature = "arc"))]
unsafe impl Sync for Scientific {}

impl Display for Scientific {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      return f.write_char('0');
    }
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    let exp = self.exponent0();
    if exp >= -1 && exp <= 0 {
      f.write_char('0')?;
      f.write_char('.')?;
      for _ in exp..0 {
        f.write_char('0')?;
      }
      for i in 0..self.len {
        f.write_char((b'0' + self.data[i] as u8).into())?;
      }
    } else if exp > 1 && exp <= 7 {
      let mid = exp.min(self.len);
      for i in 0..mid {
        f.write_char((b'0' + self.data[i] as u8).into())?;
      }
      for _ in mid..exp {
        f.write_char('0')?;
      }
      if self.len > exp {
        f.write_char('.')?;
        for i in exp..self.len {
          f.write_char((b'0' + self.data[i] as u8).into())?;
        }
      }
    } else {
      f.write_char((b'0' + *self.data as u8).into())?;
      if self.len > 1 {
        f.write_char('.')?;
        for i in 1..self.len {
          f.write_char((b'0' + self.data[i] as u8).into())?;
        }
      }
      if exp != 1 {
        write!(f, "e{}", exp - 1)?;
      }
    }
    Ok(())
  }
}

impl Debug for Scientific {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      return f.write_char('0');
    }
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    for i in 0..self.len {
      f.write_char((b'0' + self.data[i] as u8).into())?;
    }
    if self.exponent != 0 {
      write!(f, "e{}", self.exponent)?;
    }
    Ok(())
  }
}
