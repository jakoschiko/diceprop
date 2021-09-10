#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod associative_example {
    use diceprop::{infix_fun_2, props, FateVarExt};
    use dicetest::prelude::*;

    #[test]
    fn add_is_associative_for_small_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let small_f32_die = dice::f32(-100.0..=100.0);
            let var = fate.roll_var("f32 ∩ [-100,100]", ["x", "y", "z"], small_f32_die);
            let add = infix_fun_2("+", |x, y| x + y);
            props::binop::associative(var, add);
        })
    }
}

#[cfg(test)]
mod left_inverse_example {
    use diceprop::{fun_1, postfix_fun_1, props, FateVarExt};
    use dicetest::prelude::*;

    #[test]
    fn sqrt_is_left_inverse_of_sq_for_non_negative_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let non_negative_f32_die = dice::f32(0.0..);
            let var = fate.roll_var("f32 ∩ [0,+∞]", ["x"], non_negative_f32_die);
            let sq = postfix_fun_1("²", |x| x * x);
            let sqrt = fun_1("√", |x: f32| x.sqrt());
            props::fun::left_inverse(var, sq, sqrt);
        })
    }
}

#[cfg(test)]
mod partial_order_example {
    use diceprop::{infix_fun_2, props, FateVarExt};
    use dicetest::prelude::*;

    #[test]
    fn gt_is_partial_order_for_any_f32() {
        Dicetest::repeatedly().run(|mut fate| {
            let any_f32_die = dice::any_f32();
            let var = fate.roll_var("f32", ["x", "y", "z"], any_f32_die);
            let gt = infix_fun_2("≤", |x, y| x <= y);
            props::binrel::partial_order(var, gt);
        })
    }
}
