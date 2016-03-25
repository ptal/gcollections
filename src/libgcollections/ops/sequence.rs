// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod ordering {
  pub struct Back;
  pub struct Front;
}

pub trait Push<Order, Item> {
  fn push(&mut self, value: Item);
}

pub trait Pop<Order, Item> {
  fn pop(&mut self) -> Option<Item>;
}
