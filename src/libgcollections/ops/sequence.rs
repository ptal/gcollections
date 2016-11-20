// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::constructor::*;
use ops::cardinality::*;

pub mod ordering {
  pub struct Back;
  pub struct Front;
}

pub trait Push<Order> : Collection {
  fn push(&mut self, value: Self::Item);
}

pub trait Pop<Order> : Collection {
  fn pop(&mut self) -> Option<Self::Item>;
}

pub trait Sequence<OrderPush, OrderPop> :
   Push<OrderPush>
 + Pop<OrderPop>
 + Cardinality
 + Empty
{}

impl<R, OrderPush, OrderPop> Sequence<OrderPush, OrderPop> for R where
 R: Push<OrderPush>,
 R: Pop<OrderPop>,
 R: Cardinality,
 R: Empty
{}
