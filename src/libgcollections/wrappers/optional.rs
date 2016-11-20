// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kind::*;
use ops::*;
use std::ops::*;

fn binary_map_unwrap_or<T, U, R, F>(x: &Optional<T>, y: &Optional<U>, default: R, f: F) -> R where
 F: FnOnce(&T, &U) -> R
{
  if let Some(ref x) = x.as_ref() {
    if let Some(ref y) = y.as_ref() {
      return f(x, y)
    }
  }
  default
}

fn binary_value_map_unwrap_or<T, U, R, F>(x: &Optional<T>, y: &U, default: R, f: F) -> R where
 F: FnOnce(&T, &U) -> R
{
  x.as_ref().map_or(default, |x| f(x, y))
}

fn binary_value_map<T, U, R, F>(x: Optional<T>, y: U, f: F) -> Optional<R> where
 F: FnOnce(T, U) -> R
{
  x.unwrap().map_or(Optional::empty(), |x|
      Optional::singleton(f(x, y)))
}

fn binary_map<T, U, R, F>(x: Optional<T>, y: Optional<U>, f: F) -> Optional<R> where
 F: FnOnce(T, U) -> R
{
  x.unwrap().map_or(Optional::empty(), |x|
    y.unwrap().map_or(Optional::empty(), |y|
      Optional::singleton(f(x, y))))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Optional<T>
{
  value: Option<T>
}

impl<T> Optional<T>
{
  pub fn wrap(value: Option<T>) -> Optional<T> {
    Optional {
      value: value
    }
  }

  pub fn unwrap(self) -> Option<T> {
    self.value
  }
}

impl<T> Collection for Optional<T> {
  type Item = T;
}

impl<T> Deref for Optional<T>
{
  type Target = Option<T>;

  fn deref<'a>(&'a self) -> &'a Option<T> {
    &self.value
  }
}

impl<T> DerefMut for Optional<T>
{
  fn deref_mut<'a>(&'a mut self) -> &'a mut Option<T> {
    &mut self.value
  }
}

impl<T> Cardinality for Optional<T>
{
  type Size = usize;
  fn size(&self) -> usize {
    self.is_some() as usize
  }
}

impl<T> Singleton for Optional<T>
{
  fn singleton(value: T) -> Optional<T> {
    Optional::wrap(Some(value))
  }
}

impl<T> Empty for Optional<T>
{
  fn empty() -> Optional<T> {
    Optional::wrap(None)
  }
}

impl<T> Intersection<Optional<T>> for Optional<T> where
 T: Clone + PartialEq
{
  type Output = Optional<T>;

  fn intersection(&self, other: &Optional<T>) -> Self::Output {
    if self.is_empty() || other.is_empty() || self != other {
      Optional::empty()
    }
    else {
      self.clone()
    }
  }
}

impl<T> Intersection<T> for Optional<T> where
 T: Clone + PartialEq
{
  type Output = Optional<T>;

  fn intersection(&self, other: &T) -> Self::Output {
    if self.is_empty() || self.as_ref().unwrap() != other {
      Optional::empty()
    }
    else {
      self.clone()
    }
  }
}

macro_rules! primitive_optional_intersection_operation
{
  ( $( $source:ty ),* ) =>
  {$(
    impl Intersection<Optional<$source>> for $source
    {
      type Output = Optional<$source>;

      fn intersection(&self, other: &Optional<$source>) -> Self::Output {
        other.intersection(self)
      }
    }
  )*}
}

primitive_optional_intersection_operation!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

impl<T> Difference<Optional<T>> for Optional<T> where
 T: Clone + PartialEq
{
  type Output = Optional<T>;

  fn difference(&self, other: &Optional<T>) -> Self::Output {
    if self.is_empty() || self == other {
      Optional::empty()
    }
    else {
      self.clone()
    }
  }
}

impl<T> Difference<T> for Optional<T> where
 T: Clone + PartialEq
{
  type Output = Optional<T>;

  fn difference(&self, other: &T) -> Self::Output {
    if self.is_empty() || self.as_ref().unwrap() == other {
      Optional::empty()
    }
    else {
      self.clone()
    }
  }
}

