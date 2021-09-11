use crate::{Eval, Fun1, Fun1Label, Fun2, Fun2Label};
use std::fmt::Display;
use std::ops::Not;

/// Logical negation.
pub fn not<L>(b: Eval<L, bool>) -> Eval<Fun1Label<'static, L>, bool>
where
    L: Display + Copy,
{
    Fun1::new("!", bool::not).eval_once(b)
}

/// Logical conjunction.
pub fn and<L1, L2>(
    lhs: Eval<L1, bool>,
    rhs: Eval<L2, bool>,
) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
{
    Fun2::infix("&&", |l, r| l && r).eval_once(lhs, rhs)
}

/// Logical disjunction.
pub fn or<L1, L2>(
    lhs: Eval<L1, bool>,
    rhs: Eval<L2, bool>,
) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
{
    Fun2::infix("||", |l, r| l || r).eval_once(lhs, rhs)
}

/// Logical implication.
pub fn implies<L1, L2>(
    antecedent: Eval<L1, bool>,
    consequent: Eval<L2, bool>,
) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
{
    Fun2::infix("-->", |a: bool, c| !a || c).eval_once(antecedent, consequent)
}

/// Logical equivalence.
pub fn iff<L1, L2>(
    lhs: Eval<L1, bool>,
    rhs: Eval<L2, bool>,
) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
{
    Fun2::infix("<->", |l, r| l == r).eval_once(lhs, rhs)
}
