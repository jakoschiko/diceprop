//! Properties for [functions].
//!
//! [functions]: https://en.wikipedia.org/wiki/Function_(mathematics)

use dicetest::hint_section;
use std::fmt::Debug;

use crate::{ops, Fun1, Fun2, Vars};

/// Asserts that the function `f` is [idempotent].
///
/// For all `a` of `vars.set` it must hold:
/// - `f(a) == f(f(a))`
///
/// [idempotent]: https://en.wikipedia.org/wiki/Idempotence
pub fn idempotent<S, F>(vars: Vars<S, 1>, f: Fun1<F>)
where
    S: Debug + Clone + PartialEq,
    F: Fn(S) -> S,
{
    hint_section!("Is `{}` idempotent?", f.name);

    let [a] = vars.eval();
    let fa = f.eval(a);

    ops::assert(ops::eq(fa.as_ref(), f.eval(fa.clone()).as_ref()));
}

/// Asserts that the function `g` is the [left inverse] of function `f`.
///
/// For all `a` of `vars.set` it must hold:
/// - `g(f(a)) == a`
///
/// [left inverse]: https://en.wikipedia.org/wiki/Inverse_function#Left_and_right_inverses
pub fn left_inverse<S, T, F, G>(vars: Vars<S, 1>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone + PartialEq,
    T: Debug,
    F: FnOnce(S) -> T,
    G: FnOnce(T) -> S,
{
    hint_section!("Is `{}` left inverse of `{}`?", g.name, f.name);

    let [a] = vars.eval();

    ops::assert(ops::eq(
        g.eval_once(f.eval_once(a.clone())).as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the function `g` is the [right inverse] of function `f`.
///
/// For all `a` of `vars.set` it must hold:
/// - `f(g(a)) == a`
///
/// [right inverse]: https://en.wikipedia.org/wiki/Inverse_function#Left_and_right_inverses
pub fn right_inverse<S, T, F, G>(vars: Vars<T, 1>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug,
    T: Debug + Clone + PartialEq,
    F: FnOnce(S) -> T,
    G: FnOnce(T) -> S,
{
    hint_section!("Is `{}` right inverse of `{}`?", g.name, f.name);

    let [a] = vars.eval();

    ops::assert(ops::eq(
        f.eval_once(g.eval_once(a.clone())).as_ref(),
        a.as_ref(),
    ));
}

/// Asserts that the function `g` is the [inverse] of function `f`.
///
/// It must hold:
/// - `g` is the left inverse of `f` ([`left_inverse`])
/// - `g` is the right inverse of `f` ([`right_inverse`])
///
/// [inverse]: https://en.wikipedia.org/wiki/Inverse_function
pub fn inverse<S, T, F, G>(vars_s: Vars<S, 1>, vars_t: Vars<T, 1>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone + PartialEq,
    T: Debug + Clone + PartialEq,
    F: Fn(S) -> T,
    G: Fn(T) -> S,
{
    hint_section!("Is `{}` inverse of `{}`?", g.name, f.name);

    left_inverse(vars_s, f.as_ref(), g.as_ref());
    right_inverse(vars_t, f, g);
}

/// Asserts that the function `g` is equal to the function `f`.
///
/// For all `a` of `vars.set` it must hold:
/// - `f(a) == g(a)`
pub fn equal_1<S, R, F, G>(vars: Vars<S, 1>, f: Fun1<F>, g: Fun1<G>)
where
    S: Debug + Clone,
    R: Debug + PartialEq,
    F: FnOnce(S) -> R,
    G: FnOnce(S) -> R,
{
    hint_section!("Is `{}` equal to `{}`?", g.name, f.name);

    let [a] = vars.eval();

    ops::assert(ops::eq(
        f.eval_once(a.clone()).as_ref(),
        g.eval_once(a).as_ref(),
    ));
}

/// Asserts that the function `g` is equal to the function `f`.
///
/// For all `a` of `vars_s.set` and `b` of `vars_t.set` it must hold:
/// - `f(a, b) == g(a, b)`
pub fn equal_2<S, T, R, F, G>(vars_s: Vars<S, 1>, vars_t: Vars<T, 1>, f: Fun2<F>, g: Fun2<G>)
where
    S: Debug + Clone,
    T: Debug + Clone,
    R: Debug + PartialEq,
    F: FnOnce(S, T) -> R,
    G: FnOnce(S, T) -> R,
{
    hint_section!("Is `{}` equal to `{}`?", g.name, f.name);

    let [a] = vars_s.eval();
    let [b] = vars_t.eval();

    ops::assert(ops::eq(
        f.eval_once(a.clone(), b.clone()).as_ref(),
        g.eval_once(a, b).as_ref(),
    ));
}

/// Asserts that the function `f` is [commutative].
///
/// For `a`, `b` of `vars.set` it must hold:
/// - `f(a, b) == f(b, a)`
///
/// [commutative]: https://en.wikipedia.org/wiki/Commutative_property#Mathematical_definitions
pub fn commutative<S, R, O>(vars: Vars<S, 2>, f: Fun2<O>)
where
    S: Debug + Clone,
    R: Debug + PartialEq,
    O: Fn(S, S) -> R,
{
    hint_section!("Is `{}` commutative?", f.name);

    let [a, b] = vars.eval();

    ops::assert(ops::eq(
        f.eval(a.clone(), b.clone()).as_ref(),
        f.eval(b, a).as_ref(),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{props, Fun1, Fun2, Set};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;
    use std::str::FromStr;

    #[test]
    fn idempotent_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new(
                "String",
                dice::string(
                    dice::one_of_die().two(dice::one_of_slice(&[' ', '\n', '\t']), dice::char()),
                    ..,
                ),
            );
            let vars = fate.roll(set.vars(["x"]));
            let f = Fun1::new("trim", |x: String| x.trim().to_owned());
            props::fun::idempotent(vars, f);
        })
    }

    #[test]
    fn left_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("f32", dice::f32(..));
            let vars = fate.roll(set.vars(["x"]));
            let f = Fun1::new("to_string", |x: f32| x.to_string());
            let g = Fun1::new("from_string", |y: String| f32::from_str(&y).unwrap());
            props::fun::left_inverse(vars, f, g);
        })
    }

    #[test]
    fn right_inverse_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("i8", dice::i8(..));
            let vars = fate.roll(set.vars(["x"]));
            let f = Fun1::new("from_string", |x: String| i8::from_str(&x).unwrap());
            let g = Fun1::new("to_string", |y: i8| y.to_string());
            props::fun::right_inverse(vars, f, g);
        })
    }

    #[test]
    fn inverse_example() {
        Dicetest::once().run(|mut fate| {
            let set_s = Set::new("i8", dice::i8(..));
            let set_t = Set::new("u8", dice::u8(..));
            let vars_s = fate.roll(set_s.vars(["x"]));
            let vars_t = fate.roll(set_t.vars(["y"]));
            let f = Fun1::new("from_string", |x: i8| x as u8);
            let g = Fun1::new("to_string", |y: u8| y as i8);
            props::fun::inverse(vars_s, vars_t, f, g);
        })
    }

    #[test]
    fn equal_1_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u64", dice::u64(..1000));
            let vars = fate.roll(set.vars(["x"]));
            let f = Fun1::postfix("+2", |x: u64| x + 2);
            let g = Fun1::postfix("+1+1", |x: u64| x + 1 + 1);
            props::fun::equal_1(vars, f, g);
        })
    }

    #[test]
    fn equal_2_example() {
        Dicetest::once().run(|mut fate| {
            let set_s = Set::new("BTreeSet<u8>", dice::b_tree_set(dice::u8(..), ..));
            let set_t = Set::new("u8", dice::u8(..));
            let vars_s = fate.roll(set_s.vars(["xs"]));
            let vars_t = fate.roll(set_t.vars(["x"]));
            let f = Fun2::new("insert", |mut xs: BTreeSet<u8>, x| {
                xs.insert(x);
                xs
            });
            let g = Fun2::new("remove_and_insert", |mut xs: BTreeSet<u8>, x| {
                xs.remove(&x);
                xs.insert(x);
                xs
            });
            props::fun::equal_2(vars_s, vars_t, f, g);
        })
    }

    #[test]
    fn commutative_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("BTreeSet<u8>", dice::b_tree_set(dice::u8(..), ..));
            let vars = fate.roll(set.vars(["x", "y"]));
            let f = Fun2::new("is_disjoin", |x, y| BTreeSet::<u8>::is_disjoint(&x, &y));
            props::fun::commutative(vars, f);
        })
    }
}
