#[cfg(not(feature = "arc"))]
use alloc::rc::Rc;
use alloc::string::String;
#[cfg(feature = "arc")]
use alloc::sync::Arc;
use alloc::vec::Vec;

#[cfg(not(feature = "arc"))]
#[derive(Clone)]
pub(crate) enum Owner {
  None,
  StringInternal(Rc<String>),
  VecInternal(Rc<Vec<u8>>),
}

#[cfg(feature = "arc")]
#[derive(Clone)]
pub(crate) enum Owner {
  None,
  StringInternal(Arc<String>),
  VecInternal(Arc<Vec<u8>>),
}

impl Owner {
  #[cfg(not(feature = "arc"))]
  #[inline(always)]
  pub(crate) fn new_string(data: String) -> Owner {
    Owner::StringInternal(Rc::new(data))
  }
  #[cfg(feature = "arc")]
  #[inline(always)]
  pub(crate) fn new_string(data: String) -> Owner {
    Owner::StringInternal(Arc::new(data))
  }

  #[cfg(not(feature = "arc"))]
  #[inline(always)]
  pub(crate) fn new_vec(data: Vec<u8>) -> Owner {
    Owner::VecInternal(Rc::new(data))
  }
  #[cfg(feature = "arc")]
  #[inline(always)]
  pub(crate) fn new_vec(data: Vec<u8>) -> Owner {
    Owner::VecInternal(Arc::new(data))
  }

  #[cfg(not(feature = "arc"))]
  #[inline]
  pub(crate) fn make_writeable(&mut self) -> Result<(), ()> {
    match self {
      Owner::None => Err(()),
      Owner::StringInternal(ref s) => {
        if Rc::strong_count(s) == 1 {
          Ok(())
        } else {
          Err(())
        }
      }
      Owner::VecInternal(ref v) => {
        if Rc::strong_count(v) == 1 {
          Ok(())
        } else {
          Err(())
        }
      }
    }
  }

  #[cfg(feature = "arc")]
  #[inline]
  pub(crate) fn make_writeable(&mut self) -> Result<(), ()> {
    match core::mem::replace(self, Owner::None) {
      Owner::None => Err(()),
      Owner::StringInternal(s) => match Arc::try_unwrap(s) {
        Ok(s) => {
          *self = Owner::new_string(s);
          Ok(())
        }
        Err(s) => {
          *self = Owner::StringInternal(s);
          Err(())
        }
      },
      Owner::VecInternal(v) => match Arc::try_unwrap(v) {
        Ok(v) => {
          *self = Owner::new_vec(v);
          Ok(())
        }
        Err(s) => {
          *self = Owner::VecInternal(s);
          Err(())
        }
      },
    }
  }
}
