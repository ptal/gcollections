// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::SequenceKind;
use ops::*;
use ops::sequence::ordering::*;
use stack::*;
use queue::*;
use std::ops::{Deref, DerefMut};
use std::collections::VecDeque;

type DequeFrontStack<T> = Stack<VectorDeque<T>, Front, T>;
type DequeBackStack<T> = Stack<VectorDeque<T>, Back, T>;
type DequeFrontBackQueue<T> = Queue<VectorDeque<T>, Front, Back, T>;
type DequeBackFrontQueue<T> = Queue<VectorDeque<T>, Back, Front, T>;

pub struct VectorDeque<T>
{
  deque: VecDeque<T>
}

impl<T> SequenceKind for VectorDeque<T> {}

impl<T> VectorDeque<T>
{
  pub fn wrap(deque: VecDeque<T>) -> VectorDeque<T> {
    VectorDeque {
      deque: deque
    }
  }
}

impl<T> Deref for VectorDeque<T>
{
  type Target = VecDeque<T>;

  fn deref<'a>(&'a self) -> &'a VecDeque<T> {
    &self.deque
  }
}

impl<T> DerefMut for VectorDeque<T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut VecDeque<T> {
    &mut self.deque
  }
}

impl<T> Empty for VectorDeque<T> {
  fn empty() -> VectorDeque<T> {
    VectorDeque::wrap(VecDeque::new())
  }
}

impl<T> Push<Front, T> for VectorDeque<T> {
  fn push(&mut self, value: T) {
    self.deque.push_front(value);
  }
}

impl<T> Push<Back, T> for VectorDeque<T> {
  fn push(&mut self, value: T) {
    self.deque.push_back(value);
  }
}

impl<T> Pop<Front, T> for VectorDeque<T> {
  fn pop(&mut self) -> Option<T> {
    self.deque.pop_front()
  }
}

impl<T> Pop<Back, T> for VectorDeque<T> {
  fn pop(&mut self) -> Option<T> {
    self.deque.pop_back()
  }
}

impl<T> Cardinality for VectorDeque<T> {
  type Size = usize;
  fn size(&self) -> usize {
    self.deque.len()
  }
}
