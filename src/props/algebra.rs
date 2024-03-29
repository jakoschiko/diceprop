//! Properties for [algebraic structures].
//!
//! [algebraic structures]: https://en.wikipedia.org/wiki/Algebraic_structure

use dicetest::hint_section;
use std::fmt::Debug;

use crate::props::binop::{associative, commutative, distributive, identity_elem, inverse_elem};
use crate::{Elem, Fun1, Fun2, Vars};

/// Asserts that `(vars.set, op)` is a [semigroup].
///
/// It must hold:
/// - `op` is associative  ([`associative`])
///
/// [semigroup]: https://en.wikipedia.org/wiki/Semigroup
pub fn semigroup<S, O>(vars: Vars<S, 3>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {})` a semigroup?", vars.set, op.name);

    associative(vars, op);
}

/// Asserts that `(vars.set, op, e)` is a [monoid].
///
/// It must hold:
/// - `(vars.set, op)` is a semigroup ([`semigroup`])
/// - `e` is the identity element of `op` ([`identity_elem`])
///
/// [monoid]: https://en.wikipedia.org/wiki/Monoid
pub fn monoid<S, O>(vars: Vars<S, 3>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {}, {})` a monoid?", vars.set, op.name, e.name,);

    let [a, b, c] = vars.elems;
    let vars_1 = Vars::new(vars.set, [a.clone()]);
    let vars_3 = Vars::new(vars.set, [a, b, c]);

    semigroup(vars_3, op.as_ref());
    identity_elem(vars_1, op, e)
}

/// Asserts that `(vars.set, op, inv, e)` is a [group].
///
/// It must hold:
/// - `(vars.set, op, e)` is a monoid ([`monoid`])
/// - `inv` returns the inverse elements regarding to `op` ([`inverse_elem`])
///
/// [group]: https://en.wikipedia.org/wiki/Group_(mathematics)
pub fn group<S, O, I>(vars: Vars<S, 3>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {})` a group?",
        vars.set,
        op.name,
        inv.name,
        e.name,
    );

    let [a, b, c] = vars.elems;
    let vars_2 = Vars::new(vars.set, [a.clone(), b.clone()]);
    let vars_3 = Vars::new(vars.set, [a, b, c]);

    monoid(vars_3, op.as_ref(), e);
    inverse_elem(vars_2, op, inv);
}

/// Asserts that `(vars.set, op, inv, e)` is an [abelian group].
///
/// It must hold:
/// - `(vars.set, op, inv, e)` is a group ([`group`])
/// - `op` is commutative ([`commutative`])
///
/// [abelian group]: https://en.wikipedia.org/wiki/Abelian_group
pub fn abelian_group<S, O, I>(vars: Vars<S, 3>, op: Fun2<O>, inv: Fun1<I>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Is `({}, {}, {}, {})` an abelian group?",
        vars.set,
        op.name,
        inv.name,
        e.name,
    );

    let [a, b, c] = vars.elems;
    let vars_2 = Vars::new(vars.set, [a.clone(), b.clone()]);
    let vars_3 = Vars::new(vars.set, [a, b, c]);

    group(vars_3, op.as_ref(), inv, e);
    commutative(vars_2, op);
}

/// Asserts that `(vars.set, add, mul, neg, zero, one)` is a [ring].
///
/// It must hold:
/// - `(vars.set, add, neg, zero)` is an abelian group ([`abelian_group`])
/// - `(vars.set, mul, one)` is a monoid ([`monoid`])
/// - `mul` is distributive over `add` ([`distributive`])
///
/// [ring]: https://en.wikipedia.org/wiki/Ring_(mathematics)
pub fn ring<S, A, N, M>(
    vars: Vars<S, 3>,
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
        vars.set,
        add.name,
        mul.name,
        neg.name,
        zero.name,
        one.name,
    );

    abelian_group(vars.clone(), add.as_ref(), neg, zero);
    monoid(vars.clone(), mul.as_ref(), one);
    distributive(vars, add, mul);
}

/// Asserts that `(vars.set, add, mul, neg, zero, one)` is a [commutative ring].
///
/// It must hold:
/// - `(vars.set, add, mul, neg, zero, one)` is a ring ([`ring`])
/// - `mul` is commutative ([`commutative`])
///
/// [commutative ring]: https://en.wikipedia.org/wiki/Commutative_ring
pub fn commutative_ring<S, A, M, N>(
    vars: Vars<S, 3>,
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
        vars.set,
        add.name,
        mul.name,
        neg.name,
        zero.name,
        one.name,
    );

    let [a, b, c] = vars.elems;
    let vars_2 = Vars::new(vars.set, [a.clone(), b.clone()]);
    let vars_3 = Vars::new(vars.set, [a, b, c]);

    ring(vars_3, add, mul.as_ref(), neg, zero, one);
    commutative(vars_2, mul);
}

/// Asserts that `(vars.set, add, mul, neg, inv, zero, one)` is a [field].
///
/// It must hold:
/// - `(vars.set, add, mul, neg, zero, one)` is a commutative ring ([`commutative_ring`])
/// - For `a` of `non_zero_vars.set` the result of `inv(a)` is the inverse element of `a` regarding
/// to `mul` ([`inverse_elem`])
///
/// [field]: https://en.wikipedia.org/wiki/Field_(mathematics)
#[allow(clippy::too_many_arguments)]
pub fn field<S, A, M, N, I>(
    vars: Vars<S, 3>,
    non_zero_vars: Vars<S, 2>,
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
        vars.set,
        add.name,
        mul.name,
        neg.name,
        inv.name,
        zero.name,
        one.name,
    );

    commutative_ring(vars, add, mul.as_ref(), neg, zero, one);
    inverse_elem(non_zero_vars, mul, inv);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{props, Elem, Fun1, Fun2, Set};

    #[test]
    fn semigroup_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u64", dice::u64(..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let op = Fun2::infix("+", |x, y| x + y);
            props::algebra::semigroup(vars, op);
        })
    }

    #[test]
    fn monoid_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u64", dice::u64(..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let op = Fun2::infix("+", |x, y| x + y);
            let e = Elem::new("zero", 0);
            props::algebra::monoid(vars, op, e);
        })
    }

    #[test]
    fn group_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let op = Fun2::infix("+", |x, y| x + y);
            let inv = Fun1::new("-", |x: i64| -x);
            let e = Elem::new("zero", 0);
            props::algebra::group(vars, op, inv, e);
        })
    }

    #[test]
    fn abelian_group_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let op = Fun2::infix("+", |x, y| x + y);
            let inv = Fun1::new("-", |x: i64| -x);
            let e = Elem::new("zero", 0);
            props::algebra::abelian_group(vars, op, inv, e);
        })
    }

    #[test]
    fn ring_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let add = Fun2::infix("+", |x, y| x + y);
            let mul = Fun2::infix("*", |x, y| x * y);
            let neg = Fun1::new("-", |x: i64| -x);
            let zero = Elem::new("zero", 0);
            let one = Elem::new("one", 1);
            props::algebra::ring(vars, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn commutative_ring_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i64", dice::i64(-1000..=1000));
            let vars = fate.roll(set.vars(["x", "y", "z"]));
            let add = Fun2::infix("+", |x, y| x + y);
            let mul = Fun2::infix("*", |x, y| x * y);
            let neg = Fun1::new("-", |x: i64| -x);
            let zero = Elem::new("zero", 0);
            let one = Elem::new("one", 1);
            props::algebra::commutative_ring(vars, add, mul, neg, zero, one);
        })
    }

    #[test]
    fn field_example() {
        // Are there any fields in libstd?
    }
}
