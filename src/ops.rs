//! A collection of operations on [`Eval`] that are useful for writing properties.
//!
//! These operations will log their arguments and result via [`dicetest::hints`].
//!
//! [`Eval`]: crate::Eval

mod logic;
pub use logic::*;

mod eq;
pub use eq::*;

mod ord;
pub use ord::*;

mod assert;
pub use assert::*;
