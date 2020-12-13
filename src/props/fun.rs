//! Properties for [functions].
//!
//! [functions]: https://en.wikipedia.org/wiki/Function_(mathematics)

use dicetest::hint_section;
use std::fmt::Debug;

use crate::{ops, Fun1, Fun2, Var1, Var2};

/// Asserts that the function `f` is idempotent.
///
/// For `a` of `var.set` it must hold:
/// - `f(a) == f(f(a))`
pub fn idempotent<S, F>(var: Var1<S>, f: Fun1<F>)
where
    S: Debug + Clone + PartialEq,
    F: Fn(S) -> S,
{
    hint_section!("Is `{}` idempotent?", f.name);

    let a = var.eval();
    let fa = f.eval(a);

    ops::assert(ops::eq(fa.as_ref(), f.eval(fa.clone()).as_ref()));
}

/// Asserts that the function `g` is the left inverse of function `f`.
///
/// For `a` of `var.set` it must hold:
/// - `g(f(a)) == a`
pub fn left_inverse<S, T, F, G>(var: Var1<S>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone + PartialEq,
    T: Debug,
    F: FnOnce(S) -> T,
    G: FnOnce(T) -> S,
{
    hint_section!("Is `{}` left inverse of `{}`?", g.name, f.name);

    let a = var.eval();

    ops::assert(ops::eq(
        g.eval_once(f.eval_once(a.clone())).as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the function `g` is the right inverse of function `f`.
///
/// For `a` of `var.set` it must hold:
/// - `f(g(a)) == a`
pub fn right_inverse<S, T, F, G>(var: Var1<T>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug,
    T: Debug + Clone + PartialEq,
    F: FnOnce(S) -> T,
    G: FnOnce(T) -> S,
{
    hint_section!("Is `{}` right inverse of `{}`?", g.name, f.name);

    let a = var.eval();

    ops::assert(ops::eq(
        f.eval_once(g.eval_once(a.clone())).as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the function `g` is the inverse of function `f`.
///
/// It must hold:
/// - `g` is the left inverse of `f` ([`left_inverse`])
/// - `g` is the right inverse of `f` ([`right_inverse`])
pub fn inverse<S, T, F, G>(var_s: Var1<S>, var_t: Var1<T>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone + PartialEq,
    T: Debug + Clone + PartialEq,
    F: Fn(S) -> T,
    G: Fn(T) -> S,
{
    hint_section!("Is `{}` inverse of `{}`?", g.name, f.name);

    left_inverse(var_s, f.as_ref(), g.as_ref());
    right_inverse(var_t, f, g);
}

/// Asserts that the function `g` is equal to the function `f`.
///
/// For `a` of `var.set` it must hold:
/// - `f(a) == g(a)`
pub fn equal_1<S, R, F, G>(var: Var1<S>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone,
    R: Debug + PartialEq,
    F: FnOnce(S) -> R,
    G: FnOnce(S) -> R,
{
    hint_section!("Is `{}` equivalent to `{}`?", g.name, f.name);

    let a = var.eval();

    ops::assert(ops::eq(
        f.eval_once(a.clone()).as_ref(),
        g.eval_once(a).as_ref(),
    ));
}

/// Asserts that the function `g` is equivalent to the function `f`.
///
/// For `a` of `var_s.set` and `b` of `var_t.set` it must hold:
/// - `f(a, b) == g(a, b)`
pub fn equal_2<S, T, R, F, G>(var_s: Var1<S>, var_t: Var1<T>, f: Fun2<F>, g: Fun2<G>)
where
    S: Debug + Clone,
    T: Debug + Clone,
    R: Debug + PartialEq,
    F: FnOnce(S, T) -> R,
    G: FnOnce(S, T) -> R,
{
    hint_section!("Is `{}` equivalent to `{}`?", g.name, f.name);

    let a = var_s.eval();
    let b = var_t.eval();

    ops::assert(ops::eq(
        f.eval_once(a.clone(), b.clone()).as_ref(),
        g.eval_once(a, b).as_ref(),
    ));
}

/// Asserts that the function `f` is commutative.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `f(a, b) == f(b, a)`
pub fn commutative<S, R, O>(var: Var2<S>, f: Fun2<O>)
where
    S: Debug + Clone,
    R: Debug + PartialEq,
    O: Fn(S, S) -> R,
{
    hint_section!("Is `{}` commutative?", f.name);

    let [a, b] = var.eval();

    ops::assert(ops::eq(
        f.eval(a.clone(), b.clone()).as_ref(),
        f.eval(b, a).as_ref(),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{fun_1, fun_2, postfix_fun_1, props, FateVarExt};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    #[test]
    fn idempotent_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1(
                "String",
                "x",
                dice::string(
                    dice::one_of_die_2(dice::one_of_slice(&[' ', '\n', '\t']), dice::char()),
                    ..,
                ),
            );
            let f = fun_1("trim", |x: String| x.trim().to_owned());
            props::fun::idempotent(var, f);
        })
    }

    #[test]
    fn left_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("f32", "x", dice::f32(..));
            let f = fun_1("to_string", |x: f32| x.to_string());
            let g = fun_1("from_string", |y: String| f32::from_str(&y).unwrap());
            props::fun::left_inverse(var, f, g);
        })
    }

    #[test]
    fn right_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("i8", "x", dice::i8(..));
            let f = fun_1("from_string", |x: String| i8::from_str(&x).unwrap());
            let g = fun_1("to_string", |y: i8| y.to_string());
            props::fun::right_inverse(var, f, g);
        })
    }

    #[test]
    fn inverse_example() {
        Dicetest::once().run(|mut fate| {
            let var_s = fate.roll_var_1("i8", "x", dice::i8(..));
            let var_t = fate.roll_var_1("u8", "y", dice::u8(..));
            let f = fun_1("from_string", |x: i8| x as u8);
            let g = fun_1("to_string", |y: u8| y as i8);
            props::fun::inverse(var_s, var_t, f, g);
        })
    }

    #[test]
    fn equal_1_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("u64", "x", dice::u64(..1000));
            let f = postfix_fun_1("+2", |x: u64| x + 2);
            let g = postfix_fun_1("+1+1", |x: u64| x + 1 + 1);
            props::fun::equal_1(var, f, g);
        })
    }

    #[test]
    fn equal_2_example() {
        Dicetest::once().run(|mut fate| {
            let var_s = fate.roll_var_1("BTreeSet<u8>", "xs", dice::b_tree_set(dice::u8(..), ..));
            let var_t = fate.roll_var_1("u8", "x", dice::u8(..));
            let f = fun_2("insert", |mut xs: BTreeSet<u8>, x| {
                xs.insert(x);
                xs
            });
            let g = fun_2("remove_and_insert", |mut xs: BTreeSet<u8>, x| {
                xs.remove(&x);
                xs.insert(x);
                xs
            });
            props::fun::equal_2(var_s, var_t, f, g);
        })
    }

    #[test]
    fn commutative_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2(
                "BTreeSet<u8>",
                ["x", "y"],
                dice::b_tree_set(dice::u8(..), ..),
            );
            let f = fun_2("is_disjoin", |x, y| BTreeSet::<u8>::is_disjoint(&x, &y));
            props::fun::commutative(var, f);
        })
    }
}
