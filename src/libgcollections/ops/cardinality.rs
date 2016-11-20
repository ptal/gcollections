// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::{One, Zero, Unsigned, Integer};

pub trait Cardinality
{
  type Size: Unsigned + Integer;
  fn size(&self) -> Self::Size;
}

pub trait IsSingleton
{
  fn is_singleton(&self) -> bool;
}

pub trait IsEmpty
{
  fn is_empty(&self) -> bool;
}

impl<R> IsSingleton for R where
 R: Cardinality
{
  default fn is_singleton(&self) -> bool {
    self.size() == <<Self as Cardinality>::Size as One>::one()
  }
}

impl <R> IsEmpty for R where
 R: Cardinality
{
  default fn is_empty(&self) -> bool {
    self.size().is_zero()
  }
}
