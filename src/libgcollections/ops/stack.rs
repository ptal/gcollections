// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ops::sequence::*;

pub trait Stack<Order, Item> :
    Push<Order, Item>
  + Pop<Order, Item>
{}

impl<R, Order, Item> Stack<Order, Item> for R where
 R: Push<Order, Item> + Pop<Order, Item>
{}
