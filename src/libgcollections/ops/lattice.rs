// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// A [lattice](https://en.wikipedia.org/wiki/Lattice_(order)) is an ordered set where every pair of elements can be combined with a `join` and `meet` operations.
/// The following traits operate on an element of the lattice, and do not represent a set of elements (which would be the whole lattice).
/// This is the reason why we do not require `Lattice` to inherit from `Collection`.

use trilean::SKleene;

pub trait Join
{
  fn join(self, other: Self) -> Self;
}

pub trait Meet
{
  fn meet(self, other: Self) -> Self;
}

pub trait Entailment
{
  fn entail(&self, other: &Self) -> SKleene;
}

pub trait StrictEntailment
{
  /// Must be similar to `a.entail(&b) && a != b`.
  fn strict_entail(&self, other: &Self) -> SKleene;
}

macro_rules! strict_entailment_impl
{
  ( $( $keyword:tt ),*) =>
  {
    impl<R> StrictEntailment for R where
      R: Entailment + Eq
    {
      $($keyword)* fn strict_entail(&self, other: &R) -> SKleene {
        self.entail(other).and(SKleene::from_bool(self != other))
      }
    }
  }
}

#[cfg(feature = "nightly")]
strict_entailment_impl!(default);
#[cfg(not(feature = "nightly"))]
strict_entailment_impl!();

pub trait Top {
  fn top() -> Self;
}

pub trait Bot {
  fn bot() -> Self;
}

pub trait Lattice:
   Join
 + Meet
 + Entailment
{}

impl<R> Lattice for R where
 R: Join,
 R: Meet,
 R: Entailment,
{}

pub trait BoundedLattice:
   Lattice
 + Top
 + Bot
{}

impl<R> BoundedLattice for R where
 R: Lattice,
 R: Top,
 R: Bot,
{}

pub mod test
{
  use super::*;
  use trilean::SKleene;
  use trilean::SKleene::*;
  use std::fmt::{Display, Debug};

  pub struct LatticeTester<T>
  {
    test_id: usize,
    current_test: String,
    data_a: Vec<T>,
    data_b: Vec<T>,
    expected_entailment: Vec<SKleene>,
    expected_join: Vec<T>,
    expected_meet: Vec<T>
  }

