//! A collection of mathematical properties for random testing.
//!
//! # Example
//!
//! ```no_run
//! use diceprop::{props, Fun2, Set};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn wrapping_add_is_associative_for_u32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let set = Set::new("u32", dice::u32(..));
//!         let vars = fate.roll(set.vars(["x", "y", "z"]));
//!         let add = Fun2::infix("+", |x, y| u32::wrapping_add(x, y));
//!         props::binop::associative(vars, add);
//!     })
//! }
//! ```

#[macro_use]
mod eval;
pub use eval::Eval;

mod elem;
pub use elem::Elem;

mod vars;
pub use vars::Vars;

mod set;
pub use set::Set;

mod fun;
pub use fun::{Fun1, Fun1Label, Fun2, Fun2Label, Fun3, Fun3Label};

pub mod ops;

pub mod props;

// Test examples from the readme.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
