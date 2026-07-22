use crate::types::limited::{Length, Limited, ToIsize};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

pub(crate) trait UncheckedFromIsize: ToIsize {
  fn from_isize_unchecked(value: isize) -> Self;
}
pub(crate) trait RangeTo<Rhs> {
  // The output must be the bigger of Self,Rhs
  type Output: UncheckedFromIsize;
  fn range_to(self, rhs: Rhs) -> RangeToIter<Self::Output>;
}

impl<const N: usize> RangeTo<Limited<N>> for i8 {
  type Output = Limited<N>;

  fn range_to(self, rhs: Limited<N>) -> RangeToIter<Self::Output> {
    RangeToIter::new(isize::from(self), rhs)
  }
}

impl<const N: usize> RangeTo<i8> for Limited<N> {
  type Output = Limited<N>;

  fn range_to(self, rhs: i8) -> RangeToIter<Self::Output> {
    RangeToIter::new(self, isize::from(rhs))
  }
}

impl<const N: usize> RangeTo<Limited<N>> for Length {
  type Output = Limited<N>;

  fn range_to(self, rhs: Limited<N>) -> RangeToIter<Self::Output> {
    RangeToIter::new(self, rhs)
  }
}

impl<const N: usize> RangeTo<Length> for Limited<N> {
  type Output = Limited<N>;

  fn range_to(self, rhs: Length) -> RangeToIter<Self::Output> {
    RangeToIter::new(self, rhs)
  }
}

impl RangeTo<Limited<2>> for Limited<2> {
  type Output = Limited<2>;

  fn range_to(self, rhs: Limited<2>) -> RangeToIter<Self::Output> {
    RangeToIter::new(self, rhs)
  }
}

impl RangeTo<Length> for i8 {
  type Output = Length;

  fn range_to(self, rhs: Length) -> RangeToIter<Self::Output> {
    RangeToIter::new(isize::from(self), rhs)
  }
}

impl RangeTo<i8> for Length {
  type Output = Length;

  fn range_to(self, rhs: i8) -> RangeToIter<Self::Output> {
    RangeToIter::new(self, isize::from(rhs))
  }
}

pub(crate) struct RangeToIter<T: UncheckedFromIsize> {
  range: Range<isize>,
  phantom_data: PhantomData<T>,
}

impl<T: UncheckedFromIsize> RangeToIter<T> {
  fn new<S, E>(start: S, end: E) -> Self
  where
    S: ToIsize,
    E: ToIsize,
  {
    Self {
      range: start.to_isize()..end.to_isize(),
      phantom_data: PhantomData,
    }
  }
}

impl<T: UncheckedFromIsize> Iterator for RangeToIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.range.next().map(|v| T::from_isize_unchecked(v))
  }
}
impl<T: UncheckedFromIsize> ExactSizeIterator for RangeToIter<T> {}
impl<T: UncheckedFromIsize> FusedIterator for RangeToIter<T> {}
