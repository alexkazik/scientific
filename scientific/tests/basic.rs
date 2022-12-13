use core::str::FromStr;
use scientific::{Decimals, Digits, RoundHalfAwayFromZero, Scientific};
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
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Digits(2), RoundHalfAwayFromZero),
    Scientific::from_str("5500").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Digits(3), RoundHalfAwayFromZero),
    Scientific::from_str("5450").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Decimals(0), RoundHalfAwayFromZero),
    Scientific::from_str("5453").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Decimals(3), RoundHalfAwayFromZero),
    Scientific::from_str("5453.233").unwrap(),
  );
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
