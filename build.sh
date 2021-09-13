#!/bin/bash
set -e

echo "STEP diceprop: cargo fmt"
cargo fmt

echo "STEP diceprop: cargo build"
cargo build

echo "STEP diceprop: cargo test -- --format=terse"
cargo test -- --format=terse

echo "STEP diceprop: cargo clippy"
cargo clippy

echo "STEP diceprop: cargo doc --no-deps"
cargo doc --no-deps

echo "STEP diceprop: cargo readme > README.md"
cargo readme > README.md


cd examples_readme

echo "STEP examples_readme: cargo fmt"
cargo fmt

echo "STEP examples_readme: cargo build"
cargo build

echo "STEP examples_readme: cargo test --no-run"
cargo test --no-run

echo "STEP examples_readme: cargo clippy"
cargo clippy
