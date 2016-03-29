// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use collections::enum_set::EnumSet as StdEnumSet;
use collections::enum_set::CLike;
use std::ops::{Deref, DerefMut};
use ops::*;

pub struct EnumSet<T>
{
  es: StdEnumSet<T>
}

impl<E: CLike> EnumSet<E>
{
  pub fn wrap(es: StdEnumSet<E>) -> EnumSet<E> {
    EnumSet{es: es}
  }
}

impl<T> Deref for EnumSet<T>
{
  type Target = StdEnumSet<T>;

  fn deref<'a>(&'a self) -> &'a StdEnumSet<T> {
    &self.es
  }
}

impl<T> DerefMut for EnumSet<T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut StdEnumSet<T> {
    &mut self.es
  }
}

impl<E: CLike> Contains<E> for EnumSet<E>
{
  contains_deref_impl!(E);
}

macro_rules! set_op_impl
{
  ( $( $t: ident, $m:ident );* ) =>
  {$(
    impl<E: CLike> $t for EnumSet<E>
    {
      type Output = EnumSet<E>;

      fn $m(&self, other: &EnumSet<E>) -> EnumSet<E> {
        EnumSet::wrap(self.deref().$m(**other))
      }
    }
  )*}
}

set_op_impl! {
  Intersection, intersection;
  Union, union
}
