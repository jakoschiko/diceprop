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
