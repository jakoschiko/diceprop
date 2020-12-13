//! Properties for [binary relations].
//!
//! [binary relations]: https://en.wikipedia.org/wiki/Binary_relation

use dicetest::hint_section;
use std::fmt::Debug;

use crate::{ops, var_1, var_2, var_3, Fun2, Var1, Var2, Var3};

/// Asserts that the binary relation `rel` is [reflexive].
///
/// For all `a` of `var.set` it must hold:
/// - `rel(a, a)`
///
/// [reflexive]: https://en.wikipedia.org/wiki/Reflexive_relation
pub fn reflexive<S, R>(var: Var1<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: FnOnce(S, S) -> bool,
{
    hint_section!("Is `{}` reflexive?", rel.name);

    let a = var.eval();

    ops::assert(rel.eval_once(a.clone(), a));
}

/// Asserts that the binary relation `rel` is [symmetric].
///
/// For all `a`, `b` of `var.set` it must hold:
/// - `rel(a, b) --> rel(b, a)`
///
/// [symmetric]: https://en.wikipedia.org/wiki/Symmetric_relation
pub fn symmetric<S, R>(var: Var2<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` symmetric?", rel.name);

    let [a, b] = var.eval();

    ops::assert(ops::implies(rel.eval(a.clone(), b.clone()), rel.eval(b, a)));
}

/// Asserts that the binary relation `rel` is [asymmetric].
///
/// For all `a`, `b` of `var.set` it must hold:
/// - `rel(a, b) --> !rel(b, a)`
///
/// [asymmetric]: https://en.wikipedia.org/wiki/Asymmetric_relation
pub fn asymmetric<S, R>(var: Var2<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` asymmetric?", rel.name);

    let [a, b] = var.eval();

    ops::assert(ops::implies(
        rel.eval(a.clone(), b.clone()),
        ops::not(rel.eval(b, a)),
    ));
}

/// Asserts that the binary relation `rel` is [antisymmetric].
///
/// For all `a`, `b` of `var.set` it must hold:
/// - `a != b && rel(a, b) --> !rel(b, a)`
///
/// [antisymmetric]: https://en.wikipedia.org/wiki/Antisymmetric_relation
pub fn antisymmetric<S, R>(var: Var2<S>, rel: Fun2<R>)
where
    S: Debug + Clone + PartialEq,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` antisymmetric?", rel.name);

    let [a, b] = var.eval();

    ops::assert(ops::implies(
        ops::and(
            ops::ne(a.as_ref(), b.as_ref()),
            rel.eval(a.clone(), b.clone()),
        ),
        ops::not(rel.eval(b, a)),
    ));
}

/// Asserts that the binary relation `rel` is [connex].
///
/// For all `a`, `b` of `var.set` it must hold:
/// - `rel(a, b) || rel(b, a)`
///
/// [connex]: https://en.wikipedia.org/wiki/Connex_relation
pub fn connex<S, R>(var: Var2<S>, rel: Fun2<R>)
where
    S: Debug + Clone + PartialEq,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` connex?", rel.name);

    let [a, b] = var.eval();

    ops::assert(ops::or(rel.eval(a.clone(), b.clone()), rel.eval(b, a)));
}

/// Asserts that the binary relation `rel` is [transitive].
///
/// For all `a`, `b`, `c` of `var.set` it must hold:
/// - `rel(a, b) && rel(b, c) --> rel(a, c)`
///
/// [transitive]: https://en.wikipedia.org/wiki/Transitive_relation
pub fn transitive<S, R>(var: Var3<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` transitive?", rel.name);

    let [a, b, c] = var.eval();

    ops::assert(ops::implies(
        ops::and(
            rel.eval(a.clone(), b.clone()),
            rel.eval(b.clone(), c.clone()),
        ),
        rel.eval(a, c),
    ));
}

/// Asserts that the binary relation `rel` is a [partial equivalence relation].
///
/// It must hold:
/// - `rel` is symmetric ([`symmetric`])
/// - `rel` is transitive ([`transitive`])
///
/// [partial equivalence relation]: https://en.wikipedia.org/wiki/Partial_equivalence_relation
pub fn partial_equivalence<S, R>(var: Var3<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` a partial equality relation?", rel.name);

    let [a, b, c] = var.elems;
    let var_2 = var_2(var.set, [a.clone(), b.clone()]);
    let var_3 = var_3(var.set, [a, b, c]);

    symmetric(var_2, rel.as_ref());
    transitive(var_3, rel.as_ref());
}

/// Asserts that the binary relation `rel` is an [equivalence relation].
///
/// It must hold:
/// - `rel` is reflexive ([`reflexive`])
/// - `rel` is symmetric ([`symmetric`])
/// - `rel` is transitive ([`transitive`])
///
/// [equivalence relation]: https://en.wikipedia.org/wiki/Equivalence_relation
pub fn equivalence<S, R>(var: Var3<S>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` an equality relation?", rel.name);

    let [a, b, c] = var.elems;
    let var_1 = var_1(var.set, a.clone());
    let var_2 = var_2(var.set, [a.clone(), b.clone()]);
    let var_3 = var_3(var.set, [a, b, c]);

    reflexive(var_1, rel.as_ref());
    symmetric(var_2, rel.as_ref());
    transitive(var_3, rel.as_ref());
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{infix_fun_2, props, FateVarExt};

    #[test]
    fn reflexive_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_1("u8", "x", dice::u8(..));
            let rel = infix_fun_2("==", |x, y| x == y);
            props::binrel::reflexive(var, rel);
        })
    }

    #[test]
    fn symmetric_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("f32", ["x", "y"], dice::any_f32());
            let rel = infix_fun_2("!=", |x, y| x != y);
            props::binrel::symmetric(var, rel);
        })
    }

    #[test]
    fn asymmetric_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("f32", ["x", "y"], dice::any_f32());
            let rel = infix_fun_2("<", |x, y| x < y);
            props::binrel::asymmetric(var, rel);
        })
    }

    #[test]
    fn antisymmetric_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("u8", ["x", "y"], dice::u8(..));
            let rel = infix_fun_2("<", |x, y| x < y);
            props::binrel::antisymmetric(var, rel);
        })
    }

    #[test]
    fn connex_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_2("u8", ["x", "y"], dice::u8(..));
            let rel = infix_fun_2("<=", |x, y| x <= y);
            props::binrel::connex(var, rel);
        })
    }

    #[test]
    fn transitive_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("char", ["x", "y", "z"], dice::char());
            let rel = infix_fun_2("<", |x, y| x < y);
            props::binrel::transitive(var, rel);
        })
    }

    #[test]
    fn partial_equivalence_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("f32", ["x", "y", "z"], dice::any_f32());
            let rel = infix_fun_2("==", |x, y| x == y);
            props::binrel::partial_equivalence(var, rel);
        })
    }

    #[test]
    fn equivalence_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("String", ["x", "y", "z"], dice::string(dice::char(), ..));
            let rel = infix_fun_2("==", |x, y| x == y);
            props::binrel::equivalence(var, rel);
        })
    }
}