macro_rules! primitive_optional_difference_operation
{
  ( $( $source:ty ),* ) =>
  {$(
    impl Difference<Optional<$source>> for $source
    {
      type Output = Optional<$source>;

      fn difference(&self, other: &Optional<$source>) -> Self::Output {
        if other.is_empty() || self != other.as_ref().unwrap() {
          Optional::singleton(self.clone())
        }
        else {
          Optional::empty()
        }
      }
    }
  )*}
}

primitive_optional_difference_operation!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

impl<T, U> Disjoint<Optional<U>> for Optional<T> where
 T: Disjoint<U>
{
  fn is_disjoint(&self, other: &Optional<U>) -> bool {
    self.is_empty() || other.is_empty() ||
    self.as_ref().unwrap().is_disjoint(other.as_ref().unwrap())
  }
}

impl<T, U> Disjoint<U> for Optional<T> where
 T: Disjoint<U>,
 U: GroundType
{
  fn is_disjoint(&self, other: &U) -> bool {
    self.is_empty() ||
    self.as_ref().unwrap().is_disjoint(other)
  }
}

macro_rules! primitive_optional_disjoint_operation
{
  ( $( $source:ty ),* ) =>
  {$(
    impl<T> Disjoint<Optional<T>> for $source where
     T: Disjoint<$source>
    {
      fn is_disjoint(&self, other: &Optional<T>) -> bool {
        other.is_disjoint(self)
      }
    }
  )*}
}

primitive_optional_disjoint_operation!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

impl<T> Contains for Optional<T> where
  T: Eq
{
  fn contains(&self, value: &T) -> bool {
    self.as_ref().map_or(false, |x| x == value)
  }
}

impl<T> Subset<Optional<T>> for Optional<T> where
 T: Subset
{
  fn is_subset(&self, other: &Optional<T>) -> bool {
    if self.is_empty() { true }
    else if other.is_empty() { false }
    else {
      self.as_ref().unwrap().is_subset(other.as_ref().unwrap())
    }
  }
}

impl<T> ProperSubset<Optional<T>> for Optional<T> where
 T: Subset + PartialEq
{
  fn is_proper_subset(&self, other: &Optional<T>) -> bool {
    self.is_subset(other) && self != other
  }
}

impl<T, U> Overlap<Optional<U>> for Optional<T> where
 T: Overlap<U>
{
  fn overlap(&self, other: &Optional<U>) -> bool {
    binary_map_unwrap_or(self, other, false, T::overlap)
  }
}

impl<T, U> Overlap<U> for Optional<T> where
 T: Overlap<U>,
 U: GroundType
{
  fn overlap(&self, other: &U) -> bool {
    binary_value_map_unwrap_or(self, other, false, T::overlap)
  }
}

macro_rules! primitive_optional_overlap_operation
{
  ( $( $source:ty ),* ) =>
  {$(
    impl<T> Overlap<Optional<T>> for $source where
     T: Overlap<$source>
    {
      fn overlap(&self, other: &Optional<T>) -> bool {
        other.overlap(self)
      }
    }
  )*}
}

primitive_optional_overlap_operation!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

impl<T, U, R> Add<Optional<U>> for Optional<T> where
 T: Add<U, Output=R>
{
  type Output = Optional<R>;

  fn add(self, other: Optional<U>) -> Self::Output {
    binary_map(self, other, T::add)
  }
}

impl<T, U, R> Add<U> for Optional<T> where
 T: Add<U, Output=R>,
 U: GroundType
{
  type Output = Optional<R>;

  fn add(self, other: U) -> Self::Output {
    binary_value_map(self, other, T::add)
  }
}

macro_rules! add_optional_arithmetics
{
  ( $( $source:ty ),* ) =>
  {$(
    impl<T, R> Add<Optional<T>> for $source where
     T: Add<$source, Output=R>
    {
      type Output = Optional<R>;

      fn add(self, rhs: Optional<T>) -> Self::Output {
        rhs + self
      }
    }
  )*}
}

add_optional_arithmetics!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

