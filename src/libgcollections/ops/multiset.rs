// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ops::constructor::*;
use ops::cardinality::*;

pub trait Insert<Item> {
  fn insert(&mut self, value: Item);
}

pub trait Extract<Item> {
  fn extract(&mut self) -> Option<Item>;
}

pub trait Multiset<Item> :
   Insert<Item>
 + Extract<Item>
 + IsEmpty
 + Empty
{}

impl<R, Item> Multiset<Item> for R where
 R: Insert<Item>,
 R: Extract<Item>,
 R: IsEmpty,
 R: Empty
{}
