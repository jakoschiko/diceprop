use crate::{infix_fun_2, Eval, Fun2Label};
use std::fmt::{Debug, Display};

/// The values are equal based on [`PartialEq`].
pub fn eq<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialEq,
{
    infix_fun_2("==", |l, r| l == r).eval_once(lhs, rhs)
}

/// The values are not equal based on [`PartialEq`].
pub fn ne<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialEq,
{
    infix_fun_2("!=", |l, r| l != r).eval_once(lhs, rhs)
}
