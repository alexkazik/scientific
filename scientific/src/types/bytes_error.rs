use crate::ConversionError;
use core::fmt::{Display, Formatter};

pub(crate) enum BytesError {
  // will only happen on 16 and 32 bit platforms, is not used on 64 bit platforms
  #[allow(dead_code)]
  ExponentTooLarge,
  MalformedNumber,
  NumberTooShort,
  NumberHasLeadingZeroes,
  InvalidTrailingZeroes,
}

impl Display for BytesError {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(match self {
      BytesError::ExponentTooLarge => "exponent too large for this platform",
      BytesError::MalformedNumber => "malformed number",
      BytesError::NumberTooShort => "number too short",
      BytesError::NumberHasLeadingZeroes => "number has leading zero",
      BytesError::InvalidTrailingZeroes => "invalid trailing zero(es)",
    })
  }
}

impl From<BytesError> for ConversionError {
  fn from(error: BytesError) -> Self {
    match error {
      BytesError::ExponentTooLarge => ConversionError::ExponentTooLargeForThisPlatform,
      _ => ConversionError::ParseError,
    }
  }
}
