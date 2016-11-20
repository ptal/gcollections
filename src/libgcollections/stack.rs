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

pub struct Stack<S, Ordering>
{
  seq: S,
  phantom_ordering: PhantomData<Ordering>
}

impl<S, Ordering> Stack<S, Ordering>
{
  pub fn wrap(seq: S) -> Self {
    Stack {
      seq: seq,
      phantom_ordering: PhantomData
    }
  }
}

impl<S, Ordering> Collection for Stack<S, Ordering> where
  S: Collection
{
  type Item = <S as Collection>::Item;
}

impl<S, Ordering> Deref for Stack<S, Ordering>
{
  type Target = S;

  fn deref<'a>(&'a self) -> &'a S {
    &self.seq
  }
}

impl<S, Ordering> DerefMut for Stack<S, Ordering>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut S {
    &mut self.seq
  }
}

impl<S, Ordering> Empty for Stack<S, Ordering> where
 S: Empty
{
  fn empty() -> Self {
    Stack::wrap(S::empty())
  }
}

impl<S, Ordering> Cardinality for Stack<S, Ordering> where
 S: Cardinality
{
  type Size = <S as Cardinality>::Size;
  fn size(&self) -> Self::Size {
    self.seq.size()
  }
}

impl<S, Ordering> Insert for Stack<S, Ordering> where
 S: Push<Ordering>
{
  fn insert(&mut self, value: Self::Item) {
    self.push(value);
  }
}

impl<S, Ordering> Extract for Stack<S, Ordering> where
 S: Pop<Ordering>
{
  fn extract(&mut self) -> Option<Self::Item> {
    self.pop()
  }
}
