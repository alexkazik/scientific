use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, null};
use core::slice::from_raw_parts;

#[derive(Clone, Copy)]
pub(crate) struct Ptr {
  pub(super) ptr: *const u8,
  start: *const u8,
  end: *const u8,
  writeable: bool,
}

impl Ptr {
  #[inline(always)]
  pub(crate) fn new(ptr: *const u8, len: isize) -> Ptr {
    let ptr = ptr;
    Ptr {
      ptr,
      start: ptr,
      end: unsafe { ptr.offset(len) },
      writeable: false,
    }
  }

  #[inline(always)]
  pub(crate) const fn new_const(slice: &[u8]) -> Ptr {
    let ptr = slice.as_ptr();
    Ptr {
      ptr,
      start: ptr,
      end: unsafe { ptr.add(slice.len()) },
      writeable: false,
    }
  }

  #[inline(always)]
  pub(crate) const fn new_invalid() -> Ptr {
    Ptr {
      ptr: null(),
      start: null(),
      end: null(),
      writeable: false,
    }
  }

  #[inline(always)]
  pub(crate) fn new_mut(ptr: *mut u8, len: isize) -> Ptr {
    let ptr = ptr;
    Ptr {
      ptr,
      start: ptr,
      end: unsafe { ptr.offset(len) },
      writeable: true,
    }
  }

  #[inline(always)]
  pub(crate) fn offset(self, count: isize) -> Ptr {
    Ptr {
      ptr: unsafe { self.ptr.offset(count) },
      start: self.start,
      end: self.end,
      writeable: self.writeable,
    }
  }

  pub(crate) fn copy_to_nonoverlapping(&self, len: isize, to: Ptr, offset: isize) {
    unsafe {
      if !(self.ptr >= self.start && self.ptr.offset(len) <= self.end) {
        panic!("Ptr: self out of bounds");
      }
      if !to.writeable {
        panic!("Ptr: write to const");
      }
      if !(to.ptr.offset(offset) >= to.start && to.ptr.offset(offset + len) <= to.end) {
        panic!("Ptr: to out of bounds");
      }
      copy_nonoverlapping(self.ptr, to.ptr.offset(offset) as *mut u8, len as usize)
    }
  }

  pub(crate) fn invalidate(&mut self) {
    // since ptr must be >= start and < end, this will always be invalid
    self.end = self.start;
  }

  pub(crate) fn set_mutable(&mut self) {
    self.writeable = true;
  }

  pub(crate) fn set_immutable(&mut self) {
    self.writeable = false;
  }

  pub(crate) fn as_slice(&self, len: isize) -> &[u8] {
    unsafe {
      if len < 0 || self.ptr < self.start || self.ptr.offset(len) > self.end {
        panic!("Ptr: out of bounds");
      }
      from_raw_parts(self.ptr.cast(), len as usize)
    }
  }

  pub(crate) fn offset_from(&self, other: Ptr) -> isize {
    if self.ptr < self.start
      || self.ptr > self.end
      || other.ptr < other.start
      || other.ptr > other.end
    {
      panic!("Ptr: out of bounds");
    }
    unsafe { self.ptr.offset_from(other.ptr) }
  }
}

impl Index<isize> for Ptr {
  type Output = i8;

  fn index(&self, index: isize) -> &Self::Output {
    unsafe {
      let ptr = self.ptr.offset(index);
      if !(ptr >= self.start && ptr < self.end) {
        panic!("Ptr: out of bounds");
      }
      (ptr as *const i8).as_ref().unwrap()
    }
  }
}

impl IndexMut<isize> for Ptr {
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    unsafe {
      if !self.writeable {
        panic!("Ptr: write to const");
      }
      let ptr = self.ptr.offset(index);
      if !(ptr >= self.start && ptr < self.end) {
        panic!("Ptr: out of bounds");
      }
      (ptr as *mut i8).as_mut().unwrap()
    }
  }
}
