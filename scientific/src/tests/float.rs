use crate::types::error::Error;
use crate::types::scientific::Scientific;
use crate::Precision;
use core::ops::Neg;
use std::convert::TryFrom;

const POSITIVE_NUMBERS: [f64; 32] = [
  9_223_372_036_854_775_807.0, // 1<<63-1, i64::MAX
  5_000_000_000_000_000_000.1,
  5_000_000_000_000_050_000.0,
  5_000_000_000_000_000_000.0,
  2_000_000_000_000_000_000.0,
  1_000_000_000_000_000_000.0,
  0_576_688_696_100_104_512.0,
  0_555_555_555_555_555_555.0,
  0_368_664_834_771_612_712.0,
  0_004_623_903_520_983_798.0,
  0_000_000_070_000_000_000.0,
  0_000_000_040_000_000_000.0,
  0_000_000_010_000_000_000.0,
  0_000_000_000_799_165_882.0,
  0_000_000_000_525_731_755.0,
  0_000_000_000_000_041_403.0,
  0_000_000_000_000_000_100.0,
  0_000_000_000_000_000_070.0,
  0_000_000_000_000_000_010.0,
  0_000_000_000_000_000_005.0,
  0_000_000_000_000_000_004.0,
  0_000_000_000_000_000_003.0,
  0_000_000_000_000_000_002.0,
  0_000_000_000_000_000_001.0,
  0_000_000_000_000_000_000.0,
  7e100,
  3e100,
  1e100,
  7e-100,
  3e-100,
  1e-100,
  0.1,
];

const FUNCTIONS: [(
  &str,
  fn(f64, f64) -> f64,
  fn(&Scientific, &Scientific) -> Result<Scientific, Error>,
); 4] = [
  ("add", |a, b| a + b, |a, b| Ok(a + b)),
  ("sub", |a, b| a - b, |a, b| Ok(a - b)),
  ("mul", |a, b| a * b, |a, b| Ok(a * b)),
  ("div", |a, b| a / b, |a, b| a.div(b, Precision::F64)),
  // rem can't be tested due to the lack of precision in float
];

#[test]
fn float() {
  let numbers = POSITIVE_NUMBERS
    .iter()
    .map(|i| *i)
    .chain(POSITIVE_NUMBERS.iter().map(Neg::neg))
    .map(|n| (n, Scientific::from_string(n.to_string()).unwrap()))
    .collect::<Vec<(f64, Scientific)>>();

  for (flt_a, sci_a) in numbers.iter() {
    for (flt_b, sci_b) in numbers.iter() {
      for (name, flt_fn, sci_fn) in FUNCTIONS.iter() {
        let flt_result = flt_fn(*flt_a, *flt_b);
        let sci_result = sci_fn(sci_a, sci_b);
        let diff = diff(sci_a, sci_b, flt_result, &sci_result);
        assert!(
          diff < 2e-15_f64,
          "function {}({}, {}) -> {:?} = {:?}; diff: {:e}",
          name,
          flt_a,
          flt_b,
          flt_result,
          sci_result,
          diff,
        );
      }
      // partial_cmp
      let flt_result = flt_a.partial_cmp(flt_b);
      let sci_result = sci_a.partial_cmp(sci_b);
      assert_eq!(
        flt_result, sci_result,
        "function {}({}, {}) -> {:?} = {:?}",
        "partial_cmp", flt_a, flt_b, flt_result, sci_result,
      );
    }
    // powi
    for int_b in &[0, 1, 2, 10, 100] {
      let flt_result = flt_a.powi(*int_b);
      let sci_result = sci_a.powi((*int_b) as usize);
      let diff = diff(sci_a, sci_a, flt_result, &Ok(sci_result.clone()));
      assert!(
        diff < 5e-15_f64,
        "function {}({}, {}) -> {:?} = {:?}; diff: {:e}",
        "powi",
        flt_a,
        int_b,
        flt_result,
        sci_result,
        diff,
      );
    }
    // sqrt
    let flt_result = flt_a.sqrt();
    let sci_result = sci_a.sqrt(Precision::F64);
    let diff = diff(sci_a, sci_a, flt_result, &sci_result);
    assert!(
      diff < 2e-15_f64,
      "function {}({}) -> {:?} = {:?}; diff: {:e}",
      "sqrt",
      flt_a,
      flt_result,
      sci_result,
      diff,
    );
  }
}

fn diff(sci_a: &Scientific, sci_b: &Scientific, flt: f64, sci: &Result<Scientific, Error>) -> f64 {
  match sci {
    Err(_) => {
      if flt.is_finite() {
        // Error while calculating sci, but a finite number while calculating f64 -> a mismatch
        1.0
      } else {
        // Error while calculating sci (probably division by zero) and
        // infinite or nan while calculating f64 -> probably a match
        // (1.0 / 0.0 => infinite, 0.0 / 0.0 => nan)
        0.0
      }
    }
    Ok(sci) => {
      if flt.is_infinite() && f64::from(sci).is_infinite() {
        // if the sci result as a f64 and the f64 result itself is infinite then
        // the number is too large to calculate in f64 -> match (just maybe, but can't be proven wrong)
        0.0
      } else {
        match Scientific::try_from(flt) {
          Err(_) => 1.0, // sci produced a finite value but f64 didn't -> a mismatch (infinite is handled above)
          Ok(flt) => {
            let max_exp = sci
              .exponent1()
              .max(sci_a.exponent1())
              .max(sci_b.exponent1());
            let n1 = sci >> max_exp;
            let n2 = &flt >> max_exp;
            (&(&n1 - &n2).abs()).into()
          }
        }
      }
    }
  }
}
