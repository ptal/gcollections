// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use wrappers::{HashSet, BTreeSet, BitSet, EnumSet};
use collections::enum_set::CLike;
use std::hash::{Hash, BuildHasher};
use std::iter::FromIterator;
use std::default::Default;
use std::ops::Deref;

// Basic set operations
pub trait Intersection<RHS = Self> {
  type Output;
  fn intersection(&self, rhs: &RHS) -> Self::Output;
}

pub trait Union<RHS = Self> {
  type Output;
  fn union(&self, rhs: &RHS) -> Self::Output;
}

pub trait Difference<RHS = Self> {
  type Output;
  fn difference(&self, rhs: &RHS) -> Self::Output;
}

pub trait SymmetricDifference<RHS = Self> {
  type Output;
  fn symmetric_difference(&self, rhs: &RHS) -> Self::Output;
}

pub trait Complement {
  fn complement(&self) -> Self;
}

macro_rules! set_op_impl
{
  ( $( $t: ident, $m:ident, $v:ident );* ) =>
  {$(
    impl<T> $t for BTreeSet<T>
    where T: Ord+Clone
    {
      type Output = BTreeSet<T>;

      fn $m(&self, other: &BTreeSet<T>) -> BTreeSet<T> {
        BTreeSet::wrap(FromIterator::from_iter(self.deref().$m(other).cloned()))
      }
    }

    impl $t for BitSet {
      type Output = BitSet;

      fn $m(&self, other: &BitSet) -> BitSet {
        let mut new = self.deref().clone();
        new.$v(other);
        BitSet::wrap(new)
      }
    }

    impl<T, S> $t for HashSet<T, S>
    where T: Eq + Hash + Clone,
          S: BuildHasher + Default
    {
      type Output = HashSet<T, S>;

      fn $m(&self, other: &HashSet<T, S>) -> HashSet<T, S> {
        HashSet::wrap(FromIterator::from_iter(self.deref().$m(other).cloned()))
      }
    }
  )*}
}

macro_rules! set_enum_op_impl
{
  ( $( $t: ident, $m:ident, $v:ident );* ) =>
  {$(
    set_op_impl! {$t, $m, $v}

    impl<E: CLike> $t for EnumSet<E>
    {
      type Output = EnumSet<E>;

      fn $m(&self, other: &EnumSet<E>) -> EnumSet<E> {
        EnumSet::wrap(self.deref().$m(**other))
      }
    }
  )*}
}

set_enum_op_impl! {
  Intersection, intersection, intersect_with;
  Union, union, union_with
}

set_op_impl! {
  Difference, difference, difference_with;
  SymmetricDifference, symmetric_difference, symmetric_difference_with
}
