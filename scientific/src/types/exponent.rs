use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Deref, Sub};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Exponent(RelaxedExponent);
impl Exponent {
  pub(crate) const ZERO: Exponent = Exponent(RelaxedExponent(0));
  pub(crate) const ONE: Exponent = Exponent(RelaxedExponent(1));
  pub(crate) const NEG_ONE: Exponent = Exponent(RelaxedExponent(-1));

  pub(crate) const fn new(exponent: isize) -> Self {
    assert!(
      !(exponent < isize::MIN / 4 - 1 || exponent > isize::MAX / 4 + 1),
      "Exponent is out of range"
    );
    Self(RelaxedExponent(exponent))
  }

  pub(crate) fn modify<F>(&mut self, f: F)
  where
    F: FnOnce(&mut RelaxedExponent),
  {
    let mut exponent = self.0;
    f(&mut exponent);
    self.0 = Exponent::new(exponent.0).0;
  }
}

impl Display for Exponent {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0 .0.fmt(f)
  }
}

impl Deref for Exponent {
  type Target = RelaxedExponent;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add for Exponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: Self) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Add<isize> for Exponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: isize) -> Self::Output {
    self.0 + RelaxedExponent(rhs)
  }
}
impl Add<Exponent> for isize {
  type Output = RelaxedExponent;

  fn add(self, rhs: Exponent) -> Self::Output {
    RelaxedExponent(self) + rhs.0
  }
}

impl Sub for Exponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: Self) -> Self::Output {
    self.0 - rhs.0
  }
}
impl Sub<isize> for Exponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: isize) -> Self::Output {
    self.0 - RelaxedExponent(rhs)
  }
}
impl Sub<Exponent> for isize {
  type Output = RelaxedExponent;

  fn sub(self, rhs: Exponent) -> Self::Output {
    RelaxedExponent(self) - rhs.0
  }
}

impl PartialEq<isize> for Exponent {
  fn eq(&self, other: &isize) -> bool {
    self.0 .0.eq(other)
  }
}
impl PartialEq<Exponent> for isize {
  fn eq(&self, other: &Exponent) -> bool {
    self.eq(&other.0 .0)
  }
}
impl PartialOrd<isize> for Exponent {
  fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
    self.0 .0.partial_cmp(other)
  }
}
impl PartialOrd<Exponent> for isize {
  fn partial_cmp(&self, other: &Exponent) -> Option<Ordering> {
    self.partial_cmp(&other.0 .0)
  }
}

impl TryFrom<Exponent> for i8 {
  type Error = core::num::TryFromIntError;

  fn try_from(value: Exponent) -> Result<Self, Self::Error> {
    Self::try_from(value.0 .0)
  }
}
impl TryFrom<Exponent> for i16 {
  type Error = core::num::TryFromIntError;

  fn try_from(value: Exponent) -> Result<Self, Self::Error> {
    Self::try_from(value.0 .0)
  }
}
impl TryFrom<Exponent> for i32 {
  type Error = core::num::TryFromIntError;

  fn try_from(value: Exponent) -> Result<Self, Self::Error> {
    Self::try_from(value.0 .0)
  }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct RelaxedExponent(isize);

impl RelaxedExponent {
  pub(crate) fn into(self) -> Exponent {
    Exponent::new(self.0)
  }
}

impl Display for RelaxedExponent {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl Deref for RelaxedExponent {
  type Target = isize;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add for RelaxedExponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: Self) -> Self::Output {
    RelaxedExponent(self.0.checked_add(rhs.0).expect("Exponent overflow"))
  }
}
impl Add<isize> for RelaxedExponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: isize) -> Self::Output {
    RelaxedExponent(self.0.checked_add(rhs).expect("Exponent overflow"))
  }
}
impl Add<RelaxedExponent> for isize {
  type Output = RelaxedExponent;

  fn add(self, rhs: RelaxedExponent) -> Self::Output {
    RelaxedExponent(self.checked_add(rhs.0).expect("Exponent overflow"))
  }
}
impl Add<Exponent> for RelaxedExponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: Exponent) -> Self::Output {
    RelaxedExponent(self.0.checked_add(rhs.0 .0).expect("Exponent overflow"))
  }
}
impl Add<RelaxedExponent> for Exponent {
  type Output = RelaxedExponent;

  fn add(self, rhs: RelaxedExponent) -> Self::Output {
    RelaxedExponent(self.0 .0.checked_add(rhs.0).expect("Exponent overflow"))
  }
}

impl AddAssign<isize> for RelaxedExponent {
  fn add_assign(&mut self, rhs: isize) {
    self.0 = self.0.checked_add(rhs).expect("Exponent overflow");
  }
}

impl Sub for RelaxedExponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: Self) -> Self::Output {
    RelaxedExponent(self.0.checked_sub(rhs.0).expect("Exponent overflow"))
  }
}
impl Sub<isize> for RelaxedExponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: isize) -> Self::Output {
    RelaxedExponent(self.0.checked_sub(rhs).expect("Exponent overflow"))
  }
}
impl Sub<RelaxedExponent> for isize {
  type Output = RelaxedExponent;

  fn sub(self, rhs: RelaxedExponent) -> Self::Output {
    RelaxedExponent(self.checked_sub(rhs.0).expect("Exponent overflow"))
  }
}
impl Sub<Exponent> for RelaxedExponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: Exponent) -> Self::Output {
    RelaxedExponent(self.0.checked_sub(rhs.0 .0).expect("Exponent overflow"))
  }
}
impl Sub<RelaxedExponent> for Exponent {
  type Output = RelaxedExponent;

  fn sub(self, rhs: RelaxedExponent) -> Self::Output {
    RelaxedExponent(self.0 .0.checked_sub(rhs.0).expect("Exponent overflow"))
  }
}

impl PartialEq<isize> for RelaxedExponent {
  fn eq(&self, other: &isize) -> bool {
    self.0.eq(other)
  }
}
impl PartialEq<RelaxedExponent> for isize {
  fn eq(&self, other: &RelaxedExponent) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<isize> for RelaxedExponent {
  fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
impl PartialOrd<RelaxedExponent> for isize {
  fn partial_cmp(&self, other: &RelaxedExponent) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }
}

impl PartialEq<Exponent> for RelaxedExponent {
  fn eq(&self, other: &Exponent) -> bool {
    self.0.eq(&other.0 .0)
  }
}
impl PartialEq<RelaxedExponent> for Exponent {
  fn eq(&self, other: &RelaxedExponent) -> bool {
    self.0 .0.eq(&other.0)
  }
}
impl PartialOrd<Exponent> for RelaxedExponent {
  fn partial_cmp(&self, other: &Exponent) -> Option<Ordering> {
    self.0.partial_cmp(&other.0 .0)
  }
}
impl PartialOrd<RelaxedExponent> for Exponent {
  fn partial_cmp(&self, other: &RelaxedExponent) -> Option<Ordering> {
    self.0 .0.partial_cmp(&other.0)
  }
}
