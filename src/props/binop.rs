use std::fmt::Debug;

use crate::props::commutative_fun;
use crate::{hint_section, ops, Fun2, Var2, Var3};

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
pub fn associative_binop<S, O>(set: Var3<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `{}` associative?", op.name);

    let [a, b, c] = set.eval();

    ops::assert(ops::eq(
        op.eval(op.eval(a.clone(), b.clone()), c.clone()).as_ref(),
        op.eval(a, op.eval(b, c)).as_ref(),
    ));
}

#[cfg(test)]
mod tests {
    use crate::{fun_2, props, FateVarExt};

    use dicetest::prelude::*;
    use std::collections::BTreeSet;

    #[test]
    fn commutative_binop_example() {
        Dicetest::once().run(|mut fate| {
            let rel = fun_2("intersection", |x, y| {
                BTreeSet::<u8>::intersection(&x, &y)
                    .cloned()
                    .collect::<BTreeSet<_>>()
            });
            let var = fate.roll_var_2(
                "BTreeSet<u8>",
                ["x", "y"],
                dice::b_tree_set(dice::u8(..), ..),
            );
            props::commutative_binop(var, rel);
        })
    }

    #[test]
    fn associative_binop_example() {
        Dicetest::once().run(|mut fate| {
            let rel = fun_2("append", |mut x, mut y| {
                Vec::<u8>::append(&mut x, &mut y);
                x
            });
            let var = fate.roll_var_3("Vec<u8>", ["x", "y", "z"], dice::vec(dice::u8(..), ..));
            props::associative_binop(var, rel);
        })
    }
}
