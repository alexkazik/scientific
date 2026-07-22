use crate::types::limited::{
  Length, LengthOutOfRangeError, RangeToIter, ToIsize, UncheckedFromIsize,
};
use core::fmt::Write;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::ptr::{copy_nonoverlapping, NonNull};
use core::slice::from_raw_parts;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct Ptr {
  ptr: NonNull<u8>,
}

impl Ptr {
  #[inline]
  const fn new_ptr(ptr: *const u8) -> NonNull<u8> {
    unsafe { NonNull::new_unchecked(ptr.cast_mut()) }
  }

  #[inline]
  pub(crate) const fn new(slice: &[u8]) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(slice.as_ptr()),
    }
  }

  #[inline]
  pub(crate) fn new_mut(slice: &mut [u8]) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(slice.as_mut_ptr()),
    }
  }

  #[inline]
  pub(crate) const fn new_invalid() -> Ptr {
    Ptr {
      ptr: NonNull::dangling(),
    }
  }

  #[inline]
  pub(crate) fn offset<T: ToIsize>(self, count: T) -> Ptr {
    Ptr {
      ptr: Self::new_ptr(unsafe { self.ptr.as_ptr().offset(count.to_isize()) }),
    }
  }

  #[inline]
  pub(crate) fn copy_to_nonoverlapping(self, len: Length, to: Ptr, offset: isize) {
    unsafe {
      copy_nonoverlapping(
        self.ptr.as_ptr(),
        to.ptr.as_ptr().offset(offset),
        len.to_usize(),
      );
    }
  }

  #[inline]
  pub(crate) fn as_slice(&self, len: Length) -> &[u8] {
    unsafe { from_raw_parts(self.ptr.as_ptr(), len.to_usize()) }
  }

  #[inline]
  pub(crate) fn try_offset_from(self, other: Ptr) -> Result<Length, LengthOutOfRangeError> {
    Length::try_new(unsafe { self.ptr.as_ptr().offset_from(other.ptr.as_ptr()) })
  }

  #[inline]
  pub(crate) fn inc(&mut self) {
    self.ptr = Self::new_ptr(unsafe { self.ptr.as_ptr().add(1) });
  }

  #[inline]
  pub(crate) fn dec(&mut self) {
    self.ptr = Self::new_ptr(unsafe { self.ptr.as_ptr().sub(1) });
  }

  #[inline]
  pub(crate) fn write_first_char<W: Write>(self, f: &mut W) -> Result<(), core::fmt::Error> {
    f.write_char((b'0' + (*self as u8)).into())
  }

  #[inline]
  pub(crate) fn write_chars<W: Write, T: UncheckedFromIsize>(
    self,
    f: &mut W,
    range: RangeToIter<T>,
  ) -> Result<(), core::fmt::Error> {
    for i in range {
      f.write_char((b'0' + (self[i] as u8)).into())?;
    }
    Ok(())
  }
}

impl<T: ToIsize> Index<T> for Ptr {
  type Output = i8;

  #[inline]
  fn index(&self, index: T) -> &Self::Output {
    unsafe { &*self.ptr.as_ptr().cast::<i8>().offset(index.to_isize()) }
  }
}

impl<T: ToIsize> IndexMut<T> for Ptr {
  #[inline]
  fn index_mut(&mut self, index: T) -> &mut Self::Output {
    unsafe { &mut *self.ptr.as_ptr().cast::<i8>().offset(index.to_isize()) }
  }
}

impl Deref for Ptr {
  type Target = i8;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.index(0)
  }
}

impl DerefMut for Ptr {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.index_mut(0)
  }
}
