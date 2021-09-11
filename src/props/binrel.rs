//! Properties for [binary relations].
//!
//! [binary relations]: https://en.wikipedia.org/wiki/Binary_relation

use dicetest::hint_section;
use std::fmt::Debug;

use crate::{ops, Fun2, Var};

/// Asserts that the binary relation `rel` is [reflexive].
///
/// For all `a` of `var.set` it must hold:
/// - `rel(a, a)`
///
/// [reflexive]: https://en.wikipedia.org/wiki/Reflexive_relation
pub fn reflexive<S, R>(var: Var<S, 1>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: FnOnce(S, S) -> bool,
{
    hint_section!("Is `{}` reflexive?", rel.name);

    let [a] = var.eval();

    ops::assert(rel.eval_once(a.clone(), a));
}

/// Asserts that the binary relation `rel` is [symmetric].
///
/// For all `a`, `b` of `var.set` it must hold:
/// - `rel(a, b) --> rel(b, a)`
///
/// [symmetric]: https://en.wikipedia.org/wiki/Symmetric_relation
pub fn symmetric<S, R>(var: Var<S, 2>, rel: Fun2<R>)
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
pub fn asymmetric<S, R>(var: Var<S, 2>, rel: Fun2<R>)
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
pub fn antisymmetric<S, R>(var: Var<S, 2>, rel: Fun2<R>)
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
pub fn connex<S, R>(var: Var<S, 2>, rel: Fun2<R>)
where
    S: Debug + Clone,
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
pub fn transitive<S, R>(var: Var<S, 3>, rel: Fun2<R>)
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
pub fn partial_equivalence<S, R>(var: Var<S, 3>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` a partial equality relation?", rel.name);

    let [a, b, c] = var.elems;
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    symmetric(var_2, rel.as_ref());
    transitive(var_3, rel);
}

/// Asserts that the binary relation `rel` is an [equivalence relation].
///
/// It must hold:
/// - `rel` is reflexive ([`reflexive`])
/// - `rel` is symmetric ([`symmetric`])
/// - `rel` is transitive ([`transitive`])
///
/// [equivalence relation]: https://en.wikipedia.org/wiki/Equivalence_relation
pub fn equivalence<S, R>(var: Var<S, 3>, rel: Fun2<R>)
where
    S: Debug + Clone,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` an equality relation?", rel.name);

    let [a, b, c] = var.elems;
    let var_1 = Var::new(var.set, [a.clone()]);
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    reflexive(var_1, rel.as_ref());
    symmetric(var_2, rel.as_ref());
    transitive(var_3, rel);
}

/// Asserts that the binary relation `rel` is a [partial order].
///
/// It must hold:
/// - `rel` is reflexive ([`connex`])
/// - `rel` is antisymmetric ([`antisymmetric`])
/// - `rel` is transitive ([`transitive`])
///
/// [partial order]: https://en.wikipedia.org/wiki/Partially_ordered_set
pub fn partial_order<S, R>(var: Var<S, 3>, rel: Fun2<R>)
where
    S: Debug + Clone + PartialEq,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` a partial order?", rel.name);

    let [a, b, c] = var.elems;
    let var_1 = Var::new(var.set, [a.clone()]);
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    reflexive(var_1, rel.as_ref());
    antisymmetric(var_2, rel.as_ref());
    transitive(var_3, rel);
}

/// Asserts that the binary relation `rel` is a [total order].
///
/// It must hold:
/// - `rel` is connex ([`connex`])
/// - `rel` is antisymmetric ([`antisymmetric`])
/// - `rel` is transitive ([`transitive`])
///
/// [total order]: https://en.wikipedia.org/wiki/Total_order
pub fn total_order<S, R>(var: Var<S, 3>, rel: Fun2<R>)
where
    S: Debug + Clone + PartialEq,
    R: Fn(S, S) -> bool,
{
    hint_section!("Is `{}` a total order?", rel.name);

    let [a, b, c] = var.elems;
    let var_2 = Var::new(var.set, [a.clone(), b.clone()]);
    let var_3 = Var::new(var.set, [a, b, c]);

    connex(var_2.clone(), rel.as_ref());
    antisymmetric(var_2, rel.as_ref());
    transitive(var_3, rel);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{props, Fun2, Set};

    #[test]
    fn reflexive_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u8", dice::u8(..));
            let var = fate.roll(set.var(["x"]));
            let rel = Fun2::infix("==", |x, y| x == y);
            props::binrel::reflexive(var, rel);
        })
    }

    #[test]
    fn symmetric_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("f32", dice::any_f32());
            let var = fate.roll(set.var(["x", "y"]));
            let rel = Fun2::infix("!=", |x, y| x != y);
            props::binrel::symmetric(var, rel);
        })
    }

    #[test]
    fn asymmetric_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("f32", dice::any_f32());
            let var = fate.roll(set.var(["x", "y"]));
            let rel = Fun2::infix("<", |x, y| x < y);
            props::binrel::asymmetric(var, rel);
        })
    }

    #[test]
    fn antisymmetric_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u8", dice::u8(..));
            let var = fate.roll(set.var(["x", "y"]));
            let rel = Fun2::infix("<", |x, y| x < y);
            props::binrel::antisymmetric(var, rel);
        })
    }

    #[test]
    fn connex_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u8", dice::u8(..));
            let var = fate.roll(set.var(["x", "y"]));
            let rel = Fun2::infix("<=", |x, y| x <= y);
            props::binrel::connex(var, rel);
        })
    }

    #[test]
    fn transitive_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("char", dice::char());
            let var = fate.roll(set.var(["x", "y", "z"]));
            let rel = Fun2::infix("<", |x, y| x < y);
            props::binrel::transitive(var, rel);
        })
    }

    #[test]
    fn partial_equivalence_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("f32", dice::any_f32());
            let var = fate.roll(set.var(["x", "y", "z"]));
            let rel = Fun2::infix("==", |x, y| x == y);
            props::binrel::partial_equivalence(var, rel);
        })
    }

    #[test]
    fn equivalence_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("String", dice::string(dice::char(), ..));
            let var = fate.roll(set.var(["x", "y", "z"]));
            let rel = Fun2::infix("==", |x, y| x == y);
            props::binrel::equivalence(var, rel);
        })
    }

    #[test]
    fn partial_order_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u8²", dice::zip().two(dice::u8(..), dice::u8(..)));
            let var = fate.roll(set.var(["x", "y", "z"]));
            let rel = Fun2::infix("<=", |x: (u8, u8), y: (u8, u8)| {
                (x.0 <= y.0) && (x.1 <= y.1)
            });
            props::binrel::partial_order(var, rel);
        })
    }

    #[test]
    fn total_order_example() {
        Dicetest::once().run(|mut fate| {
            let set = Set::new("u8", dice::u8(..));
            let var = fate.roll(set.var(["x", "y", "z"]));
            let rel = Fun2::infix("<=", |x, y| x <= y);
            props::binrel::total_order(var, rel);
        })
    }
}
