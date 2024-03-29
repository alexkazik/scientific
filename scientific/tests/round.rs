use scientific::{Precision, Rounding, Scientific};
use std::str::FromStr;

#[test]
fn round() {
  let test = [
    Scientific::from_str("-1.7").unwrap(),
    Scientific::from_str("-1.5").unwrap(),
    Scientific::from_str("-1.3").unwrap(),
    Scientific::from_str("-0.7").unwrap(),
    Scientific::from_str("-0.5").unwrap(),
    Scientific::from_str("-0.3").unwrap(),
    Scientific::from_str("0.3").unwrap(),
    Scientific::from_str("0.5").unwrap(),
    Scientific::from_str("0.7").unwrap(),
    Scientific::from_str("1.3").unwrap(),
    Scientific::from_str("1.5").unwrap(),
    Scientific::from_str("1.7").unwrap(),
  ];

  for (rounding, results) in [
    (
      Rounding::RoundDown,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
      ],
    ),
    (
      Rounding::RoundUp,
      [
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundTowardsZero,
      [
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
      ],
    ),
    (
      Rounding::RoundAwayFromZero,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfDown,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfUp,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfTowardsZero,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfAwayFromZero,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfToEven,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
    (
      Rounding::RoundHalfToOdd,
      [
        Scientific::from_str("-2").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("-1").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("0").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("1").unwrap(),
        Scientific::from_str("2").unwrap(),
      ],
    ),
  ] {
    for (t, r) in test.iter().zip(results) {
      assert_eq!(
        t.round(Precision::INTEGER, rounding),
        r,
        "while rounding {rounding:?} {t}"
      );
    }
  }
}
