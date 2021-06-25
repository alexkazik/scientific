use crate::Precision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Trimmer {
  Basic,
  Trim(Precision),
}