impl<T, U, R> Sub<Optional<U>> for Optional<T> where
 T: Sub<U, Output=R>
{
  type Output = Optional<R>;

  fn sub(self, other: Optional<U>) -> Self::Output {
    binary_map(self, other, T::sub)
  }
}

impl<T, U, R> Sub<U> for Optional<T> where
 T: Sub<U, Output=R>,
 U: GroundType
{
  type Output = Optional<R>;

  fn sub(self, other: U) -> Self::Output {
    binary_value_map(self, other, T::sub)
  }
}

macro_rules! sub_optional_arithmetics
{
  ( $( $source:ty ),* ) =>
  {$(
    impl Sub<Optional<$source>> for $source
    {
      type Output = Optional<$source>;

      fn sub(self, other: Optional<$source>) -> Self::Output {
        binary_value_map(other, self, |x, y| y - x)
      }
    }
  )*}
}

sub_optional_arithmetics!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64);

impl<T, U, R> Mul<Optional<U>> for Optional<T> where
 T: Mul<U, Output=R>
{
  type Output = Optional<R>;

  fn mul(self, other: Optional<U>) -> Self::Output {
    binary_map(self, other, T::mul)
  }
}

impl<T, U, R> Mul<U> for Optional<T> where
 T: Mul<U, Output=R>,
 U: GroundType
{
  type Output = Optional<R>;

  fn mul(self, other: U) -> Self::Output {
    binary_value_map(self, other, T::mul)
  }
}

macro_rules! mul_optional_arithmetics
{
  ( $( $source:ty ),* ) =>
  {$(
    impl<T, R> Mul<Optional<T>> for $source where
     T: Mul<$source, Output=R>
    {
      type Output = Optional<R>;

      fn mul(self, other: Optional<T>) -> Self::Output {
        other.mul(self)
      }
    }
  )*}
}

mul_optional_arithmetics!(i8,u8,i16,u16,i32,u32,i64,u64,isize,usize,f32,f64,bool,char);

