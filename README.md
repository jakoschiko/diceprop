[![crates.io](https://img.shields.io/crates/v/diceprop.svg)](https://crates.io/crates/diceprop)
[![Documentation](https://docs.rs/diceprop/badge.svg)](https://docs.rs/diceprop)
[![Test](https://github.com/jakoschiko/diceprop/actions/workflows/test.yml/badge.svg)](https://github.com/jakoschiko/diceprop/actions/workflows/test.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/jakoschiko/diceprop?tab=readme-ov-file#license)

# diceprop

A collection of mathematical properties for random testing.

It's based on [dicetest].

[dicetest]: https://crates.io/crates/dicetest

## Examples

### Associative binary operation

```rust,no_run
use diceprop::{props, Fun2, Set};
use dicetest::prelude::*;

#[test]
fn add_is_associative_for_small_f32() {
    Dicetest::repeatedly().run(|mut fate| {
        let set = Set::new("f32 ∩ [-100,100]", dice::f32(-100.0..=100.0));
        let vars = fate.roll(set.vars(["x", "y", "z"]));
        let add = Fun2::infix("+", |x, y| x + y);
        props::binop::associative(vars, add);
    })
}
```

The test fails with the following output:

```text
The test failed after 8 passes.

# Config
- seed: 3953300129614487606
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: 2xMt8jfykZD8kKocVrFZR84lYvMbMs04Rl0ZcpHvDq5WA05p7iI9U8
- limit: 4
- hints:
        - Is `+` associative?
                - x, y, z of f32 ∩ [-100,100]
                - x = -77.55548
                - y = 96.37662
                - z = -25.76199
                - (x + y) = 18.821136
                - ((x + y) + z) = -6.940853
                - (y + z) = 70.614624
                - (x + (y + z)) = -6.940857
                - (((x + y) + z) == (x + (y + z))) = false
- error: assertion failed: (((x + y) + z) == (x + (y + z)))
```

### Left inverse function

```rust,no_run
use diceprop::{props, Fun1, Set};
use dicetest::prelude::*;

#[test]
fn sqrt_is_left_inverse_of_sq_for_non_negative_f32() {
    Dicetest::repeatedly().run(|mut fate| {
        let set = Set::new("f32 ∩ [0,+∞]", dice::f32(0.0..));
        let vars = fate.roll(set.vars(["x"]));
        let sq = Fun1::postfix("²", |x| x * x);
        let sqrt = Fun1::new("√", |x: f32| x.sqrt());
        props::fun::left_inverse(vars, sq, sqrt);
    })
}
```

The test fails with the following output:

```text
The test failed after 0 passes.

# Config
- seed: 11456840474003924454
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: 64H4o8H1xczhbtq1FWgThGEkosuz8WYzyZ5nYe7sSie9qsgkfbbtRI
- limit: 0
- hints:
        - Is `√` left inverse of `²`?
                - x of f32 ∩ [0,+∞]
                - x = 3.079607e38
                - (x)² = inf
                - √((x)²) = inf
                - (√((x)²) == x) = false
- error: assertion failed: (√((x)²) == x)
```

### Partial order

```rust,no_run
use diceprop::{props, Fun2, Set};
use dicetest::prelude::*;

#[test]
fn gt_is_partial_order_for_any_f32() {
    Dicetest::repeatedly().run(|mut fate| {
        let set = Set::new("f32", dice::any_f32());
        let vars = fate.roll(set.vars(["x", "y", "z"]));
        let gt = Fun2::infix("≤", |x, y| x <= y);
        props::binrel::partial_order(vars, gt);
    })
}
```

The test fails with the following output:

```text
The test failed after 4 passes.

# Config
- seed: 14352745908157362390
- start limit: 0
- end limit: 100
- passes: 200

# Counterexample
- run code: EiazScJsgqxJ5a69wRiYCiam1wEa8vnOAlZNwGI8f07oeiXlMlDZIG
- limit: 2
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
