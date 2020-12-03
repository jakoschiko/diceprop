use std::fmt::Debug;

use crate::props::commutative_fun;
use crate::{hint_section, ops, Elem, Fun2, Var1, Var2, Var3};

/// Asserts that the binary operation `op` is commutative.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `op(a, b) == op(b, a)`
pub fn commutative_binop<S, O>(var: Var2<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    commutative_fun(var, op)
}

/// Asserts that the binary operation `op` is associative.
///
/// For `a`, `b`, `c` of `var.set` it must hold:
/// - `op(op(a, b), c) == op(a, op(b, c))`
pub fn associative_binop<S, O>(var: Var3<S>, op: Fun2<O>)
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
pub fn left_distributive_binop<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
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
pub fn right_distributive_binop<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
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
/// - `mul` is left distributive over the `add` ([`left_distributive_binop`])
/// - `mul` is right distributive over the `add` ([`right_distributive_binop`])
pub fn distributive_binop<S, A, M>(var: Var3<S>, add: Fun2<A>, mul: Fun2<M>)
where
    S: Debug + Clone + PartialEq,
    A: Fn(S, S) -> S,
    M: Fn(S, S) -> S,
{
    hint_section!("Is `{}` distributive over `{}`?", mul.name, add.name);

    left_distributive_binop(var.clone(), add.as_ref(), mul.as_ref());
    right_distributive_binop(var, add, mul);
}

/// Asserts that `e` is the left identity element of the binary operation `op`.
///
/// For `a` of `var.set` it must hold:
/// - `op(e, a) == a`
pub fn left_identity_elem_of_binop<S, O>(var: Var1<S>, op: Fun2<O>, e: Elem<S>)
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
pub fn right_identity_elem_of_binop<S, O>(var: Var1<S>, op: Fun2<O>, e: Elem<S>)
where
    S: Debug + Clone + PartialEq,
    O: FnOnce(S, S) -> S,
{
    hint_section!("Is `{}` right identity element of `{}`?", e.name, op.name);

    let a = var.eval();
    let e = e.eval();

    ops::assert(ops::eq(op.eval_once(a.clone(), e).as_ref(), a.as_ref()));
}

#[cfg(test)]
mod tests {
    use crate::{elem, fun_2, infix_fun_2, props, FateVarExt};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;

    #[test]
    fn commutative_binop_example() {
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
            props::commutative_binop(var, op);
        })
    }

    #[test]
    fn associative_binop_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("Vec<u8>", ["x", "y", "z"], dice::vec(dice::u8(..), ..));
            let op = fun_2("append", |mut x, mut y| {
                Vec::<u8>::append(&mut x, &mut y);
                x
            });
            props::associative_binop(var, op);
        })
    }

    #[test]
    fn left_distributive_binop_example() {
        Dicetest::once().run(|mut fate| {
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            props::left_distributive_binop(var, add, mul);
        })
    }

    #[test]
    fn right_distributive_binop_example() {
        Dicetest::once().run(|mut fate| {
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            props::right_distributive_binop(var, add, mul);
        })
    }

    #[test]
    fn distributive_binop_example() {
        Dicetest::once().run(|mut fate| {
            let add = infix_fun_2("+", |x, y| x + y);
            let mul = infix_fun_2("*", |x, y| x * y);
            let var = fate.roll_var_3("i64", ["x", "y", "z"], dice::i64(-1000..=1000));
            props::distributive_binop(var, add, mul);
        })
    }

    #[test]
    fn left_identity_elem_of_binop_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("i8", "x", dice::i8(..));
            let op = infix_fun_2("+", |x, y| x + y);
            let e = elem("zero", 0);
            props::left_identity_elem_of_binop(var, op, e);
        })
    }

    #[test]
    fn right_identity_elem_of_binop_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("i8", "x", dice::i8(..));
            let op = infix_fun_2("*", |x, y| x * y);
            let e = elem("one", 1);
            props::right_identity_elem_of_binop(var, op, e);
        })
    }
}
