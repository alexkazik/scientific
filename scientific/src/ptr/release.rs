use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, NonNull};
use core::slice::from_raw_parts;

#[derive(Clone, Copy)]
pub(crate) struct Ptr {
  pub(super) ptr: NonNull<u8>,
}

impl Ptr {
  #[inline(always)]
  pub(crate) const fn new(slice: &[u8]) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(slice.as_ptr()),
    }
  }

  #[inline(always)]
  pub(crate) fn new_mut(slice: &mut [u8]) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(slice.as_ptr()),
    }
  }

  #[inline(always)]
  pub(crate) const fn new_invalid() -> Ptr {
    Ptr {
      ptr: NonNull::dangling(),
    }
  }

  #[inline(always)]
  pub(crate) fn offset(self, count: isize) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(unsafe { self.ptr().offset(count) }),
    }
  }

  #[inline(always)]
  pub(crate) fn copy_to_nonoverlapping(&self, len: isize, to: Ptr, offset: isize) {
    unsafe { copy_nonoverlapping(self.ptr(), to.ptr_mut().offset(offset), len as usize) }
  }

  #[allow(clippy::unused_self)]
  #[inline(always)]
  pub(crate) fn invalidate(&mut self) {}

  #[allow(clippy::unused_self)]
  #[inline(always)]
  pub(crate) fn as_mutable(self) -> Self {
    self
  }

  #[allow(clippy::unused_self)]
  #[inline(always)]
  pub(crate) fn as_immutable(self) -> Self {
    self
  }

  #[inline(always)]
  pub(crate) fn as_slice(&self, len: isize) -> &[u8] {
    unsafe { from_raw_parts(self.ptr(), len as usize) }
  }

  #[inline(always)]
  pub(crate) fn offset_from(&self, other: Ptr) -> isize {
    unsafe { self.ptr().offset_from(other.ptr()) }
  }
}

impl Index<isize> for Ptr {
  type Output = i8;

  #[inline(always)]
  fn index(&self, index: isize) -> &Self::Output {
    unsafe { self.ptr().cast::<i8>().offset(index).as_ref().unwrap() }
  }
}

impl IndexMut<isize> for Ptr {
  #[inline(always)]
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    unsafe { self.ptr_mut().cast::<i8>().offset(index).as_mut().unwrap() }
  }
}
