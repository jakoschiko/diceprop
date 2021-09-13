//! A collection of mathematical properties for random testing.
//!
//! It's based on [dicetest](https://github.com/jakoschiko/dicetest).
//!
//! # Status of this crate
//!
//! The author does not consider this crate as stable yet. Changes will be documented in the
//! [changelog](https://github.com/jakoschiko/diceprop/blob/master/CHANGELOG.md).
//!
//! # Examples
//!
//! ## Associative binary operation
//!
//! ```
//! use diceprop::{props, Fun2, Set};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn add_is_associative_for_small_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let set = Set::new("f32 ∩ [-100,100]", dice::f32(-100.0..=100.0));
//!         let vars = fate.roll(set.vars(["x", "y", "z"]));
//!         let add = Fun2::infix("+", |x, y| x + y);
//!         props::binop::associative(vars, add);
//!     })
//! }
//! ```
//!
//! The test fails with the following output:
//!
//! ```text
//! The test failed after 12 passes.
//!
//! # Config
//! - seed: 14859458141222391139
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "2pYRCj9fj8sV52fB5iyFhxCISGY3nKlMzlzIKq0NKLwGAAAAAAAAAA=="
//! - limit: 6
//! - hints:
//!         - Is `+` associative?
//!                 - x, y, z of f32 ∩ [-100,100]
//!                 - x = 96.621735
//!                 - y = -90.97134
//!                 - z = -8.10239
//!                 - (x + y) = 5.6503983
//!                 - ((x + y) + z) = -2.451992
//!                 - (y + z) = -99.07373
//!                 - (x + (y + z)) = -2.4519958
//!                 - (((x + y) + z) == (x + (y + z))) = false
//! - error: assertion failed: (((x + y) + z) == (x + (y + z)))
//! ```
//!
//! ## Left inverse function
//!
//! ```
//! use diceprop::{props, Fun1, Set};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn sqrt_is_left_inverse_of_sq_for_non_negative_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let set = Set::new("f32 ∩ [0,+∞]", dice::f32(0.0..));
//!         let vars = fate.roll(set.vars(["x"]));
//!         let sq = Fun1::postfix("²", |x| x * x);
//!         let sqrt = Fun1::new("√", |x: f32| x.sqrt());
//!         props::fun::left_inverse(vars, sq, sqrt);
//!     })
//! }
//! ```
//!
//! The test fails with the following output:
//!
//! ```text
//! The test failed after 0 passes.
//!
//! # Config
//! - seed: 7632522237817347676
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "F2/nnlbX6qyCOm5MU7P8BSXdnJ4XNXJdihgwhtWxlzMAAAAAAAAAAA=="
//! - limit: 0
//! - hints:
//!         - Is `√` left inverse of `²`?
//!                 - x of f32 ∩ [0,+∞]
//!                 - x = 305770290000000000000000000000000000000.0
//!                 - (x)² = inf
//!                 - √((x)²) = inf
//!                 - (√((x)²) == x) = false
//! - error: assertion failed: (√((x)²) == x)
//! ```
//!
//! ## Partial order
//!
//! ```
//! use diceprop::{props, Fun2, Set};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn gt_is_partial_order_for_any_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let set = Set::new("f32", dice::any_f32());
//!         let vars = fate.roll(set.vars(["x", "y", "z"]));
//!         let gt = Fun2::infix("≤", |x, y| x <= y);
//!         props::binrel::partial_order(vars, gt);
//!     })
//! }
//! ```
//!
//! The test fails with the following output:
//!
//! ```text
//! The test failed after 3 passes.
//!
//! # Config
//! - seed: 18374838706510982620
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "h6jQMNr6fi/j9OZOXmklXYAUATM96EpE6+DENMhSZHkBAAAAAAAAAA=="
//! - limit: 1
//! - hints:
//!         - Is `≤` a partial order?
//!                 - Is `≤` reflexive?
//!                         - x of f32
//!                         - x = NaN
//!                         - (x ≤ x) = false
//! - error: assertion failed: (x ≤ x)
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
