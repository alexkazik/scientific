use crate::types::scientific::Scientific;
use core::cmp::Ordering;

impl PartialOrd for Scientific {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Scientific {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    s_compare::<true>(self, other)
  }
}

impl PartialEq for Scientific {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl Eq for Scientific {}

pub(crate) fn s_compare<const USE_SIGN: bool>(lhs: &Scientific, rhs: &Scientific) -> Ordering {
  #[allow(clippy::collapsible_else_if)]
  if lhs.is_zero() && rhs.is_zero() {
    Ordering::Equal
  } else if lhs.is_zero() {
    if USE_SIGN && rhs.sign.is_negative() {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  } else if rhs.is_zero() {
    if USE_SIGN && lhs.sign.is_negative() {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  } else {
    if USE_SIGN && lhs.sign != rhs.sign {
      if lhs.sign.is_negative() {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    } else {
      match lhs.exponent0().cmp(&rhs.exponent0()) {
        Ordering::Equal => nz_compare_mantissa::<USE_SIGN>(lhs, rhs),
        ordering => {
          if USE_SIGN && lhs.sign.is_negative() {
            ordering.reverse()
          } else {
            ordering
          }
        }
      }
    }
  }
}

pub(crate) fn nz_compare_mantissa<const USE_SIGN: bool>(
  lhs: &Scientific,
  rhs: &Scientific,
) -> Ordering {
  let mut lhs_ptr = lhs.data;
  let mut rhs_ptr = rhs.data;
  let mut count = lhs.len.min(rhs.len);

  while count > 0 {
    let lhs_value = *lhs_ptr;
    let rhs_value = *rhs_ptr;
    if lhs_value != rhs_value {
      #[allow(clippy::collapsible_else_if)]
      return if lhs_value > rhs_value {
        if USE_SIGN && lhs.sign.is_negative() {
          Ordering::Less
        } else {
          Ordering::Greater
        }
      } else {
        if USE_SIGN && lhs.sign.is_negative() {
          Ordering::Greater
        } else {
          Ordering::Less
        }
      };
    }
    lhs_ptr.inc();
    rhs_ptr.inc();
    count -= 1;
  }

  let ordering = lhs.len.cmp(&rhs.len);
  if USE_SIGN && lhs.sign.is_negative() {
    ordering.reverse()
  } else {
    ordering
  }
}
