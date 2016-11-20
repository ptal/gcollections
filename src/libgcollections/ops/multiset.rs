// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::constructor::*;
use ops::cardinality::*;

pub trait Insert: Collection {
  fn insert(&mut self, value: Self::Item);
}

pub trait Extract: Collection {
  fn extract(&mut self) -> Option<Self::Item>;
}

pub trait Multiset:
   Insert
 + Extract
 + IsEmpty
 + Empty
{}

impl<R> Multiset for R where
 R: Insert,
 R: Extract,
 R: IsEmpty,
 R: Empty
{}
