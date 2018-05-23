// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::multiset::Insert;

pub trait Empty {
  fn empty() -> Self;
}

pub trait Singleton : Collection {
  fn singleton(value: Self::Item) -> Self;
}

macro_rules! singleton_impl
{
  ( $( $keyword:tt ),*) =>
  {
    impl<R> Singleton for R where
     R: Empty + Insert + Collection + SequenceKind
    {
      $($keyword)* fn singleton(value: Self::Item) -> Self {
        let mut collection = R::empty();
        collection.insert(value);
        collection
      }
    }

  }
}

#[cfg(feature = "nightly")]
singleton_impl!(default);
#[cfg(not(feature = "nightly"))]
singleton_impl!();
