// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;

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

// Membership
pub trait Contains : Collection {
  fn contains(&self, value: &Self::Item) -> bool;
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

macro_rules! contains_deref_impl {
  ($t:ty) => {
    fn contains(&self, value: &$t) -> bool {
      self.deref().contains(value)
    }
  }
}

