// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ops::sequence::*;
use ops::sequence::ordering::*;

pub trait Queue<OrderPush, OrderPop, Item> :
    Push<OrderPush, Item>
  + Pop<OrderPop, Item>
{}

impl<R, Item> Queue<Back, Front, Item> for R where
 R: Push<Back, Item> + Pop<Front, Item>
{}

impl<R, Item> Queue<Front, Back, Item> for R where
 R: Push<Front, Item> + Pop<Back, Item>
{}
