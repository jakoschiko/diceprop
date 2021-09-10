# diceprop

A collection of mathematical properties for random testing.

It's based on [dicetest](https://github.com/jakoschiko/dicetest).

## Status of this crate

The author does not consider this crate as stable yet. Changes will be documented in the
[changelog](https://github.com/jakoschiko/diceprop/blob/master/CHANGELOG.md).

## Examples

### Associative binary operation

```rust
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
```

The test fails with the following output:

```
The test failed after 12 passes.

# Config
- seed: 14859458141222391139
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: "2pYRCj9fj8sV52fB5iyFhxCISGY3nKlMzlzIKq0NKLwGAAAAAAAAAA=="
- limit: 6
- hints:
        - Is `+` associative?
                - x, y, z of f32 ∩ [-100,100]
                - x = 96.621735
                - y = -90.97134
                - z = -8.10239
                - (x + y) = 5.6503983
                - ((x + y) + z) = -2.451992
                - (y + z) = -99.07373
                - (x + (y + z)) = -2.4519958
                - (((x + y) + z) == (x + (y + z))) = false
- error: assertion failed: (((x + y) + z) == (x + (y + z)))
```

### Left inverse function

```rust
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
```

The test fails with the following output:

```
The test failed after 0 passes.

# Config
- seed: 7632522237817347676
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: "F2/nnlbX6qyCOm5MU7P8BSXdnJ4XNXJdihgwhtWxlzMAAAAAAAAAAA=="
- limit: 0
- hints:
        - Is `√` left inverse of `²`?
                - x of f32 ∩ [0,+∞]
                - x = 305770290000000000000000000000000000000.0
                - (x)² = inf
                - √((x)²) = inf
                - (√((x)²) == x) = false
- error: assertion failed: (√((x)²) == x)
```

### Partial order

```rust
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
```

The test fails with the following output:

```
The test failed after 3 passes.

# Config
- seed: 18374838706510982620
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: "h6jQMNr6fi/j9OZOXmklXYAUATM96EpE6+DENMhSZHkBAAAAAAAAAA=="
- limit: 1
- hints:
        - Is `≤` a partial order?
                - Is `≤` reflexive?
                        - x of f32
                        - x = NaN
                        - (x ≤ x) = false
- error: assertion failed: (x ≤ x)
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
