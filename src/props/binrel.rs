use std::fmt::Debug;

use crate::{hint_section, ops, Fun2, Var1, Var2};

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
            let set = fate.roll_var_2("f32", ["x", "y"], dice::f32(..));
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
}
