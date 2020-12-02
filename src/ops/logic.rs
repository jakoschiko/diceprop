use crate::{fun_1, infix_fun_2, Eval, Fun1Label, Fun2Label};
use std::fmt::Display;
use std::ops::Not;

/// Logical negation.
pub fn not<L>(b: Eval<L, bool>) -> Eval<Fun1Label<'static, L>, bool>
where
    L: Display + Copy,
{
    fun_1("!", bool::not).eval_once(b)
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
    infix_fun_2("&&", |l, r| l && r).eval_once(lhs, rhs)
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
    infix_fun_2("||", |l, r| l || r).eval_once(lhs, rhs)
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
    infix_fun_2("-->", |a: bool, c| !a || c).eval_once(antecedent, consequent)
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
    infix_fun_2("<->", |l, r| l == r).eval_once(lhs, rhs)
}