#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
  use super::*;
  use ops::*;

  const empty: Optional<i32> = Optional { value: None };
  const zero: Optional<i32> = Optional { value: Some(0) };
  const ten: Optional<i32> = Optional { value: Some(10) };

  #[test]
  fn cardinality_test() {
    assert_eq!(empty.size(), 0);
    assert_eq!(zero.size(), 1);
    assert_eq!(ten.size(), 1);
    assert!(empty.is_empty());
    assert!(!empty.is_singleton());
    assert!(!zero.is_empty());
    assert!(zero.is_singleton());
  }

  #[test]
  fn constructors_test() {
    assert_eq!(empty, Empty::empty());
    assert_eq!(zero, Singleton::singleton(0));
  }

  #[test]
  fn intersection_test() {
    let sym_cases = vec![
      (empty, empty, empty),
      (empty, zero, empty),
      (zero, zero, zero),
      (zero, ten, empty),
      (ten, ten, ten)
    ];

    for (x,y,r) in sym_cases.into_iter() {
      assert!(x.intersection(&y) == r, "{:?} intersection {:?} is not equal to {:?}", x, y, r);
      assert!(y.intersection(&x) == r, "{:?} intersection {:?} is not equal to {:?}", y, x, r);
    }
  }

  #[test]
  fn difference_test() {
    let cases = vec![
      (empty, empty,  empty, empty),
      (empty, zero,   empty, zero),
      (zero, zero,    empty, empty),
      (zero, ten,     zero, ten),
      (ten, ten,      empty, empty)
    ];

    for (x,y,r1,r2) in cases.into_iter() {
      assert!(x.difference(&y) == r1, "{:?} difference {:?} is not equal to {:?}", x, y, r1);
      assert!(y.difference(&x) == r2, "{:?} difference {:?} is not equal to {:?}", y, x, r2);
    }
  }

  #[test]
  fn intersection_value_test() {
    let sym_cases = vec![
      (empty, 0, empty),
      (empty, 1, empty),
      (zero, 0, zero),
      (zero, 1, empty),
      (ten, 10, ten)
    ];

    for (x,y,r) in sym_cases.into_iter() {
      assert!(x.intersection(&y) == r, "{:?} intersection {:?} is not equal to {:?}", x, y, r);
      assert!(y.intersection(&x) == r, "{:?} intersection {:?} is not equal to {:?}", y, x, r);
    }
  }

  #[test]
  fn difference_value_test() {
    let cases = vec![
      (empty, 0,  empty, zero),
      (zero, 0,   empty, empty),
      (zero, 10,  zero, ten),
      (ten,  10,  empty, empty)
    ];

    for (x,y,r1,r2) in cases.into_iter() {
      assert!(x.difference(&y) == r1, "{:?} difference {:?} is not equal to {:?}", x, y, r1);
      assert!(y.difference(&x) == r2, "{:?} difference {:?} is not equal to {:?}", y, x, r2);
    }
  }

  #[test]
  fn is_disjoint_and_overlap_test() {
    let sym_cases = vec![
      (empty, empty, true),
      (empty, zero, true),
      (zero, zero, false),
      (zero, ten, true),
      (ten, ten, false)
    ];

    for (x,y,r) in sym_cases.into_iter() {
      assert!(x.is_disjoint(&y) == r, "{:?} disjoint {:?} is not equal to {:?}", x, y, r);
      assert!(y.is_disjoint(&x) == r, "{:?} disjoint {:?} is not equal to {:?}", y, x, r);
      assert!(x.overlap(&y) == !r, "{:?} overlap {:?} is not equal to {:?}", x, y, !r);
      assert!(y.overlap(&x) == !r, "{:?} overlap {:?} is not equal to {:?}", y, x, !r);
    }
  }

  #[test]
  fn contains_test() {
    let cases = vec![
      (empty, 0, false),
      (empty, 1, false),
      (zero, 0, true),
      (zero, 1, false),
      (ten, 9, false),
      (ten, 10, true)
    ];

    for (x,y,r) in cases.into_iter() {
      assert!(x.contains(&y) == r, "{:?} contains {:?} is not equal to {:?}", x, y, r);
    }
  }

  #[test]
  fn subset_test() {
    let cases = vec![
      (empty, empty,  true, true),
      (empty, zero,   true, false),
      (zero, zero,    true, true),
      (zero, ten,     false, false),
      (ten, ten,      true, true)
    ];

    for (x,y,r1,r2) in cases.into_iter() {
      assert!(x.is_subset(&y) == r1, "{:?} subset {:?} is not equal to {:?}", x, y, r1);
      assert!(y.is_subset(&x) == r2, "{:?} subset {:?} is not equal to {:?}", y, x, r2);
    }
  }

  #[test]
  fn proper_subset_test() {
    let cases = vec![
      (empty, empty,  false, false),
      (empty, zero,   true, false),
      (zero, zero,    false, false),
      (zero, ten,     false, false),
      (ten, ten,      false, false)
    ];

    for (x,y,r1,r2) in cases.into_iter() {
      assert!(x.is_proper_subset(&y) == r1, "{:?} proper_subset {:?} is not equal to {:?}", x, y, r1);
      assert!(y.is_proper_subset(&x) == r2, "{:?} proper_subset {:?} is not equal to {:?}", y, x, r2);
    }
  }

  #[test]
  fn arithmetics_tests() {
    let twenty = Optional::singleton(20);
    let one_hundred = Optional::singleton(100);
    let cases = vec![
      // x,     y,    x+y,   x-y,   x*y
      (empty, empty, empty, empty, empty),
      (empty, ten,   empty, empty, empty),
      (ten,   empty, empty, empty, empty),
      (ten,   zero,  ten,   ten,   zero),
      (ten,   ten,   twenty,zero,  one_hundred)
    ];
    for (x,y,add,sub,mul) in cases.into_iter() {
      assert!(x + y == add, "{:?} + {:?} is not equal to {:?}", x, y, add);
      assert!(x - y == sub, "{:?} - {:?} is not equal to {:?}", x, y, sub);
      assert!(x * y == mul, "{:?} * {:?} is not equal to {:?}", x, y, mul);
    }
  }
}
