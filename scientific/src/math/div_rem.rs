use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::sci::Sci;

impl Sci {
  pub(crate) fn div_rem(&self, rhs: &Sci) -> Result<(Sci, Sci), Error> {
    let quot = self.div(rhs, Precision::INTEGER)?;

    let rem = self.sub(&quot.mul(rhs));

    Ok((quot, rem))
  }
}