  impl<T> LatticeTester<T> where
    T: BoundedLattice + Clone + Debug + Display + Eq
  {
    pub fn new(test_id: usize, data_a: Vec<T>, data_b: Vec<T>,
      expected_entailment: Vec<SKleene>,
      expected_join: Vec<T>,
      expected_meet: Vec<T>) -> Self
    {
      let tester = LatticeTester {
        test_id: test_id,
        current_test: String::new(),
        data_a: data_a,
        data_b: data_b,
        expected_entailment: expected_entailment,
        expected_join: expected_join,
        expected_meet: expected_meet
      };
      tester.verify_input_data();
      tester
    }

    fn verify_input_data(&self) {
      assert_eq!(self.data_a.len(), self.data_b.len());
      assert_eq!(self.data_a.len(), self.expected_entailment.len());
      assert_eq!(self.data_a.len(), self.expected_join.len());
      assert_eq!(self.data_a.len(), self.expected_meet.len());
    }

    pub fn test_all(mut self) {
      let n = self.data_a.len();
      for i in 0..n {
        self.current_test = self.make_test_name(i, "entail");
        self.test_entailment(self.data_a[i].clone(), self.data_b[i].clone(), self.expected_entailment[i]);
      }
      for i in 0..n {
        self.current_test = self.make_test_name(i, "join");
        self.test_join(self.data_a[i].clone(), self.data_b[i].clone(), self.expected_join[i].clone());
      }
      for i in 0..n {
        self.current_test = self.make_test_name(i, "meet");
        self.test_meet(self.data_a[i].clone(), self.data_b[i].clone(), self.expected_meet[i].clone());
      }
    }

    fn make_test_name(&self, idx: usize, op: &str) -> String {
      format!("[{}] `{}`-test on the data with the index {} has failed.\n\
                 a = {}\n\
                 b = {}\n\
                 reason: ", self.test_id, op, idx, self.data_a[idx], self.data_b[idx])
    }

    pub fn test_entailment(&self, a: T, b: T, expected: SKleene)
    {
      let ab = a.entail(&b);
      self.assert_expected("a.entail(b)", ab.clone(), expected, "");
      let ba = b.entail(&a);
      match ab {
        True => {
          assert!(ba != Unknown,
            "{}`a.entail(b) == True` and `b.entail(a) == Unknown` \n\
             problem: if `a |= b` holds, then `b |= a` is either true (a = b) or false.\n\
             note: `a` and `b` form a chain and can not be unordered.",
            self.current_test);
          match ba {
            True => self.test_equality(a, b, true,
              "\n problem: if `b |= a` is `true` then we must have `a == b`."),
            False => self.test_strict_entail(a, b, True),
            Unknown => self.test_strict_entail(a, b, Unknown)
          }
        },
        False | Unknown => {
          self.assert_expected("b |= a", ba, !ab,
            format!("\n  problem: if `a |= b` is `{}` then we must have `b |= a` equal to `{}`.",
              ab, !ab).as_str());
          self.test_equality(a, b, false,
            format!("\n problem: if `a |= b` is `{}` then we must have `a != b`", ab).as_str());
        }
      }
    }

    fn test_equality(&self, a: T, b: T, expected: bool, msg: &str) {
      self.assert_expected("a == b", a == b, expected, msg);
      self.assert_expected("b == a", b == a, expected, msg);
    }

    fn assert_expected<U: Eq + Debug + Display>(&self, op: &str, obtained: U, expected: U, msg: &str) {
      assert_eq!(obtained, expected,
        "{}`{}` is equal to `{}` instead of the expected value `{}`.{}",
          self.current_test, op, obtained, expected, msg);
    }

    fn test_strict_entail(&self, a: T, b: T, expected: SKleene) {
      let ab = a.strict_entail(&b);
      let ba = b.strict_entail(&a);
      self.assert_expected("a.strict_entail(b)", ab, expected, "");
      self.assert_expected("b.strict_entail(a)", ba, !expected, "");
      match expected {
        True | Unknown => self.test_equality(a, b, false,
          format!("\n problem: if `a.strict_entail(b)` is `{}` then we must have `a != b`.", expected).as_str()),
        False => (),
      }
    }

    pub fn test_join(&self, a: T, b: T, expected: T) {
      self.test_top_bot_join(a.clone());
      let c = a.clone().join(b.clone());
      self.test_equality(c.clone(), expected.clone(), true,
        format!("\n problem: `a.join(b)` != {}.", expected).as_str());

      // Test commutativity
      let d = b.clone().join(a.clone());
      self.test_equality(d.clone(), c.clone(), true,
        "\n problem (commutativity): `a.join(b)` != b.join(a).");

      // Test idempotence
      let e = a.clone().join(b.clone()).join(c.clone());
      self.test_equality(c.clone(), e.clone(), true,
        "\n problem (idempotency): `a.join(b)` != `a.join(b).join(a.join(b))`.");
      let f = d.clone().join(c.clone());
      self.test_equality(c.clone(), f.clone(), true,
        "\n problem (idempotency): `a.join(b)` != `b.join(a).join(a.join(b))`.");
      let g = a.clone().join(a.clone());
      let h = b.clone().join(b.clone());
      self.test_equality(a.clone(), g.clone(), true,
        "\n problem (idempotency): `a` != `a.join(a)`.");
      self.test_equality(b.clone(), h.clone(), true,
        "\n problem (idempotency): `b` != `b.join(b)`.");

      // Test relation with entailment.
      self.test_entailment(c.clone(), a.clone(), True);
      self.test_entailment(c.clone(), b.clone(), True);
      if a == c {
        self.test_entailment(a.clone(), b.clone(), True);
      }
      if b == c {
        self.test_entailment(b.clone(), a.clone(), True);
      }
    }

    pub fn test_meet(&self, a: T, b: T, expected: T) {
      self.test_top_bot_meet(a.clone());
      let c = a.clone().meet(b.clone());
      self.test_equality(c.clone(), expected.clone(), true,
        format!("\n problem: `a.meet(b)` != {}.", expected).as_str());

      // Test commutativity
      let d = b.clone().meet(a.clone());
      self.test_equality(d.clone(), c.clone(), true,
        "\n problem (commutativity): `a.meet(b)` != b.meet(a).");

      // Test idempotence
      let e = a.clone().meet(b.clone()).meet(c.clone());
      self.test_equality(c.clone(), e.clone(), true,
        "\n problem (idempotency): `a.meet(b)` != `a.meet(b).meet(a.meet(b))`.");
      let f = d.clone().meet(c.clone());
      self.test_equality(c.clone(), f.clone(), true,
        "\n problem (idempotency): `a.meet(b)` != `b.meet(a).meet(a.meet(b))`.");
      let g = a.clone().meet(a.clone());
      let h = b.clone().meet(b.clone());
      self.test_equality(a.clone(), g.clone(), true,
        "\n problem (idempotency): `a` != `a.meet(a)`.");
      self.test_equality(b.clone(), h.clone(), true,
        "\n problem (idempotency): `b` != `b.meet(b)`.");

      // Test relation with entailment.
      self.test_entailment(a.clone(), c.clone(), True);
      self.test_entailment(b.clone(), c.clone(), True);
      if a == c {
        self.test_entailment(b.clone(), a.clone(), True);
      }
      if b == c {
        self.test_entailment(a.clone(), b.clone(), True);
      }
    }

    fn test_top_bot_join(&self, a: T) {
      let top = T::top();
      let bot = T::bot();
      self.assert_expected("a.join(top)", a.clone().join(top.clone()), top.clone(),
        "\n problem: `a.join(top)` must be equal to `top`.");
      self.assert_expected("top.join(a)", top.clone().join(a.clone()), top.clone(),
        "\n problem: `top.join(a)` must be equal to `top`.");
     self.assert_expected("a.join(bot)", a.clone().join(bot.clone()), a.clone(),
        "\n problem: `a.join(bot)` must be equal to `a`.");
      self.assert_expected("bot.join(a)", bot.clone().join(a.clone()), a.clone(),
        "\n problem: `bot.join(a)` must be equal to `a`.");
    }

    fn test_top_bot_meet(&self, a: T) {
      let top = T::top();
      let bot = T::bot();
      self.assert_expected("a.meet(top)", a.clone().meet(top.clone()), a.clone(),
        "\n problem: `a.meet(top)` must be equal to `a`.");
      self.assert_expected("top.meet(a)", top.clone().meet(a.clone()), a.clone(),
        "\n problem: `top.meet(a)` must be equal to `a`.");
     self.assert_expected("a.meet(bot)", a.clone().meet(bot.clone()), bot.clone(),
        "\n problem: `a.meet(bot)` must be equal to `bot`.");
      self.assert_expected("bot.meet(a)", bot.clone().meet(a.clone()), bot.clone(),
        "\n problem: `bot.meet(a)` must be equal to `bot`.");
    }
  }
}
