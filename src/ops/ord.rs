use crate::{Eval, Fun2, Fun2Label};
use std::fmt::{Debug, Display};

/// The first value is less than the second value based on [`PartialOrd`].
pub fn lt<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialOrd,
{
    Fun2::infix("<", V::lt).eval_once(lhs, rhs)
}

/// The first value is less than or equal to the second value based on [`PartialOrd`].
pub fn le<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialOrd,
{
    Fun2::infix("<=", V::le).eval_once(lhs, rhs)
}

/// The first value is greater than the second value based on [`PartialOrd`].
pub fn gt<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialOrd,
{
    Fun2::infix(">", V::gt).eval_once(lhs, rhs)
}

/// The first value is greater than or equal to the second value based on [`PartialOrd`].
pub fn ge<L1, L2, V>(lhs: Eval<L1, &V>, rhs: Eval<L2, &V>) -> Eval<Fun2Label<'static, L1, L2>, bool>
where
    L1: Display + Copy,
    L2: Display + Copy,
    V: Debug + PartialOrd,
{
    Fun2::infix(">=", V::ge).eval_once(lhs, rhs)
}
