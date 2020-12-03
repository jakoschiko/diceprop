use std::fmt::Debug;

use crate::props::{associative_binop, identity_elem_of_binop, inverse_elem_of_binop};
use crate::{hint_section, var_1, var_2, var_3, Elem, Fun1, Fun2, Var3};

/// Asserts that `(var.set, op)` is a semigroup.
///
/// It must hold:
/// - `op` is associative  ([`associative_binop`])
pub fn semigroup<S, O>(var: Var3<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {})` a semigroup?", var.set, op.name);

    associative_binop(var, op);
}

/// Asserts that `(var.set, op, e)` is a monoid.
///
/// It must hold:
/// - `(var.set, op)` is a semigroup ([`semigroup`])
/// - `e` is the identity element of `op` ([`identity_elem_of_binop`])
pub fn monoid<S, O>(var: Var3<S>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {}, {})` a monoid?", var.set, op.name, e.name,);

    let [a, b, c] = var.elems;
    let var_1 = var_1(var.set, a.clone());
    let var_3 = var_3(var.set, [a, b, c]);

    semigroup(var_3, op.as_ref());
    identity_elem_of_binop(var_1, op, e)
}

/// Asserts that `(var.set, op, inv, e)` is a group.
///
/// It must hold:
/// - `(var.set, op, e)` is a monoid ([`monoid`])
/// - `inv` returns the inverse elements regarding to `op` ([`inverse_elem_of_binop`])
pub fn group<S, O, I>(var: Var3<S>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {})` a group?",
        var.set,
        op.name,
        inv.name,
        e.name,
    );

    let [a, b, c] = var.elems;
    let var_2 = var_2(var.set, [a.clone(), b.clone()]);
    let var_3 = var_3(var.set, [a, b, c]);

    monoid(var_3, op.as_ref(), e);
    inverse_elem_of_binop(var_2, op, inv);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{elem, fun_1, fun_2, props, FateVarExt};

    #[test]
    fn semigroup_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("u64", ["x", "y", "z"], dice::u64(..=1000));
            let op = fun_2("+", |x, y| x + y);
            props::semigroup(var, op);
        })
    }

    #[test]
    fn monoid_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("u64", ["x", "y", "z"], dice::u64(..=1000));
            let op = fun_2("+", |x, y| x + y);
            let e = elem("zero", 0);
            props::monoid(var, op, e);
        })
    }

    #[test]
    fn group_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let op = fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            let e = elem("zero", 0);
            props::group(var, op, inv, e);
        })
    }
}
