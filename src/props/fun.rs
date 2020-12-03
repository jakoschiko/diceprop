use std::fmt::Debug;

use crate::{hint_section, ops, Fun2, Var2};

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
    use crate::{fun_2, props, FateVarExt};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;

    #[test]
    fn commutative_fun_example() {
        Dicetest::once().run(|mut fate| {
            let rel = fun_2("is_disjoin", |x, y| BTreeSet::<u8>::is_disjoint(&x, &y));
            let var = fate.roll_var_2(
                "BTreeSet<u8>",
                ["x", "y"],
                dice::b_tree_set(dice::u8(..), ..),
            );
            props::commutative_fun(var, rel);
        })
    }
}
