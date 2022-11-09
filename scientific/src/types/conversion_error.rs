use core::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConversionError {
  FloatIsNotFinite,
  NumberTooLarge,
  ParseError,
  NumberIsNegative,
  NumberIsNotAnInteger,
  /// Only used by [Scientific::from_bytes](crate::Scientific::from_bytes)
  ExponentTooLargeForThisPlatform,
}

impl Display for ConversionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
    match self {
      ConversionError::FloatIsNotFinite => f.write_str("Float is not finite"),
      ConversionError::NumberTooLarge => f.write_str("Number too large"),
      ConversionError::ParseError => f.write_str("Parse error"),
      ConversionError::NumberIsNegative => f.write_str("Number is negative"),
      ConversionError::NumberIsNotAnInteger => f.write_str("Number is not an integer"),
      ConversionError::ExponentTooLargeForThisPlatform => {
        f.write_str("Exponent too large for this platform")
      }
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionError {}
