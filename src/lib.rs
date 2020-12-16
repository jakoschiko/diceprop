//! Provides mathematical properties that can be asserted with randomly generated
//! test data.
//!
//! It's based on [dicetest](https://github.com/jakoschiko/dicetest).
//!
//! # Status of this crate
//!
//! The author does not consider this crate as stable yet.
//!
//! # Examples
//!
//! ## Associative binary operation
//!
//! ```
//! use diceprop::{infix_fun_2, props, FateVarExt};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn add_is_associative_for_small_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let small_f32_die = dice::f32(-100.0..=100.0);
//!         let var = fate.roll_var_3("f32 ∩ [-100,100]", ["x", "y", "z"], small_f32_die);
//!         let add = infix_fun_2("+", |x, y| x + y);
//!         props::binop::associative(var, add);
//!     })
//! }
//! ```
//!
//! The test fails with the following output:
//!
//! ```text
//! The test failed after 22 passes.
//!
//! # Config
//! - seed: 9463254571221690676
//! - start limit: 0
//! - end limit: 100
//! - passes: 200
//!
//! # Counterexample
//! - run code: "+AmE1tbVveYGYlIjaeh8pudhxZe2x8PIexq8M9U/Xn4LAAAAAAAAAA=="
//! - limit: 11
//! - hints:
//!         - Is `+` associative?
//!                 - x, y, z of f32 ∩ [-100,100]
//!                 - x = 344.4662
//!                 - y = 503.5587
//!                 - z = 0.70710677
//!                 - (x + y) = 848.0249
//!                 - ((x + y) + z) = 848.732
//!                 - (y + z) = 504.26578
//!                 - (x + (y + z)) = 848.73193
//!                 - (((x + y) + z) == (x + (y + z))) = false
//! - error: assertion failed: (((x + y) + z) == (x + (y + z)))
//! ```
//!
//! ## Left inverse function
//!
//! ```
//! use diceprop::{fun_1, postfix_fun_1, props, FateVarExt};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn sqrt_is_left_inverse_of_sq_for_non_negative_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let non_negative_f32_die = dice::f32(0.0..);
//!         let var = fate.roll_var_1("f32 ∩ [0,+∞]", "x", non_negative_f32_die);
//!         let sq = postfix_fun_1("²", |x| x * x);
//!         let sqrt = fun_1("√", |x: f32| x.sqrt());
//!         props::fun::left_inverse(var, sq, sqrt);
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
//! use diceprop::{infix_fun_2, props, FateVarExt};
//! use dicetest::prelude::*;
//!
//! #[test]
//! fn gt_is_partial_order_for_any_f32() {
//!     Dicetest::repeatedly().run(|mut fate| {
//!         let any_f32_die = dice::any_f32();
//!         let var = fate.roll_var_3("f32", ["x", "y", "z"], any_f32_die);
//!         let gt = infix_fun_2("≤", |x, y| x <= y);
//!         props::binrel::partial_order(var, gt);
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
pub use elem::{elem, Elem};

mod var;
pub use var::{var_1, var_2, var_3, FateVarExt, Var1, Var2, Var3};

mod fun;
pub use fun::{fun_1, fun_2, infix_fun_2, postfix_fun_1, Fun1, Fun1Label, Fun2, Fun2Label};

pub mod ops;

pub mod props;
