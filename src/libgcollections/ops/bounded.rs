// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::constructor::*;
use num_integer::Integer;

pub trait Bounded: Collection
{
  fn lower(&self) -> Self::Item;
  fn upper(&self) -> Self::Item;
}

pub trait ShrinkLeft: Bounded {
  fn shrink_left(&self, lb: Self::Item) -> Self;
}

pub trait ShrinkRight: Bounded {
  fn shrink_right(&self, ub: Self::Item) -> Self;
}

pub trait StrictShrinkLeft: Bounded {
  fn strict_shrink_left(&self, lb: Self::Item) -> Self;
}

pub trait StrictShrinkRight: Bounded {
  fn strict_shrink_right(&self, ub: Self::Item) -> Self;
}

macro_rules! strict_shrink_impl
{
  ( $( $keyword:tt ),*) =>
  {
    impl<B, R> StrictShrinkLeft for R where
      R: ShrinkLeft + Empty + IntervalKind + Bounded<Item=B>,
      B: Integer + num_traits::Bounded
    {
      $($keyword)* fn strict_shrink_left(&self, lb: B) -> R {
        if lb == B::max_value() {
          R::empty()
        } else {
          self.shrink_left(lb + B::one())
        }
      }
    }
    impl<B, R> StrictShrinkRight for R where
      R: ShrinkRight + Empty + IntervalKind + Bounded<Item=B>,
      B: Integer + num_traits::Bounded
    {
      $($keyword)* fn strict_shrink_right(&self, ub: B) -> R {
        if ub == B::min_value() {
          R::empty()
        } else {
          self.shrink_right(ub - B::one())
        }
      }
    }
  }
}

#[cfg(feature = "nightly")]
strict_shrink_impl!(default);
#[cfg(not(feature = "nightly"))]
strict_shrink_impl!();
