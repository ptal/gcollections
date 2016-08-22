// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::SequenceKind;
use ops::multiset::Insert;

pub trait Empty {
  fn empty() -> Self;
}

pub trait Singleton<Item> {
  fn singleton(value: Item) -> Self;
}

impl<R, Item> Singleton<Item> for R where
 R: Empty + Insert<Item> + SequenceKind
{
  default fn singleton(value: Item) -> Self {
    let mut collection = R::empty();
    collection.insert(value);
    collection
  }
}
