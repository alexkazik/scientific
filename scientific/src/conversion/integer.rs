use crate::Scientific;

macro_rules! conversion_signed {
  ($ty:ident, $len:expr, $const:ident) => {
    impl TryFrom<&Scientific> for $ty {
      type Error = ConversionError;

      fn try_from(value: &Scientific) -> Result<Self, Self::Error> {
        if value.is_zero() {
          Ok(0)
        } else if value.exponent < 0 {
          Err(ConversionError::NumberIsNotAnInteger)
        } else {
          match s_compare::<false>(value, &$const) {
            Ordering::Greater => Err(ConversionError::NumberTooLarge),
            Ordering::Equal => {
              if value.sign == Sign::Negative {
                // Special case since negating the minimum number does not work, see `isize::wrapping_neg`.
                Ok($ty::MIN)
              } else {
                Err(ConversionError::NumberTooLarge)
              }
            }
            Ordering::Less => {
              let mut value_ptr = value.data;
              let mut result = 0;

              for _ in value.exponent.max(0)..value.exponent0() {
                result = result * 10 + $ty::from(*value_ptr);
                value_ptr.inc();
              }
              result *= $ty::from(10_i8).pow(value.exponent.max(0) as u32);

              if value.sign == Sign::Negative {
                result = -result;
              }
              Ok(result)
            }
          }
        }
      }
    }

    impl From<$ty> for Scientific {
      fn from(mut value: $ty) -> Self {
        if value == 0 {
          Scientific::ZERO
        } else if value == $ty::MIN {
          // Special case since negating the minimum number does not work, see `isize::wrapping_neg`.
          $const
        } else {
          let mut sign = Sign::Positive;
          if value < 0 {
            value = -value;
            sign = Sign::Negative;
          }
          let (result, mut result_ptr) = Builder::new(sign, $len, 0);
          result_ptr.mut_offset($len);
          while value > 0 {
            result_ptr.dec();
            *result_ptr = (value % 10) as i8;
            value /= 10;
          }
          result.finish(Trimmer::Basic)
        }
      }
    }
  };
}

macro_rules! conversion_unsigned {
  ($ty:ident, $len:expr, $const:ident) => {
    impl TryFrom<&Scientific> for $ty {
      type Error = ConversionError;

      fn try_from(value: &Scientific) -> Result<Self, Self::Error> {
        if value.is_zero() {
          Ok(0)
        } else if value.exponent < 0 {
          Err(ConversionError::NumberIsNotAnInteger)
        } else if value.sign == Sign::Negative {
          Err(ConversionError::NumberIsNegative)
        } else if s_compare::<false>(value, &$const) != Ordering::Less {
          Err(ConversionError::NumberTooLarge)
        } else {
          let mut value_ptr = value.data;
          let mut result = 0;

          for _ in value.exponent.max(0)..value.exponent0() {
            result = result * 10 + $ty::from(*value_ptr as u8);
            value_ptr.inc();
          }
          result *= $ty::from(10_u8).pow(value.exponent.max(0) as u32);

          Ok(result)
        }
      }
    }

    impl From<$ty> for Scientific {
      fn from(mut value: $ty) -> Self {
        if value == 0 {
          Scientific::ZERO
        } else {
          let (result, mut result_ptr) = Builder::new(Sign::Positive, $len, 0);
          result_ptr.mut_offset($len);
          while value > 0 {
            result_ptr.dec();
            *result_ptr = (value % 10) as i8;
            value /= 10;
          }
          result.finish(Trimmer::Basic)
        }
      }
    }
  };
}

mod c_use {
  pub(super) use crate::__private::unsafe_new;
  pub(super) use crate::math::compare::s_compare;
  pub(super) use crate::types::builder::Builder;
  pub(super) use crate::types::conversion_error::ConversionError;
  pub(super) use crate::types::scientific::Scientific;
  pub(super) use crate::types::sign::Sign;
  pub(super) use crate::types::trimmer::Trimmer;
  pub(super) use core::cmp::Ordering;
  pub(super) use core::convert::TryFrom;
}

mod c_i8 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 3] = [1, 2, 8];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_signed!(i8, DIGITS.len() as isize, SCI);
}

mod c_u8 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 3] = [2, 5, 6];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_unsigned!(u8, DIGITS.len() as isize, SCI);
}

mod c_i16 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 5] = [3, 2, 7, 6, 8];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_signed!(i16, DIGITS.len() as isize, SCI);
}

mod c_u16 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 5] = [6, 5, 5, 3, 6];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_unsigned!(u16, DIGITS.len() as isize, SCI);
}

mod c_i32 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 10] = [2, 1, 4, 7, 4, 8, 3, 6, 4, 8];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_signed!(i32, DIGITS.len() as isize, SCI);
}

mod c_u32 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 10] = [4, 2, 9, 4, 9, 6, 7, 2, 9, 6];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_unsigned!(u32, DIGITS.len() as isize, SCI);
}

mod c_i64 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 19] = [9, 2, 2, 3, 3, 7, 2, 0, 3, 6, 8, 5, 4, 7, 7, 5, 8, 0, 8];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_signed!(i64, DIGITS.len() as isize, SCI);
}

mod c_u64 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 20] = [1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 6];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_unsigned!(u64, DIGITS.len() as isize, SCI);
}

mod c_i128 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 39] = [
    1, 7, 0, 1, 4, 1, 1, 8, 3, 4, 6, 0, 4, 6, 9, 2, 3, 1, 7, 3, 1, 6, 8, 7, 3, 0, 3, 7, 1, 5, 8, 8,
    4, 1, 0, 5, 7, 2, 8,
  ];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_signed!(i128, DIGITS.len() as isize, SCI);
}

mod c_u128 {
  use crate::conversion::integer::c_use::*;

  const DIGITS: [u8; 39] = [
    3, 4, 0, 2, 8, 2, 3, 6, 6, 9, 2, 0, 9, 3, 8, 4, 6, 3, 4, 6, 3, 3, 7, 4, 6, 0, 7, 4, 3, 1, 7, 6,
    8, 2, 1, 1, 4, 5, 6,
  ];
  const SCI: Scientific = unsafe_new(true, &DIGITS, 0);
  conversion_unsigned!(u128, DIGITS.len() as isize, SCI);
}

impl From<isize> for Scientific {
  fn from(val: isize) -> Self {
    Scientific::from(val as i128)
  }
}

impl From<usize> for Scientific {
  fn from(val: usize) -> Self {
    Scientific::from(val as u128)
  }
}
