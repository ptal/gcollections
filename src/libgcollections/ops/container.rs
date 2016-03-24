// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use wrappers::{HashSet, BTreeSet, BitSet, EnumSet};
use collections::enum_set::CLike;
use std::hash::{Hash, BuildHasher};
use std::ops::Deref;

// Membership
pub trait Contains<Item> {
  fn contains(&self, value: &Item) -> bool;
}

macro_rules! contains_impl {
  ($t:ty) => {
    fn contains(&self, value: &$t) -> bool {
      self.deref().contains(value)
    }
  }
}

impl<T, S> Contains<T> for HashSet<T, S>
where T: Eq + Hash,
      S: BuildHasher
{
  contains_impl!(T);
}

impl<T: Ord> Contains<T> for BTreeSet<T> {
  contains_impl!(T);
}

impl Contains<usize> for BitSet {
  contains_impl!(usize);
}

impl<E: CLike> Contains<E> for EnumSet<E> {
  contains_impl!(E);
}

pub trait Disjoint<RHS = Self> {
  fn is_disjoint(&self, rhs: &RHS) -> bool;
}

pub trait Subset<RHS = Self> {
  fn is_subset(&self, rhs: &RHS) -> bool;
}

pub trait ProperSubset<RHS = Self> {
  fn is_proper_subset(&self, rhs: &RHS) -> bool;
}

pub trait Overlap<RHS = Self> {
  fn overlap(&self, rhs: &RHS) -> bool;
}


