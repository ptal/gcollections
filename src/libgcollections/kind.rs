// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! We distinguish between ground and non-ground types. A ground type is basically a type that is not parametrized by a type parameter. This distinction is done for implementing binary operations on types.
//!
//! Let's start with an example implementing `Add` for an option-like type with the following semantics:
//!
//! * `Some(x) + Some(y) = Some(x + y)`
//! * `Some(x) + None = None`
//! * `None + Some(y) = None`
//! * `None + None = None`
//!
//! As we can see, we require inner values to be addable too. There is also another use-case:
//!
//! * `Some(x) + y = Some(x + y)`
//! * `x + Some(y) = Some(x + y)`
//! * `None + y = None`
//! * `x + None = None`
//!
//! So far we have three overloadings of `Add` for `Option`:
//!
//! * `impl<T, U> Add<Option<T>> for Option<U>`
//! * `impl<T, U> Add<T> for Option<U>`
//! * `impl<T, U> Add<Option<T>> for U`
//!
//! There is several problems:
//!
//! * Generally implementing `impl<T, U> Add<Option<T>> for U` will lead to the compilation error: "only traits defined in the current crate can be implemented for a type parameter" or "type parameter `U` must be used as the type parameter for some local type".
//! * Also, admit that we define `Add` for a type `Vec<T>` in a similar fashion, we will have a conflicting implementation in case we do `Option<A> + Vec<B>`. First of all, saying we want to define such a computation, what should be the type of the result? `Vec<Option<C>>` or `Option<Vec<C>>` (`C` being `<A as Add<B>>::Output`)? A wrong solution is to depend on the operation order, so `a + b` and `b + a` would be different, which violates the commutativity property of addition. Our choice here is to let undefined such implementation and only propose one between `Option<A>` and `Option<B>`, and between `Option<A>` and `B` if `B` is a ground type (implement `GroundType`).
//!
//! ```rust
//! #![feature(specialization)]
//! use gcollections::kind::*;
//! use std::ops::Add;
//!
//! struct Maybe<T> {
//!   value: Option<T>
//! }
//!
//! impl<T> Maybe<T> {
//!   fn some(value: T) -> Maybe<T> {
//!     Maybe {
//!       value: Some(value)
//!     }
//!   }
//!   fn none() -> Maybe<T> {
//!     Maybe {
//!       value: None
//!     }
//!   }
//! }
//!
//! impl<T, U, R> Add<Maybe<T>> for Maybe<U> where
//!  U: Add<T, Output=R>
//! {
//!   type Output = Maybe<R>;
//!
//!   fn add(self, rhs: Maybe<T>) -> Self::Output {
//!     if let Some(x) = self.value {
//!       if let Some(y) = rhs.value {
//!         return Maybe::some(x + y);
//!       }
//!     }
//!     Maybe::none()
//!   }
//! }
//!
//! impl<T, U, R> Add<T> for Maybe<U> where
//!  U: Add<T, Output=R>,
//!  T: GroundType
//! {
//!   type Output = Maybe<R>;
//!
//!   fn add(self, rhs: T) -> Self::Output {
//!     match self.value {
//!       Some(x) => Maybe::some(x + rhs),
//!       None => Maybe::none()
//!     }
//!   }
//! }
//!
//! impl<T,R> Add<Maybe<T>> for u32 where
//!  T: Add<u32, Output=R>
//! {
//!   type Output = Maybe<R>;
//!   fn add(self, rhs: Maybe<T>) -> Self::Output {
//!     rhs + self
//!   }
//! }
//! ```
//!
//! The last implementation is for `A + Option<B>`. It should be done at the ground type level but for doing it in a generic manner, we would need all non-ground types to implement a trait such as `Monad`. Thus it would be possible to propose a generic implementation handling the commutativity property of addition. However, higher-order types are not available in Rust yet so we implement `A + Option<B>` in the optional module where the ground types `A` are fixed and defined (here just shown for `u32`).
//!
//!

pub trait GroundType {}
