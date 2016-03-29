// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
