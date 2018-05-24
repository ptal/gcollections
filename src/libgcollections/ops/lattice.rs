// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A [lattice](https://en.wikipedia.org/wiki/Lattice_(order)) is an ordered set where every element can be combined with a `join` and `meet` operations.
/// However, the following family of traits do not represent the full lattice, but an element of this lattice.
/// This is the reason why we do not require `Lattice` to inherit from `Collection`.

use trilean::SKleene;

pub trait Join<RHS: ?Sized = Self>
{
  fn join(self, other: RHS) -> Self;
  fn join_in_place(&mut self, other: RHS);
}

pub trait Meet<RHS: ?Sized = Self>
{
  fn meet(self, other: RHS) -> Self;
  fn meet_in_place(&mut self, other: RHS);
}

pub trait Entailment<RHS: ?Sized = Self>
{
  fn entail(&self, other: &RHS) -> SKleene;
}

pub trait Lattice:
   Join
 + Meet
 + Entailment
{}

impl<R> Lattice for R where
 R: Join,
 R: Meet,
 R: Entailment
{}
