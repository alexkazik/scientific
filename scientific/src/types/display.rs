use crate::types::sci::Sci;
use core::fmt::{Formatter, Write};

impl Sci {
  pub(crate) fn display(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      return f.write_char('0');
    }
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    let exp = self.exponent0();
    #[allow(clippy::manual_range_contains)]
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
