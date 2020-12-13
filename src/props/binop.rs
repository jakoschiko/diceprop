//! Properties for [binary operations].
//!
//! [binary operations]: https://en.wikipedia.org/wiki/Binary_operation

use dicetest::hint_section;
use std::fmt::Debug;

use crate::{ops, props, Elem, Fun1, Fun2, Var1, Var2, Var3};

/// Asserts that the binary operation `op` is commutative.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `op(a, b) == op(b, a)`
pub fn commutative<S, O>(var: Var2<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    props::fun::commutative(var, op)
}

/// Asserts that the binary operation `op` is associative.
///
/// For `a`, `b`, `c` of `var.set` it must hold:
/// - `op(op(a, b), c) == op(a, op(b, c))`
pub fn associative<S, O>(var: Var3<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `{}` associative?", op.name);

    let [a, b, c] = var.eval();

    ops::assert(ops::eq(
        op.eval(op.eval(a.clone(), b.clone()), c.clone()).as_ref(),
        op.eval(a, op.eval(b, c)).as_ref(),
    ));
}

/// Asserts that the binary operation `mul` is left distributive over the binary operation `add`.
///
/// For `a`, `b`, `c` of `var.set` it must hold:
/// - `mul(a, add(b, c)) == add(mul(a, b), mul(a, c))`
pub fn left_distributive<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
{
    hint_section!("Is `{}` left distributive over `{}`?", mul.name, add.name,);

    let [a, b, c] = var.eval();

    ops::assert(ops::eq(
        mul.eval(a.clone(), add.eval(b.clone(), c.clone())).as_ref(),
        add.eval(
            mul.eval(a.clone(), b.clone()),
            mul.eval(a.clone(), c.clone()),
        )
        .as_ref(),
    ));
}

/// Asserts that the binary operation `mul` is right distributive over the binary operation `add`.
///
/// For `a`, `b`, `c` of `var.set` it must hold:
/// - `mul(add(a, b), c) == add(mul(a, c), mul(b, c))`
pub fn right_distributive<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
{
    hint_section!("Is `{}` right distributive over `{}`?", mul.name, add.name,);

    let [a, b, c] = var.eval();

    ops::assert(ops::eq(
        mul.eval(add.eval(a.clone(), b.clone()), c.clone()).as_ref(),
        add.eval(
            mul.eval(a.clone(), c.clone()),
            mul.eval(b.clone(), c.clone()),
        )
        .as_ref(),
    ));
}

/// Asserts that the binary operation `mul` is distributive over the binary operation `add`.
///
/// It must hold:
/// - `mul` is left distributive over `add` ([`left_distributive`])
/// - `mul` is right distributive over `add` ([`right_distributive`])
pub fn distributive<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
{
    hint_section!("Is `{}` distributive over `{}`?", mul.name, add.name);

    left_distributive(var.clone(), add.as_ref(), mul.as_ref());
    right_distributive(var, add, mul);
}

/// Asserts that `e` is the left identity element of the binary operation `op`.
///
/// For `a` of `var.set` it must hold:
/// - `op(e, a) == a`
pub fn left_identity_elem<S, O>(var: Var1<S>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: FnOnce(S, S) -> S,
{
    hint_section!("Is `{}` left identity element of `{}`?", e.name, op.name);

    let a = var.eval();
    let e = e.eval();

    ops::assert(ops::eq(op.eval_once(e, a.clone()).as_ref(), a.as_ref()));
}

/// Asserts that `e` is the right identity element of the binary operation `op`.
///
/// For `a` of `var.set` it must hold:
/// - `op(a, e) == a`
pub fn right_identity_elem<S, O>(var: Var1<S>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: FnOnce(S, S) -> S,
{
    hint_section!("Is `{}` right identity element of `{}`?", e.name, op.name);

    let a = var.eval();
    let e = e.eval();

    ops::assert(ops::eq(op.eval_once(a.clone(), e).as_ref(), a.as_ref()));
}

/// Asserts that `e` is the identity element of the binary operation `op`.
///
/// It must hold:
/// - `e` is the left identity element of `op` ([`left_identity_elem`])
/// - `e` is the right identity element of `op` ([`right_identity_elem`])
pub fn identity_elem<S, O>(var: Var1<S>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `{}` identity element of `{}`?", e.name, op.name);

    left_identity_elem(var.clone(), op.as_ref(), e.clone());
    right_identity_elem(var, op, e);
}

/// Asserts that the function `inv` returns the left inverse element regarding
/// to the binary operation `op`.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `op(b, op(inv(a), a)) == b`
pub fn left_inverse_elem<S, O, I>(var: Var2<S>, op: Fun2<O>, inv: Fun1<I>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Does `{}` return left inverse element regarding to `{}`?",
        inv.name,
        op.name,
    );

    let [a, b] = var.eval();

    ops::assert(ops::eq(
        op.eval(b.clone(), op.eval(inv.eval(a.clone()), a.clone()))
            .as_ref(),
        b.as_ref(),
    ));
}

/// Asserts that the function `inv` returns the right inverse element regarding
/// to the binary operation `op`.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `op(op(a, inv(a)), b) == b`
pub fn right_inverse_elem<S, O, I>(var: Var2<S>, op: Fun2<O>, inv: Fun1<I>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Does `{}` return right inverse element regarding to `{}`?",
        inv.name,
        op.name
    );

    let [a, b] = var.eval();

    ops::assert(ops::eq(
        op.eval(op.eval(a.clone(), inv.eval(a.clone())), b.clone())
            .as_ref(),
        b.as_ref(),
    ));
}

