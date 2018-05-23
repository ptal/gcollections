// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::{One, Zero, Unsigned, Integer};

pub trait Cardinality
{
  type Size: Unsigned + Integer;
  fn size(&self) -> Self::Size;
}

pub trait IsSingleton
{
  fn is_singleton(&self) -> bool;
}

pub trait IsEmpty
{
  fn is_empty(&self) -> bool;
}

/// For an explanation on the macros, see `lib.rs`.

macro_rules! is_singleton_impl
{
  ( $( $keyword:tt ),*) =>
  {
    impl<R> IsSingleton for R where
     R: Cardinality
    {
      $($keyword)* fn is_singleton(&self) -> bool {
        self.size() == <<Self as Cardinality>::Size as One>::one()
      }
    }
  }
}

#[cfg(feature = "nightly")]
is_singleton_impl!(default);
#[cfg(not(feature = "nightly"))]
is_singleton_impl!();

macro_rules! is_empty_impl
{
  ( $( $keyword:tt ),*) =>
  {
    impl <R> IsEmpty for R where
     R: Cardinality
    {
      $($keyword)* fn is_empty(&self) -> bool {
        self.size().is_zero()
      }
    }
  }
}

#[cfg(feature = "nightly")]
is_empty_impl!(default);
#[cfg(not(feature = "nightly"))]
is_empty_impl!();
