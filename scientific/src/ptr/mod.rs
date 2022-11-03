#[cfg(feature = "debug")]
pub(crate) use crate::ptr::debug::Ptr;
#[cfg(not(feature = "debug"))]
pub(crate) use crate::ptr::release::Ptr;
use core::cmp::Ordering;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::NonNull;

#[cfg(feature = "debug")]
pub(crate) mod debug;
#[cfg(not(feature = "debug"))]
pub(crate) mod release;

// common stuff

impl Ptr {
  #[inline(always)]
  pub(crate) const fn ptr(&self) -> *const u8 {
    self.ptr.as_ptr()
  }

  #[inline(always)]
  pub(crate) const fn ptr_mut(&self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  #[inline(always)]
  pub(crate) fn set_ptr(&mut self, ptr: *const u8) {
    self.ptr = Self::new_ptr(ptr);
  }

  #[inline(always)]
  pub(crate) const fn new_ptr(ptr: *const u8) -> NonNull<u8> {
    unsafe { NonNull::new_unchecked(ptr as *mut u8) }
  }

  #[inline(always)]
  pub(crate) fn mut_offset(&mut self, count: isize) {
    self.set_ptr(unsafe { self.ptr().offset(count) });
  }

  #[inline(always)]
  pub(crate) fn inc(&mut self) {
    self.set_ptr(unsafe { self.ptr().add(1) });
  }

  #[inline(always)]
  pub(crate) fn dec(&mut self) {
    self.set_ptr(unsafe { self.ptr().sub(1) });
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
