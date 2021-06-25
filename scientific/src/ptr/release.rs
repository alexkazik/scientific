use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, null};
use core::slice::from_raw_parts;

#[derive(Clone, Copy)]
pub(crate) struct Ptr {
  pub(super) ptr: *const u8,
}

impl Ptr {
  #[inline(always)]
  pub(crate) fn new(ptr: *const u8, _len: isize) -> Ptr {
    Ptr { ptr }
  }

  #[inline(always)]
  pub(crate) const fn new_const(slice: &[u8]) -> Ptr {
    Ptr {
      ptr: slice.as_ptr(),
    }
  }

  #[inline(always)]
  pub(crate) fn new_mut(ptr: *mut u8, _len: isize) -> Ptr {
    Ptr { ptr }
  }

  #[inline(always)]
  pub(crate) const fn new_invalid() -> Ptr {
    Ptr { ptr: null() }
  }

  #[inline(always)]
  pub(crate) fn offset(self, count: isize) -> Ptr {
    Ptr {
      ptr: unsafe { self.ptr.offset(count) },
    }
  }

  #[inline(always)]
  pub(crate) fn copy_to_nonoverlapping(&self, len: isize, to: Ptr, offset: isize) {
    unsafe { copy_nonoverlapping(self.ptr, to.ptr.offset(offset) as *mut u8, len as usize) }
  }

  #[inline(always)]
  pub(crate) fn invalidate(&mut self) {}

  #[inline(always)]
  pub(crate) fn set_immutable(&mut self) {}

  #[inline(always)]
  pub(crate) fn as_slice(&self, len: isize) -> &[u8] {
    unsafe { from_raw_parts(self.ptr.cast(), len as usize) }
  }

  #[inline(always)]
  pub(crate) fn offset_from(&self, other: Ptr) -> isize {
    unsafe { self.ptr.offset_from(other.ptr) }
  }
}

impl Index<isize> for Ptr {
  type Output = i8;

  #[inline(always)]
  fn index(&self, index: isize) -> &Self::Output {
    unsafe { (self.ptr as *const i8).offset(index).as_ref().unwrap() }
  }
}

impl IndexMut<isize> for Ptr {
  #[inline(always)]
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    unsafe { (self.ptr as *mut i8).offset(index).as_mut().unwrap() }
  }
}
