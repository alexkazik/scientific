use crate::conversion::raw_parts::s_from_raw_parts;
use crate::types::precision::Precision::{Decimals, Digits};
use crate::types::scientific::Scientific;
use core::str::FromStr;
use std::default::Default;

mod float;
mod integer;

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
      .round(Digits(2)),
    Scientific::from_str("5500").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Digits(3)),
    Scientific::from_str("5450").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Decimals(0)),
    Scientific::from_str("5453").unwrap(),
  );
  assert_eq!(
    Scientific::from_str("5453.23265346")
      .unwrap()
      .round(Decimals(3)),
    Scientific::from_str("5453.233").unwrap(),
  );
}

#[test]
fn raw_parts() {
  let n1 = Scientific::from_string("12.34e10".to_string()).unwrap();
  let n2 = s_from_raw_parts(
    n1.is_sign_negative(),
    n1.as_raw_mantissa().to_vec(),
    n1.exponent(),
  );
  assert_eq!(Ok(n1), n2);
}
