#![allow(clippy::zero_prefixed_literal)]

use core::convert::TryFrom;
use core::ops::Neg;
use scientific::{Decimals, Error, Precision, Scientific};

const POSITIVE_NUMBERS: [i128; 24] = [
  9_223_372_036_854_775_807, // 1<<63-1, i64::MAX
  5_000_000_000_000_000_000,
  2_000_000_000_000_000_000,
  1_000_000_000_000_000_000,
  1_000_000_000_000_000_001,
  0_576_688_696_100_104_512,
  0_555_555_555_555_555_555,
  0_368_664_834_771_612_712,
  0_004_623_903_520_983_798,
  0_000_000_070_000_000_000,
  0_000_000_040_000_000_000,
  0_000_000_010_000_000_000,
  0_000_000_000_799_165_882,
  0_000_000_000_525_731_755,
  0_000_000_000_000_041_403,
  0_000_000_000_000_000_100,
  0_000_000_000_000_000_070,
  0_000_000_000_000_000_010,
  0_000_000_000_000_000_005,
  0_000_000_000_000_000_004,
  0_000_000_000_000_000_003,
  0_000_000_000_000_000_002,
  0_000_000_000_000_000_001,
  0_000_000_000_000_000_000,
];

#[allow(clippy::type_complexity)]
const FUNCTIONS: [(
  &str,
  fn(i128, i128) -> Result<i128, Error>,
  fn(&Scientific, &Scientific) -> Result<Scientific, Error>,
); 5] = [
  ("add", |a, b| Ok(a + b), |a, b| Ok(a + b)),
  ("sub", |a, b| Ok(a - b), |a, b| Ok(a - b)),
  ("mul", |a, b| Ok(a * b), |a, b| Ok(a * b)),
  ("div", int_div, |a, b| a.div_truncate(b, Precision::INTEGER)),
  ("rem", int_rem, |a, b| a.div_rem(b).map(|(_, r)| r)),
];

fn int_div(a: i128, b: i128) -> Result<i128, Error> {
  if b == 0 {
    Err(Error::DivisionByZero)
  } else {
    Ok(a / b)
  }
}

fn int_rem(a: i128, b: i128) -> Result<i128, Error> {
  if b == 0 {
    Err(Error::DivisionByZero)
  } else {
    Ok(a % b)
  }
}

#[test]
fn integer() {
  let numbers = POSITIVE_NUMBERS
    .iter()
    .copied()
    .chain(POSITIVE_NUMBERS.iter().map(Neg::neg))
    .map(|n| (n, Scientific::from(n)))
    .collect::<Vec<(i128, Scientific)>>();

  for (int_a, sci_a) in numbers.iter() {
    for (int_b, sci_b) in numbers.iter() {
      // several functions
      for (name, int_fn, sci_fn) in FUNCTIONS.iter() {
        let int_result = int_fn(*int_a, *int_b);
        let sci_result = sci_fn(sci_a, sci_b);
        assert_eq!(
          int_result.map(|i| Scientific::from_string(i.to_string()).unwrap()),
          sci_result,
          "function {name}({int_a}, {int_b}) -> {int_result:?} = {sci_result:?}",
        );
      }
      // compare
      let int_result = int_a.cmp(int_b);
      let sci_result = sci_a.cmp(sci_b);
      assert_eq!(
        int_result, sci_result,
        "function {}({}, {}) -> {:?} = {:?}",
        "cmp", int_a, int_b, int_result, sci_result,
      )
    }
    // i128 via string
    let i2s = sci_a;
    let i2s_str = Scientific::from_string(int_a.to_string()).unwrap();
    assert_eq!(i2s, &i2s_str, "conversion from i128");
    // i128
    let s2i = i128::try_from(i2s);
    assert_eq!(Ok(*int_a), s2i, "conversion from/to i128");
    // i64
    let i2s = Scientific::from(*int_a as i64);
    let s2i = i64::try_from(&i2s);
    assert_eq!(Ok(*int_a as i64), s2i, "conversion from/to i64");
    // i32
    let i2s = Scientific::from(*int_a as i32);
    let s2i = i32::try_from(&i2s);
    assert_eq!(Ok(*int_a as i32), s2i, "conversion from/to i32");
    // sqrt
    if *int_a >= 0 {
      let sci_cubed = sci_a * sci_a;
      let sci_result = sci_cubed.sqrt_truncate(Decimals(100));
      assert_eq!(
        Ok(sci_a.clone()),
        sci_result,
        "function {}({}) -> {:?} = {:?}",
        "sqrt",
        int_a,
        int_a,
        sci_result,
      );
    }
  }
}
