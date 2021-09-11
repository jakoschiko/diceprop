//! Properties for [algebraic structures].
//!
//! [algebraic structures]: https://en.wikipedia.org/wiki/Algebraic_structure

use dicetest::hint_section;
use std::fmt::Debug;

use crate::props::binop::{associative, commutative, distributive, identity_elem, inverse_elem};
use crate::{Elem, Fun1, Fun2, Var};

/// Asserts that `(var.set, op)` is a [semigroup].
///
/// It must hold:
/// - `op` is associative  ([`associative`])
///
/// [semigroup]: https://en.wikipedia.org/wiki/Semigroup
pub fn semigroup<S, O>(var: Var<S, 3>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {})` a semigroup?", var.set, op.name);

    associative(var, op);
}

/// Asserts that `(var.set, op, e)` is a [monoid].
///
/// It must hold:
/// - `(var.set, op)` is a semigroup ([`semigroup`])
/// - `e` is the identity element of `op` ([`identity_elem`])
///
/// [monoid]: https://en.wikipedia.org/wiki/Monoid
pub fn monoid<S, O>(var: Var<S, 3>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {}, {})` a monoid?", var.set, op.name, e.name,);

    let [a, b, c] = var.elems;
    let var_1 = Var::new(var.set, [a.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    semigroup(var_3, op.as_ref());
    identity_elem(var_1, op, e)
}

/// Asserts that `(var.set, op, inv, e)` is a [group].
///
/// It must hold:
/// - `(var.set, op, e)` is a monoid ([`monoid`])
/// - `inv` returns the inverse elements regarding to `op` ([`inverse_elem`])
///
/// [group]: https://en.wikipedia.org/wiki/Group_(mathematics)
pub fn group<S, O, I>(var: Var<S, 3>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
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
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    monoid(var_3, op.as_ref(), e);
    inverse_elem(var_2, op, inv);
}

/// Asserts that `(var.set, op, inv, e)` is an [abelian group].
///
/// It must hold:
/// - `(var.set, op, inv, e)` is a group ([`group`])
/// - `op` is commutative ([`commutative`])
///
/// [abelian group]: https://en.wikipedia.org/wiki/Abelian_group
pub fn abelian_group<S, O, I>(var: Var<S, 3>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
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
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    group(var_3, op.as_ref(), inv, e);
    commutative(var_2, op);
}

/// Asserts that `(var.set, add, mul, neg, zero, one)` is a [ring].
///
/// It must hold:
/// - `(var.set, add, neg, zero)` is an abelian group ([`abelian_group`])
/// - `(var.set, mul, one)` is a monoid ([`monoid`])
/// - `mul` is distributive over `add` ([`distributive`])
///
/// [ring]: https://en.wikipedia.org/wiki/Ring_(mathematics)
pub fn ring<S, A, N, M>(
    var: Var<S, 3>,
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
    distributive(var, add, mul);
}

/// Asserts that `(var.set, add, mul, neg, zero, one)` is a [commutative ring].
///
/// It must hold:
/// - `(var.set, add, mul, neg, zero, one)` is a ring ([`ring`])
/// - `mul` is commutative ([`commutative`])
///
/// [commutative ring]: https://en.wikipedia.org/wiki/Commutative_ring
pub fn commutative_ring<S, A, M, N>(
    var: Var<S, 3>,
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
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    ring(var_3, add, mul.as_ref(), neg, zero, one);
    commutative(var_2, mul);
}

/// Asserts that `(var.set, add, mul, neg, inv, zero, one)` is a [field].
///
/// It must hold:
/// - `(var.set, add, mul, neg, zero, one)` is a commutative ring ([`commutative_ring`])
/// - For `a` of `non_zero_var.set` the result of `inv(a)` is the inverse element of `a` regarding
/// to `mul` ([`inverse_elem`])
///
/// [field]: https://en.wikipedia.org/wiki/Field_(mathematics)
#[allow(clippy::too_many_arguments)]
pub fn field<S, A, M, N, I>(
    var: Var<S, 3>,
    non_zero_var: Var<S, 2>,
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
    inverse_elem(non_zero_var, mul, inv);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{elem, fun_1, infix_fun_2, props, FateVarExt, Set};

    #[test]
    fn semigroup_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u64", dice::u64(..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let op = infix_fun_2("+", |x, y| x + y);
            props::algebra::semigroup(var, op);
        })
    }

    #[test]
    fn monoid_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u64", dice::u64(..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let op = infix_fun_2("+", |x, y| x + y);
            let e = elem("zero", 0);
            props::algebra::monoid(var, op, e);
        })
    }

    #[test]
    fn group_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            let e = elem("zero", 0);
            props::algebra::group(var, op, inv, e);
        })
    }

    #[test]
    fn abelian_group_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            let e = elem("zero", 0);
            props::algebra::abelian_group(var, op, inv, e);
        })
    }

    #[test]
    fn ring_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let neg = fun_1("-", |x: i64| -x);
            let zero = elem("zero", 0);
            let one = elem("one", 1);
            props::algebra::ring(var, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn commutative_ring_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let var = fate.roll_var(["x", "y", "z"], set);
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let neg = fun_1("-", |x: i64| -x);
            let zero = elem("zero", 0);
            let one = elem("one", 1);
            props::algebra::commutative_ring(var, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn field_example() {
        // Are there any fields in libstd?
    }
}