/// Asserts the function `inv` returns the inverse element that regarding
/// to the binary operation `op`.
///
/// It must hold:
/// - `inv` returns the left inverse element regarding to `op`
/// ([`left_inverse_elem`])
/// - `inv` returns the right inverse element regarding to `op`
/// ([`right_inverse_elem`])
pub fn inverse_elem<S, O, I>(var: Var2<S>, op: Fun2<O>, inv: Fun1<I>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S) -> S,
{
    hint_section!(
        "Does `{}` return inverse element regarding to `{}`?",
        inv.name,
        op.name
    );

    left_inverse_elem(var.clone(), op.as_ref(), inv.as_ref());
    right_inverse_elem(var, op, inv);
}

/// Asserts that the binary operation `invop` is the left inverse of the binary operation `op`.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `invop(op(a, b), b) == a`
pub fn left_inverse<S, O, I>(var: Var2<S>, op: Fun2<O>, invop: Fun2<I>)
where
    S: Debug + Clone + PartialEq,
    O: FnOnce(S, S) -> S,
    I: FnOnce(S, S) -> S,
{
    hint_section!("Is `{}` left inverse of `{}`?", invop.name, op.name);

    let [a, b] = var.eval();

    ops::assert(ops::eq(
        invop
            .eval_once(op.eval_once(a.clone(), b.clone()), b)
            .as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the binary operation `invop` is the right inverse of the binary operation `op`.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `op(invop(a, b), b) == a`
pub fn right_inverse<S, O, I>(var: Var2<S>, op: Fun2<O>, invop: Fun2<I>)
where
    S: Debug + Clone + PartialEq,
    O: FnOnce(S, S) -> S,
    I: FnOnce(S, S) -> S,
{
    hint_section!("Is `{}` right inverse of `{}`?", invop.name, op.name);

    let [a, b] = var.eval();

    ops::assert(ops::eq(
        op.eval_once(invop.eval_once(a.clone(), b.clone()), b)
            .as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the binary operation `invop` is the inverse of the binary operation `op`.
///
/// It must hold:
/// - `invop` is the left inverse of `op` ([`left_inverse`])
/// - `invop` is the right inverse of `op` ([`right_inverse`])
pub fn inverse<S, O, I>(var: Var2<S>, op: Fun2<O>, invop: Fun2<I>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
    I: Fn(S, S) -> S,
{
    hint_section!("Is `{}` inverse of `{}`?", invop.name, op.name);

    left_inverse(var.clone(), op.as_ref(), invop.as_ref());
    right_inverse(var, op, invop);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;
    use std::collections::BTreeSet;

    use crate::{elem, fun_1, fun_2, infix_fun_2, props, FateVarExt};

    #[test]
    fn commutative_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2(
                "BTreeSet<u8>",
                ["x", "y"],
                dice::b_tree_set(dice::u8(..), ..),
            );
            let op = fun_2("intersection", |x, y| {
                BTreeSet::<u8>::intersection(&x, &y)
                    .cloned()
                    .collect::<BTreeSet<_>>()
            });
            props::binop::commutative(var, op);
        })
    }

    #[test]
    fn associative_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("Vec<u8>", ["x", "y", "z"], dice::vec(dice::u8(..), ..));
            let op = fun_2("append", |mut x, mut y| {
                Vec::<u8>::append(&mut x, &mut y);
                x
            });
            props::binop::associative(var, op);
        })
    }

    #[test]
    fn left_distributive_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            props::binop::left_distributive(var, add, mul);
        })
    }

    #[test]
    fn right_distributive_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            props::binop::right_distributive(var, add, mul);
        })
    }

    #[test]
    fn distributive_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            props::binop::distributive(var, add, mul);
        })
    }

    #[test]
    fn left_identity_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("i8", "x", dice::i8(..));
            let op = infix_fun_2("+", |x, y| x + y);
            let e = elem("zero", 0);
            props::binop::left_identity_elem(var, op, e);
        })
    }

    #[test]
    fn right_identity_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("i8", "x", dice::i8(..));
            let op = infix_fun_2("*", |x, y| x * y);
            let e = elem("one", 1);
            props::binop::right_identity_elem(var, op, e);
        })
    }

    #[test]
    fn identity_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("f32", "x", dice::f32(..));
            let op = infix_fun_2("+", |x, y| x + y);
            let e = elem("zero", 0.0);
            props::binop::identity_elem(var, op, e);
        })
    }

    #[test]
    fn left_inverse_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            props::binop::left_inverse_elem(var, op, inv);
        })
    }

    #[test]
    fn right_inverse_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            props::binop::right_inverse_elem(var, op, inv);
        })
    }

    #[test]
    fn inverse_elem_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let inv = fun_1("-", |x: i64| -x);
            props::binop::inverse_elem(var, op, inv);
        })
    }

    #[test]
    fn left_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let invop = infix_fun_2("-", |x, y| x - y);
            props::binop::left_inverse(var, op, invop);
        })
    }

    #[test]
    fn right_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let invop = infix_fun_2("-", |x, y| x - y);
            props::binop::right_inverse(var, op, invop);
        })
    }

    #[test]
    fn inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("i64", ["x", "y"], dice::i64(-1000..=1000));
            let op = infix_fun_2("+", |x, y| x + y);
            let invop = infix_fun_2("-", |x, y| x - y);
            props::binop::inverse(var, op, invop);
        })
    }
}
