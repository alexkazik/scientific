use crate::ptr::Ptr;
use crate::types::owner::Owner;
use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use crate::types::trimmer::Trimmer;
use crate::util::zeroes::s_mut_trim_zeroes;

pub(crate) struct Builder(Scientific);

impl Builder {
  pub(crate) fn new(sign: Sign, len: isize, exponent: isize) -> (Builder, Ptr) {
    #[cfg(feature = "debug")]
    assert!(len > 0);

    let mut vec = vec![0; len as usize];
    let data = vec.as_mut_ptr();
    (
      Builder(Scientific {
        sign,
        data: Ptr::new(data, len),
        len,
        exponent,
        owner: Owner::new_vec(vec),
      }),
      Ptr::new_mut(data, len),
    )
  }

  #[inline(always)]
  pub(crate) fn finish(mut self, trimmer: Trimmer) -> Scientific {
    s_mut_trim_zeroes(&mut self.0, trimmer);
    self.0
  }
}
