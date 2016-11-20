// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use bit_set::BitSet as StdBitSet;
use std::ops::{Deref, DerefMut};
use ops::*;

pub struct BitSet
{
  bs: StdBitSet
}

impl GroundType for BitSet {}

impl BitSet {
  pub fn wrap(bs: StdBitSet) -> BitSet {
    BitSet{bs: bs}
  }
}

impl Collection for BitSet {
  type Item = usize;
}

impl Deref for BitSet
{
  type Target = StdBitSet;

  fn deref<'a>(&'a self) -> &'a StdBitSet {
    &self.bs
  }
}

impl DerefMut for BitSet
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut StdBitSet {
    &mut self.bs
  }
}

impl Contains for BitSet
{
  contains_deref_impl!(Self::Item);
}

macro_rules! set_op_impl
{
  ( $( $t: ident, $m:ident, $v:ident );* ) =>
  {$(
    impl $t for BitSet {
      type Output = BitSet;

      fn $m(&self, other: &BitSet) -> BitSet {
        let mut new = self.deref().clone();
        new.$v(other);
        BitSet::wrap(new)
      }
    }
  )*}
}

set_op_impl! {
  Intersection, intersection, intersect_with;
  Union, union, union_with;
  Difference, difference, difference_with;
  SymmetricDifference, symmetric_difference, symmetric_difference_with
}
