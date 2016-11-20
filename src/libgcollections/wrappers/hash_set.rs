// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use std::collections::HashSet as StdHashSet;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::ops::{Deref, DerefMut};
use ops::*;

pub struct HashSet<T, S = RandomState>
{
  hs: StdHashSet<T, S>
}

impl<T, S> HashSet<T, S> where
  T: Eq + Hash,
  S: BuildHasher
{
  pub fn wrap(hs: StdHashSet<T, S>) -> HashSet<T, S> {
    HashSet{hs: hs}
  }
}

impl<T, S> Collection for HashSet<T, S> {
  type Item = T;
}

impl<T, S> Deref for HashSet<T, S>
{
  type Target = StdHashSet<T, S>;

  fn deref<'a>(&'a self) -> &'a StdHashSet<T, S> {
    &self.hs
  }
}

impl<T, S> DerefMut for HashSet<T, S>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut StdHashSet<T, S> {
    &mut self.hs
  }
}

impl<T, S> Contains for HashSet<T, S>
where T: Eq + Hash,
      S: BuildHasher
{
  contains_deref_impl!(T);
}

macro_rules! set_op_impl
{
  ( $( $t: ident, $m:ident );* ) =>
  {$(
    impl<T, S> $t for HashSet<T, S> where
     T: Eq + Hash + Clone,
     S: BuildHasher + Default
    {
      type Output = HashSet<T, S>;

      fn $m(&self, other: &HashSet<T, S>) -> HashSet<T, S> {
        HashSet::wrap(self.deref().$m(other).cloned().collect())
      }
    }
  )*}
}

set_op_impl! {
  Intersection, intersection;
  Union, union;
  Difference, difference;
  SymmetricDifference, symmetric_difference
}
