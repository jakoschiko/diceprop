# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Add struct `diceprop::Var`. It is implemented with const generics and can represent an arbitrary number of variables. It replaces `diceprop::{Var1, Var2, Var3}`.
- Add struct `diceprop::Set`. It wraps a value generator and is necessary for generating `diceprop::Var`.
- The functions `diceprop::{fun1, postfix_fun1, fun2, infix_fun2}` are now associated functions of `diceprop::{Fun1, Fun2}`.
- The function `diceprop::elem` is now an associated function of `diceprop::Elem`.
- Remove `diceprop::FateVarExt`. Use the generators `diceprop::Set::{single_var, var}` instead.

### Added
- Add function `diceprop::props::binop::equal`.
- Add struct `diceprop::Fun3`.

[Unreleased]: https://github.com/jakoschiko/diceprop/compare/v0.1.0...HEAD
