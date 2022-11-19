use core::ops::{Index, IndexMut};
use core::ptr::{copy_nonoverlapping, null, NonNull};
use core::slice::from_raw_parts;

#[derive(Clone, Copy)]
pub(crate) struct Ptr {
  pub(super) ptr: NonNull<u8>,
  start: *const u8,
  end: *const u8,
  writeable: bool,
}

impl Ptr {
  #[inline(always)]
  pub(crate) const fn new(slice: &[u8]) -> Ptr {
    let ptr = slice.as_ptr();
    Ptr {
      ptr: Self::new_ptr(ptr),
      start: ptr,
      end: unsafe { ptr.add(slice.len()) },
      writeable: false,
    }
  }

  #[inline(always)]
  pub(crate) const fn new_invalid() -> Ptr {
    Ptr {
      ptr: NonNull::dangling(),
      start: null(),
      end: null(),
      writeable: false,
    }
  }

  #[inline(always)]
  pub(crate) fn new_mut(slice: &mut [u8]) -> Ptr {
    let ptr = slice.as_mut_ptr();
    Ptr {
      ptr: Self::new_ptr(ptr),
      start: ptr,
      end: unsafe { ptr.add(slice.len()) },
      writeable: true,
    }
  }

  #[inline(always)]
  pub(crate) fn offset(self, count: isize) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(unsafe { self.ptr().offset(count) }),
      start: self.start,
      end: self.end,
      writeable: self.writeable,
    }
  }

  pub(crate) fn copy_to_nonoverlapping(&self, len: isize, to: Ptr, offset: isize) {
    unsafe {
      assert!(len >= 0, "Ptr: len is negative");
      assert!(
        self.ptr() >= self.start && self.ptr().offset(len) <= self.end,
        "Ptr: self out of bounds"
      );
      assert!(to.writeable, "Ptr: write to const");
      assert!(
        to.ptr().offset(offset) >= to.start && to.ptr().offset(offset + len) <= to.end,
        "Ptr: to out of bounds"
      );
      copy_nonoverlapping(self.ptr(), to.ptr_mut().offset(offset), len as usize);
    }
  }

  pub(crate) fn invalidate(&mut self) {
    // since ptr must be >= start and < end, this will always be invalid
    self.end = self.start;
  }

  pub(crate) fn as_mutable(mut self) -> Self {
    self.writeable = true;
    self
  }

  pub(crate) fn as_immutable(mut self) -> Self {
    self.writeable = false;
    self
  }

  pub(crate) fn as_slice(&self, len: isize) -> &[u8] {
    unsafe {
      assert!(len >= 0, "Ptr: len is negative");
      assert!(
        self.ptr() >= self.start && self.ptr().offset(len) <= self.end,
        "Ptr: out of bounds"
      );
      from_raw_parts(self.ptr(), len as usize)
    }
  }

  pub(crate) fn offset_from(&self, other: Ptr) -> isize {
    assert!(
      self.ptr() >= self.start
        && self.ptr() <= self.end
        && other.ptr() >= other.start
        && other.ptr() <= other.end,
      "Ptr: out of bounds"
    );
    unsafe { self.ptr().offset_from(other.ptr()) }
  }
}

impl Index<isize> for Ptr {
  type Output = i8;

  fn index(&self, index: isize) -> &Self::Output {
    unsafe {
      let ptr = self.ptr().offset(index);
      assert!(ptr >= self.start && ptr < self.end, "Ptr: out of bounds");
      ptr.cast::<i8>().as_ref().unwrap()
    }
  }
}

impl IndexMut<isize> for Ptr {
  fn index_mut(&mut self, index: isize) -> &mut Self::Output {
    unsafe {
      assert!(self.writeable, "Ptr: write to const");
      let ptr = self.ptr().offset(index);
      assert!(ptr >= self.start && ptr < self.end, "Ptr: out of bounds");
      ptr.cast::<i8>().cast_mut().as_mut().unwrap()
    }
  }
}
