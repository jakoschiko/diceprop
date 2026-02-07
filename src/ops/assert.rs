use std::fmt::Display;

use crate::Eval;

/// Asserts that `assertion` is true.
///
/// # Panics
///
/// Panics if `assertion` is false.
pub fn assert<L: Display + Copy>(assertion: Eval<L, bool>) {
    assert!(assertion.value, "assertion failed: {}", assertion.label);
}
