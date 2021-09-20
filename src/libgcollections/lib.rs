// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Wrappers of the standard collection library for generic programming.
//!
//! This library categorizes operations on collections such as sets, tuples or vectors. The goal is to allow designing generic algorithms by specifying trait bounds on type parameters.
//!
//! It acts as a temporary substitute and will be replaced when proper generic supports will be added on standard collections. Generic operations are implemented on wrappers of the standard collections (available in `wrappers::*`), this is due to name conflicts between existing methods and traits function names.
//!

//! If the feature `nightly` is defined (use `cargo build --features="nightly"`), then some of the traits are implemented using specialization.
//! On `stable` they are implemented for every type satisfying some trait bounds, but a user cannot override the definitions.

#![cfg_attr(feature = "nightly", feature(specialization))]

extern crate num_integer;
extern crate num_traits;
extern crate bit_set;
extern crate trilean;

pub mod macros;
pub mod kind;
#[macro_use]
pub mod ops;
pub mod wrappers;
pub mod queue;
pub mod stack;

pub use kind::*;
pub use queue::*;
pub use stack::*;
pub use wrappers::hash_set::*;
pub use wrappers::btree_set::*;
pub use wrappers::bit_set::*;
pub use wrappers::optional::*;
pub use wrappers::vector::*;
pub use wrappers::vector_deque::*;
