use crate::types::limited::RangeTo;
use crate::types::sci::Sci;
use core::fmt::{Formatter, Write};

impl Sci {
  pub(crate) fn nz_display<W: Write>(&self, f: &mut W) -> Result<(), core::fmt::Error> {
    if self.sign.is_negative() {
      f.write_char('-')?;
    }
    let exp = self.exponent0();
    #[allow(clippy::manual_range_contains)]
    if exp >= -1 && exp <= 0 {
      f.write_char('0')?;
      f.write_char('.')?;
      for _ in exp.range_to(0) {
        f.write_char('0')?;
      }
      self.data.write_chars(f, 0.range_to(self.len))?;
    } else if exp > 1 && exp <= 7 {
      // the ` + 0` is a noop in calculation, but expands the type from `Limited<1>` to `Limited<2>` to match the left size
      let mid = exp.min(self.len + 0);
      self.data.write_chars(f, 0.range_to(mid))?;
      for _ in mid.range_to(exp) {
        f.write_char('0')?;
      }
      if self.len > exp {
        f.write_char('.')?;
        self.data.write_chars(f, exp.range_to(self.len))?;
      }
    } else {
      self.data.write_first_char(f)?;
      if self.len > 1 {
        f.write_char('.')?;
        self.data.write_chars(f, 1.range_to(self.len))?;
      }
      if exp != 1 {
        write!(f, "e{}", exp - 1)?;
      }
    }
    Ok(())
  }

  pub(crate) fn display(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    if self.is_zero() {
      f.write_char('0')
    } else {
      self.nz_display(f)
    }
  }
}
