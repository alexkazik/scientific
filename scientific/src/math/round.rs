use crate::ptr::Ptr;
use crate::types::mantissa::MANTISSA_1;
use crate::types::owner::Owner;
use crate::types::precision::Precision;
use crate::types::rounding::Rounding;
use crate::types::sci::Sci;

impl Sci {
  pub(crate) fn round_assign(&mut self, precision: Precision, rounding: Rounding) {
    let len = match precision {
      Precision::Digits(digits) => digits,
      Precision::Decimals(decimals) => self.exponent0() + decimals,
    };
    if len < 0 {
      self.assign_zero();
    } else if len >= self.len {
      // more precision requested as available: just return the number
    } else if !rounding.round_away_from_zero(
      self.sign.is_negative(),
      if len == 0 { 0 } else { self.data[len - 1] },
      self.data[len],
      len + 1 == self.len,
    ) {
      // the rounding does result in no change
      self.truncate_assign(Precision::Digits(len));
    } else if len == 0 {
      // the new number should have 0 of the current digits but due to overflow one
      // is added in front
      self.exponent += self.len;
      self.len = 1;
      self.data = Ptr::new(MANTISSA_1.as_ptr(), 1);
      self.owner = Owner::None;
    } else {
      // adapt length (and exponent)
      self.exponent += self.len - len;
      self.len = len;

      let mut ptr = make_writeable(self);
      ptr.mut_offset(self.len - 1);

      while self.len > 0 && *ptr == 9 {
        self.len -= 1;
        self.exponent += 1;
        ptr.dec();
      }
      if self.len == 0 {
        // all digits where 9 and this is an overflow
        // replace mantissa with `1` and set exponent/len/owner accordingly
        self.len = 1;
        self.data = Ptr::new(MANTISSA_1.as_ptr(), 1);
        self.owner = Owner::None;
      } else {
        *ptr += 1;
        self.data.set_immutable();
      }
    }
  }
}

/// either ensures that the data is no longer accessed or copy it
fn make_writeable(sci: &mut Sci) -> Ptr {
  match sci.owner.make_writeable() {
    Ok(_) => {
      // it is guaranteed that there is no other access to the data
      // and thus we can modify it
    }
    Err(_) => {
      // copy the data from the old to the new owner
      let vec = sci.data.as_slice(sci.len).to_vec();
      sci.data = Ptr::new(vec.as_slice().as_ptr(), sci.len);
      sci.owner = Owner::new_vec(vec);
    }
  }

  let mut data = sci.data; // remainder: Ptr is Copy
  data.set_mutable();
  data
}
