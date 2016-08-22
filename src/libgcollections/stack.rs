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

pub struct Stack<S, Ordering, T>
{
  seq: S,
  phantom_ordering: PhantomData<Ordering>,
  phantom_t: PhantomData<T>
}

impl<S, Ordering, T> Stack<S, Ordering, T>
{
  pub fn wrap(seq: S) -> Self {
    Stack {
      seq: seq,
      phantom_ordering: PhantomData,
      phantom_t: PhantomData
    }
  }
}

impl<S, Ordering, T> Deref for Stack<S, Ordering, T>
{
  type Target = S;

  fn deref<'a>(&'a self) -> &'a S {
    &self.seq
  }
}

impl<S, Ordering, T> DerefMut for Stack<S, Ordering, T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut S {
    &mut self.seq
  }
}

impl<S, Ordering, T> Empty for Stack<S, Ordering, T> where
 S: Empty
{
  fn empty() -> Self {
    Stack::wrap(S::empty())
  }
}

impl<S, Ordering, T> Cardinality for Stack<S, Ordering, T> where
 S: Cardinality
{
  type Size = <S as Cardinality>::Size;
  fn size(&self) -> Self::Size {
    self.seq.size()
  }
}

impl<S, Ordering, T> Insert<T> for Stack<S, Ordering, T> where
 S: Push<Ordering, T>
{
  fn insert(&mut self, value: T) {
    self.push(value);
  }
}

impl<S, Ordering, T> Extract<T> for Stack<S, Ordering, T> where
 S: Pop<Ordering, T>
{
  fn extract(&mut self) -> Option<T> {
    self.pop()
  }
}
