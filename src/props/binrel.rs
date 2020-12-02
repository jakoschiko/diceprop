use std::fmt::Debug;

use crate::{hint_section, ops, Fun2, Var1};

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

#[cfg(test)]
mod tests {
    use crate::{infix_fun_2, props, FateVarExt};
    use dicetest::prelude::*;

    #[test]
    fn reflexive_binrel_example() {
        Dicetest::repeatedly().run(|mut fate| {
            let rel = infix_fun_2("==", |x, y| x == y);
            let set = fate.roll_var_1("u8", "x", dice::u8(..));
            props::reflexive_binrel(set, rel);
        })
    }
}
