use dicetest::hint_section;
use std::fmt::Debug;

use crate::props::{
    associative_binop, commutative_binop, distributive_binop, identity_elem_of_binop,
    inverse_elem_of_binop,
};
use crate::{var_1, var_2, var_3, Elem, Fun1, Fun2, Var2, Var3};

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

/// Asserts that `(var.set, op, inv, e)` is an abelian group.
///
/// It must hold:
/// - `(var.set, op, inv, e)` is a group ([`group`])
/// - `op` is commutative ([`commutative_binop`])
pub fn abelian_group<S, O, I>(var: Var3<S>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {})` an abelian group?",
        var.set,
        op.name,
        inv.name,
        e.name,
    );

    let [a, b, c] = var.elems;
    let var_2 = var_2(var.set, [a.clone(), b.clone()]);
    let var_3 = var_3(var.set, [a, b, c]);

    group(var_3, op.as_ref(), inv, e);
    commutative_binop(var_2, op);
}

/// Asserts that `(var.set, add, mul, neg, zero, one)` is a ring.
///
/// It must hold:
/// - `(var.set, add, neg, zero)` is an abelian group ([`abelian_group`])
/// - `(var.set, mul, one)` is a monoid ([`monoid`])
/// - `mul` is distributive over `add` ([`distributive_binop`])
pub fn ring<S, A, N, M>(
    var: Var3<S>,
    add: Fun2<A>,
    mul: Fun2<M>,
    neg: Fun1<N>,
    zero: Elem<S>,
    one: Elem<S>,
) where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
    N: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {}, {}, {})` a ring?",
        var.set,
        add.name,
        mul.name,
        neg.name,
        zero.name,
        one.name,
    );

    abelian_group(var.clone(), add.as_ref(), neg, zero);
    monoid(var.clone(), mul.as_ref(), one);
    distributive_binop(var, add, mul);
}

/// Asserts that `(var.set, add, mul, neg, zero, one)` is a commutative ring.
///
/// It must hold:
/// - `(var.set, add, mul, neg, zero, one)` is a ring ([`ring`])
/// - `mul` is commutative ([`commutative_binop`])
pub fn commutative_ring<S, A, M, N>(
    var: Var3<S>,
    add: Fun2<A>,
    mul: Fun2<M>,
    neg: Fun1<N>,
    zero: Elem<S>,
    one: Elem<S>,
) where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
    N: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {}, {}, {})` a commutative ring?",
        var.set,
        add.name,
        mul.name,
        neg.name,
        zero.name,
        one.name,
    );

    let [a, b, c] = var.elems;
    let var_2 = var_2(var.set, [a.clone(), b.clone()]);
    let var_3 = var_3(var.set, [a, b, c]);

    ring(var_3, add, mul.as_ref(), neg, zero, one);
    commutative_binop(var_2, mul);
}

/// Asserts that `(var.set, add, mul, neg, inv, zero, one)` is a field.
///
/// It must hold:
/// - `(var.set, add, mul, neg, zero, one)` is a commutative ring ([`commutative_ring`])
/// - For `a` of `non_zero_var.set` the result of `inv(a)` is the inverse element of `a` regarding
/// to `mul` ([`inverse_elem_of_binop`])
#[allow(clippy::too_many_arguments)]
pub fn field<S, A, M, N, I>(
    var: Var3<S>,
    non_zero_var: Var2<S>,
    add: Fun2<A>,
    mul: Fun2<M>,
    neg: Fun1<N>,
    inv: Fun1<I>,
    zero: Elem<S>,
    one: Elem<S>,
) where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
    N: Fn(S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {}, {}, {}, {})` a field?",
        var.set,
        add.name,
        mul.name,
        neg.name,
        inv.name,
        zero.name,
        one.name,
    );

    commutative_ring(var, add, mul.as_ref(), neg, zero, one);
    inverse_elem_of_binop(non_zero_var, mul, inv);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{elem, fun_1, infix_fun_2, props, FateVarExt};

    #[test]
    fn semigroup_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("u64", ["x", "y", "z"], dice::u64(..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            props::semigroup(var, op);
        })
    }

    #[test]
    fn monoid_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("u64", ["x", "y", "z"], dice::u64(..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let e = elem("zero", 0);
            props::monoid(var, op, e);
        })
    }

    #[test]
    fn group_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            let e = elem("zero", 0);
            props::group(var, op, inv, e);
        })
    }

    #[test]
    fn abelian_group_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            let e = elem("zero", 0);
            props::abelian_group(var, op, inv, e);
        })
    }

    #[test]
    fn ring_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let neg = fun_1("-", |x: i64| -x);
            let zero = elem("zero", 0);
            let one = elem("one", 1);
            props::ring(var, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn commutative_ring_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let neg = fun_1("-", |x: i64| -x);
            let zero = elem("zero", 0);
            let one = elem("one", 1);
            props::commutative_ring(var, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn field_example() {
        // Are there any fields in libstd?
    }
}
