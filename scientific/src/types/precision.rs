#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Precision {
  Digits(isize),
  Decimals(isize),
}

impl Precision {
  pub const INTEGER: Precision = Precision::Decimals(0);
  pub const F64: Precision = Precision::Digits(16);
}

impl Default for Precision {
  fn default() -> Self {
    Precision::INTEGER
  }
}
