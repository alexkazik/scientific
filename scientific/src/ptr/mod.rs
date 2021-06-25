#[cfg(feature = "debug")]
mod debug;
#[cfg(not(feature = "debug"))]
mod release;

#[cfg(feature = "debug")]
pub(crate) use crate::ptr::debug::Ptr;
#[cfg(not(feature = "debug"))]
pub(crate) use crate::ptr::release::Ptr;
use core::cmp::Ordering;
use core::ops::{Deref, DerefMut, Index, IndexMut};

// common stuff

impl Ptr {
  #[inline(always)]
  pub(crate) fn mut_offset(&mut self, count: isize) {
    self.ptr = unsafe { self.ptr.offset(count) };
  }

  #[inline(always)]
  pub(crate) fn inc(&mut self) {
    self.ptr = unsafe { self.ptr.add(1) };
  }

  #[inline(always)]
  pub(crate) fn dec(&mut self) {
    self.ptr = unsafe { self.ptr.sub(1) };
  }
}

impl Deref for Ptr {
  type Target = i8;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.index(0)
  }
}

impl DerefMut for Ptr {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.index_mut(0)
  }
}

impl PartialEq for Ptr {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.ptr == other.ptr
  }
}

impl Eq for Ptr {}

impl PartialOrd for Ptr {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }

  #[inline(always)]
  fn lt(&self, other: &Self) -> bool {
    self.ptr.lt(&other.ptr)
  }

  #[inline(always)]
  fn le(&self, other: &Self) -> bool {
    self.ptr.le(&other.ptr)
  }

  #[inline(always)]
  fn gt(&self, other: &Self) -> bool {
    self.ptr.gt(&other.ptr)
  }

  #[inline(always)]
  fn ge(&self, other: &Self) -> bool {
    self.ptr.ge(&other.ptr)
  }
}

impl Ord for Ptr {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    self.ptr.cmp(&other.ptr)
  }
}
