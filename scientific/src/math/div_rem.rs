use crate::types::error::Error;
use crate::types::precision::Precision;
use crate::types::scientific::Scientific;

pub(crate) fn export_div_rem(
  lhs: &Scientific,
  rhs: &Scientific,
) -> Result<(Scientific, Scientific), Error> {
  let quot = lhs.div(rhs, Precision::INTEGER)?;

  let rem = lhs - &(&quot * rhs);

  Ok((quot, rem))
}
