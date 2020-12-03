use std::fmt::Debug;

use crate::props::commutative_fun;
use crate::{Fun2, Var2};

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
}
