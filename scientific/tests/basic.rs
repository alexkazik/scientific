use core::str::FromStr;
use scientific::{Decimals, Digits, Precision, RoundHalfAwayFromZero as RHAFZ, Scientific};
use std::default::Default;

#[test]
fn truncate() {
  assert_eq!(
    &Scientific::from_str("5453.23265346")
      .unwrap()
      .truncate(Default::default()),
    &Scientific::from_str("5453").unwrap(),
  );
  assert_eq!(
    &Scientific::from_str("5453.23265346")
      .unwrap()
      .truncate(Decimals(-2)),
    &Scientific::from_str("5400").unwrap(),
  );
  assert_eq!(
    &Scientific::from_str("5453.23265346")
      .unwrap()
      .truncate(Decimals(2)),
    &Scientific::from_str("5453.23").unwrap(),
  );
  assert_eq!(
    &Scientific::from_str("5453.23265346")
      .unwrap()
      .truncate(Digits(2)),
    &Scientific::from_str("5400").unwrap(),
  );
}

#[test]
fn round() {
  const TESTS: [(&str, Precision, &str); 7] = [
    ("5453.23265346", Digits(2), "5500"),
    ("5453.23265346", Digits(3), "5450"),
    ("5453.23265346", Decimals(-5), "0"),
    ("5453.23265346", Decimals(-4), "10000"),
    ("5453.23265346", Decimals(-3), "5000"),
    ("5453.23265346", Decimals(0), "5453"),
    ("5453.23265346", Decimals(3), "5453.233"),
  ];
  for (base, precision, result) in TESTS {
    assert_eq!(
      Scientific::from_str(base).unwrap().round(precision, RHAFZ),
      Scientific::from_str(result).unwrap(),
    );
  }
}

#[test]
fn raw_parts() {
  let n1 = Scientific::from_string("12.34e10".to_string()).unwrap();
  let n2 = Scientific::from_raw_parts(
    n1.is_sign_negative(),
    n1.as_raw_mantissa().to_vec(),
    n1.exponent(),
  );
  assert_eq!(Ok(n1), n2);
}
