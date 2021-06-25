#[cfg(not(any(doc, feature = "arc")))]
use alloc::rc::Rc;
use alloc::string::String;
#[cfg(any(doc, feature = "arc"))]
use alloc::sync::Arc;
use alloc::vec::Vec;

#[cfg(not(any(doc, feature = "arc")))]
#[derive(Clone)]
pub(crate) enum Owner {
  None,
  StringInternal(Rc<String>),
  VecInternal(Rc<Vec<u8>>),
}

#[cfg(any(doc, feature = "arc"))]
#[derive(Clone)]
pub(crate) enum Owner {
  None,
  StringInternal(Arc<String>),
  VecInternal(Arc<Vec<u8>>),
}

impl Owner {
  #[cfg(not(any(doc, feature = "arc")))]
  #[inline(always)]
  pub(crate) fn new_string(data: String) -> Owner {
    Owner::StringInternal(Rc::new(data))
  }
  #[cfg(feature = "arc")]
  #[inline(always)]
  pub(crate) fn new_string(data: String) -> Owner {
    Owner::StringInternal(Arc::new(data))
  }

  #[cfg(not(any(doc, feature = "arc")))]
  #[inline(always)]
  pub(crate) fn new_vec(data: Vec<u8>) -> Owner {
    Owner::VecInternal(Rc::new(data))
  }
  #[cfg(feature = "arc")]
  #[inline(always)]
  pub(crate) fn new_vec(data: Vec<u8>) -> Owner {
    Owner::VecInternal(Arc::new(data))
  }

  #[inline]
  pub(crate) fn capacity(&self) -> isize {
    match &self {
      Owner::None => 0,
      Owner::StringInternal(s) => s.capacity() as isize,
      Owner::VecInternal(v) => v.capacity() as isize,
    }
  }
}
