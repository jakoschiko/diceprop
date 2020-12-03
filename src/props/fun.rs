use std::fmt::Debug;

use crate::{hint_section, ops, Fun1, Fun2, Var1, Var2};

/// Asserts that the function `f` is idempotent.
///
/// For `a` of `var.set` it must hold:
/// - `f(a) == f(f(a))`
pub fn idempotent_fun<S, F>(var: Var1<S>, f: Fun1<F>)
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
pub fn left_inverse_fun<S, T, F, G>(var: Var1<S>, f: Fun1<F>, g: Fun1<G>)
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

/// Asserts that the function `f` is commutative.
///
/// For `a`, `b` of `var.set` it must hold:
/// - `f(a, b) == f(b, a)`
pub fn commutative_fun<S, R, O>(var: Var2<S>, f: Fun2<O>)
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
    use crate::{fun_1, fun_2, props, FateVarExt};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    #[test]
    fn idempotent_fun_example() {
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
            props::idempotent_fun(var, f);
        })
    }

    #[test]
    fn left_inverse_fun_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("f32", "x", dice::f32(..));
            let f = fun_1("to_string", |x: f32| x.to_string());
            let g = fun_1("from_string", |y: String| f32::from_str(&y).unwrap());
            props::left_inverse_fun(var, f, g);
        })
    }

    #[test]
    fn commutative_fun_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2(
                "BTreeSet<u8>",
                ["x", "y"],
                dice::b_tree_set(dice::u8(..), ..),
            );
            let f = fun_2("is_disjoin", |x, y| BTreeSet::<u8>::is_disjoint(&x, &y));
            props::commutative_fun(var, f);
        })
    }
}
