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
//! It acts as a temporary substitute and will be replaced when proper generic supports will be added on standard collections. To use these operations on standard collections, you must wrap it inside the structure of the same name in `ncollection::*`. This is because some methods have the same name than existing one's.
//!

#![feature(collections, enumset, specialization)]

extern crate collections;
extern crate num;
extern crate bit_set;

pub mod macros;
pub mod kind;
#[macro_use]
pub mod ops;
pub mod wrappers;

pub use wrappers::hash_set::HashSet;
pub use wrappers::btree_set::BTreeSet;
pub use wrappers::enum_set::EnumSet;
pub use wrappers::bit_set::BitSet;
pub use wrappers::optional::Optional;

