#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod associative_example {
    use diceprop::{props, FateVarExt, Fun2, Set};
    use dicetest::prelude::*;

    #[test]
    fn add_is_associative_for_small_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let set = Set::new("f32 ∩ [-100,100]", dice::f32(-100.0..=100.0));
            let var = fate.roll_var(["x", "y", "z"], set);
            let add = Fun2::infix("+", |x, y| x + y);
            props::binop::associative(var, add);
        })
    }
}

#[cfg(test)]
mod left_inverse_example {
    use diceprop::{props, FateVarExt, Fun1, Set};
    use dicetest::prelude::*;

    #[test]
    fn sqrt_is_left_inverse_of_sq_for_non_negative_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let set = Set::new("f32 ∩ [0,+∞]", dice::f32(0.0..));
            let var = fate.roll_var(["x"], set);
            let sq = Fun1::postfix("²", |x| x * x);
            let sqrt = Fun1::new("√", |x: f32| x.sqrt());
            props::fun::left_inverse(var, sq, sqrt);
        })
    }
}

#[cfg(test)]
mod partial_order_example {
    use diceprop::{props, FateVarExt, Fun2, Set};
    use dicetest::prelude::*;

    #[test]
    fn gt_is_partial_order_for_any_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let set = Set::new("f32", dice::any_f32());
            let var = fate.roll_var(["x", "y", "z"], set);
            let gt = Fun2::infix("≤", |x, y| x <= y);
            props::binrel::partial_order(var, gt);
        })
    }
}
