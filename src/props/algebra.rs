use std::fmt::Debug;

use crate::props::associative_binop;
use crate::{hint_section, Fun2, Var3};

/// Asserts that `(var.set, op)` is a semigroup.
///
/// It must hold:
/// - `op` is associative  ([`associative_binop`])
pub fn semigroup<S, O>(var: Var3<S>, op: Fun2<O>)
where
    S: Debug + Clone + PartialEq,
    O: Fn(S, S) -> S,
{
    hint_section!("Is `({}, {})` a semigroup?", var.set, op.name);

    associative_binop(var, op);
}

#[cfg(test)]
mod tests {
    use dicetest::prelude::*;

    use crate::{fun_2, props, FateVarExt};

    #[test]
    fn semigroup_example() {
        Dicetest::once().run(|mut fate| {
            let var = fate.roll_var_3("u64", ["x", "y", "z"], dice::u64(..=1000));
            let op = fun_2("+", |x, y| x + y);
            props::semigroup(var, op);
        })
    }
}
