use std::fmt::Debug;

use crate::{hint_section, ops, var_1, var_2, var_3, Fun2, Var1, Var2, Var3};

/// Asserts that the binary relation `rel` is reflexive.
///
/// For `a` of `set` it must hold:
/// - `rel(a, a)`
pub fn reflexive_binrel<T, R>(set: Var1<T>, rel: Fun2<R>)
where
    T: Debug + Clone,
    R: FnOnce(T, T) -> bool,
{
    hint_section!("Is `{}` reflexive?", rel.name);

    let a = set.eval();

    ops::assert(rel.eval_once(a.clone(), a));
}

/// Asserts that the binary relation `rel` is symmetric.
///
/// For `a`, `b` of `set` it must hold:
/// - `rel(a, b) --> rel(b, a)`
pub fn symmetric_binrel<T, R>(set: Var2<T>, rel: Fun2<R>)
where
    T: Debug + Clone,
    R: Fn(T, T) -> bool,
{
    hint_section!("Is `{}` symmetric?", rel.name);

    let [a, b] = set.eval();

    ops::assert(ops::implies(rel.eval(a.clone(), b.clone()), rel.eval(b, a)));
}

/// Asserts that the binary relation `rel` is antisymmetric.
///
/// For `a`, `b` of `set` it must hold:
/// - `a != b && rel(a, b) --> !rel(b, a)`
pub fn antisymmetric_binrel<T, R>(set: Var2<T>, rel: Fun2<R>)
where
    T: Debug + Clone + PartialEq,
    R: Fn(T, T) -> bool,
{
    hint_section!("Is `{}` antisymmetric?", rel.name);

    let [a, b] = set.eval();

    ops::assert(ops::implies(
        ops::and(
            ops::ne(a.as_ref(), b.as_ref()),
            rel.eval(a.clone(), b.clone()),
        ),
        ops::not(rel.eval(b, a)),
    ));
}

/// Asserts that the binary relation `rel` is transitive.
///
/// For `a`, `b`, `c` of `set` it must hold:
/// - `rel(a, b) && rel(b, c) --> rel(a, c)`
pub fn transitive_binrel<T, R>(set: Var3<T>, rel: Fun2<R>)
where
    T: Debug + Clone,
    R: Fn(T, T) -> bool,
{
    hint_section!("Is `{}` transitive?", rel.name);

    let [a, b, c] = set.eval();

    ops::assert(ops::implies(
        ops::and(
            rel.eval(a.clone(), b.clone()),
            rel.eval(b.clone(), c.clone()),
        ),
        rel.eval(a, c),
    ));
}

/// Asserts that the binary relation `rel` is a partial equality relation.
///
/// It must hold:
/// - `rel` is symmetric ([`symmetric_binrel`])
/// - `rel` is transitive ([`transitive_binrel`])
pub fn partial_eq_binrel<T, R>(set: Var3<T>, rel: Fun2<R>)
where
    T: Debug + Clone,
    R: Fn(T, T) -> bool,
{
    hint_section!("Is `{}` a partial equality relation?", rel.name);

    let [a, b, c] = set.elems;
    let set_2 = var_2(set.set, [a.clone(), b.clone()]);
    let set_3 = var_3(set.set, [a, b, c]);

    symmetric_binrel(set_2, rel.as_ref());
    transitive_binrel(set_3, rel.as_ref());
}

/// Asserts that the binary relation `rel` is an equality relation.
///
/// It must hold:
/// - `rel` is reflexive ([`reflexive_binrel`])
/// - `rel` is symmetric ([`symmetric_binrel`])
/// - `rel` is transitive ([`transitive_binrel`])
pub fn eq_binrel<T, R>(set: Var3<T>, rel: Fun2<R>)
where
    T: Debug + Clone,
    R: Fn(T, T) -> bool,
{
    hint_section!("Is `{}` an equality relation?", rel.name);

    let [a, b, c] = set.elems;
    let set_1 = var_1(set.set, a.clone());
    let set_2 = var_2(set.set, [a.clone(), b.clone()]);
    let set_3 = var_3(set.set, [a, b, c]);

    reflexive_binrel(set_1, rel.as_ref());
    symmetric_binrel(set_2, rel.as_ref());
    transitive_binrel(set_3, rel.as_ref());
}

/// Asserts that the binary relation `erel` is equivalent to the relation `rel`.
///
/// For all `a`, `b` of `set` it must hold:
/// - `rel(a, b) <-> erel(a, b)`
pub fn equivalent_binrel<T, R, E>(set: Var2<T>, rel: Fun2<R>, erel: Fun2<E>)
where
    T: Debug + Clone,
    R: FnOnce(T, T) -> bool,
    E: FnOnce(T, T) -> bool,
{
    hint_section!("Is `{}` equivalent to `{}`?", erel.name, rel.name);

    let [a, b] = set.eval();

    ops::assert(ops::iff(
        rel.eval_once(a.clone(), b.clone()),
        erel.eval_once(a, b),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{infix_fun_2, props, FateVarExt};
    use dicetest::prelude::*;

    #[test]
    fn reflexive_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("==", |x, y| x == y);
            let set = fate.roll_var_1("u8", "x", dice::u8(..));
            props::reflexive_binrel(set, rel);
        })
    }

    #[test]
    fn symmetric_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("!=", |x, y| x != y);
            let set = fate.roll_var_2("f32", ["x", "y"], dice::any_f32());
            props::symmetric_binrel(set, rel);
        })
    }

    #[test]
    fn antisymmetric_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("<", |x, y| x < y);
            let set = fate.roll_var_2("u8", ["x", "y"], dice::u8(..));
            props::antisymmetric_binrel(set, rel);
        })
    }

    #[test]
    fn transitive_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("<", |x, y| x < y);
            let set = fate.roll_var_3("char", ["x", "y", "z"], dice::char());
            props::transitive_binrel(set, rel);
        })
    }

    #[test]
    fn partial_eq_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("==", |x, y| x == y);
            let set = fate.roll_var_3("f32", ["x", "y", "z"], dice::any_f32());
            props::partial_eq_binrel(set, rel);
        })
    }

    #[test]
    fn eq_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("==", |x, y| x == y);
            let set = fate.roll_var_3("String", ["x", "y", "z"], dice::string(dice::char(), ..));
            props::eq_binrel(set, rel);
        })
    }

    #[test]
    fn equivalent_binrel_example() {
        Dicetest::once().run(|mut fate| {
            let rel = infix_fun_2("==", |x, y| x == y);
            let erel = infix_fun_2("!!=", |x, y| !(x != y));
            let set = fate.roll_var_2("u8", ["x", "y"], dice::u8(..));
            props::equivalent_binrel(set, rel, erel);
        })
    }
}
