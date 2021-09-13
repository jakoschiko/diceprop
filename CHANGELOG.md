# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2021-09-13

### Added
- Add struct `diceprop::Vars`. It is implemented with const generics and can represent an arbitrary number of variables. It replaces `diceprop::{Var1, Var2, Var3}`.
- Add struct `diceprop::Set`. It wraps a value generator and represents a mathematical set. It can be used to create generators for `diceprop::Vars` via `diceprop::Set::{var_once, var}`.
- Add associated functions `diceprop::Fun1::{new, postfix}` and `diceprop::Fun2::{new, infix}`.
- Add struct `diceprop::Fun3`.
- Add associated function `diceprop::Elem::new`.
- Add function `diceprop::props::binop::equal`.

## Removed 
- Remove structs `diceprop::{Var1, Var2, Var3}`. Use `diceprop::Vars` instead.
- Remove functions `diceprop::{var_1, var_2, var_3}`. Use `diceprop::Vars::new` instead.
- Remove trait `diceprop::FateVarExt`. Instead you can create generators with `diceprop::Set::{var_once, vars}` and pass them to `dicetest::Fate::roll`.
- Remove functions `diceprop::{fun1, postfix_fun1, fun2, infix_fun2}`. Use the associated functions of `diceprop::{Fun1, Fun2}` instead.
- Remove function `diceprop::elem`. Use `diceprop::Elem::new` instead.

### Changed
- Update dependency `dicetest` from `0.2.1` to `0.3`.
- All properties in `diceprop::props` take `diceprop::Vars` instead of `diceprop::{Var1, Var2, Var3}`.

[Unreleased]: https://github.com/jakoschiko/diceprop/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/jakoschiko/diceprop/compare/v0.1.0...v0.2.0
