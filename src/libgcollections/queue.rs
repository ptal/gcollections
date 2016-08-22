// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ops::constructor::*;
use ops::cardinality::*;
use ops::sequence::*;
use ops::multiset::*;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub struct Queue<S, OrdPush, OrdPop, T>
{
  seq: S,
  phantom_ord_push: PhantomData<OrdPush>,
  phantom_ord_pop: PhantomData<OrdPop>,
  phantom_t: PhantomData<T>
}

impl<S, OrdPush, OrdPop, T> Queue<S, OrdPush, OrdPop, T>
{
  pub fn wrap(seq: S) -> Self {
    Queue {
      seq: seq,
      phantom_ord_push: PhantomData,
      phantom_ord_pop: PhantomData,
      phantom_t: PhantomData
    }
  }
}

impl<S, OrdPush, OrdPop, T> Deref for Queue<S, OrdPush, OrdPop, T>
{
  type Target = S;

  fn deref<'a>(&'a self) -> &'a S {
    &self.seq
  }
}

impl<S, OrdPush, OrdPop, T> DerefMut for Queue<S, OrdPush, OrdPop, T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut S {
    &mut self.seq
  }
}

impl<S, OrdPush, OrdPop, T> Empty for Queue<S, OrdPush, OrdPop, T> where
 S: Empty
{
  fn empty() -> Self {
    Queue::wrap(S::empty())
  }
}

impl<S, OrdPush, OrdPop, T> Cardinality for Queue<S, OrdPush, OrdPop, T> where
 S: Cardinality
{
  type Size = <S as Cardinality>::Size;
  fn size(&self) -> Self::Size {
    self.seq.size()
  }
}

impl<S, OrdPush, OrdPop, T> Insert<T> for Queue<S, OrdPush, OrdPop, T> where
 S: Push<OrdPush, T>
{
  fn insert(&mut self, value: T) {
    self.push(value);
  }
}

impl<S, OrdPush, OrdPop, T> Extract<T> for Queue<S, OrdPush, OrdPop, T> where
 S: Pop<OrdPop, T>
{
  fn extract(&mut self) -> Option<T> {
    self.pop()
  }
}
