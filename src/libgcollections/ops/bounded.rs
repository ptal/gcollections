// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait Bounded
{
  type Bound: PartialOrd;
  fn lower(&self) -> Self::Bound;
  fn upper(&self) -> Self::Bound;
}

pub trait ShrinkLeft<Bound> {
  fn shrink_left(&self, lb: Bound) -> Self;
}

pub trait ShrinkRight<Bound> {
  fn shrink_right(&self, ub: Bound) -> Self;
}

pub trait StrictShrinkLeft<Bound> {
  fn strict_shrink_left(&self, lb: Bound) -> Self;
}

pub trait StrictShrinkRight<Bound> {
  fn strict_shrink_right(&self, ub: Bound) -> Self;
}

