// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::constructor::*;
use ops::cardinality::*;
use ops::sequence::*;
use ops::multiset::*;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub struct Queue<S, OrdPush, OrdPop>
{
  seq: S,
  phantom_ord_push: PhantomData<OrdPush>,
  phantom_ord_pop: PhantomData<OrdPop>
}

impl<S, OrdPush, OrdPop> Queue<S, OrdPush, OrdPop>
{
  pub fn wrap(seq: S) -> Self {
    Queue {
      seq: seq,
      phantom_ord_push: PhantomData,
      phantom_ord_pop: PhantomData
    }
  }
}

impl<S, OrdPush, OrdPop> Collection for Queue<S, OrdPush, OrdPop> where
  S: Collection
{
  type Item = <S as Collection>::Item;
}

impl<S, OrdPush, OrdPop> Deref for Queue<S, OrdPush, OrdPop>
{
  type Target = S;

  fn deref<'a>(&'a self) -> &'a S {
    &self.seq
  }
}

impl<S, OrdPush, OrdPop> DerefMut for Queue<S, OrdPush, OrdPop>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut S {
    &mut self.seq
  }
}

impl<S, OrdPush, OrdPop> Empty for Queue<S, OrdPush, OrdPop> where
 S: Empty
{
  fn empty() -> Self {
    Queue::wrap(S::empty())
  }
}

impl<S, OrdPush, OrdPop> Cardinality for Queue<S, OrdPush, OrdPop> where
 S: Cardinality
{
  type Size = <S as Cardinality>::Size;
  fn size(&self) -> Self::Size {
    self.seq.size()
  }
}

impl<S, OrdPush, OrdPop> Insert for Queue<S, OrdPush, OrdPop> where
 S: Push<OrdPush>
{
  fn insert(&mut self, value: Self::Item) {
    self.push(value);
  }
}

impl<S, OrdPush, OrdPop> Extract for Queue<S, OrdPush, OrdPop> where
 S: Pop<OrdPop>
{
  fn extract(&mut self) -> Option<Self::Item> {
    self.pop()
  }
}
