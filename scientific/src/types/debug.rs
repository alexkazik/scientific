use crate::types::sci::Sci;
use core::fmt::{Formatter, Write};

impl Sci {
  pub(crate) fn debug(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
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
