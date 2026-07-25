# Ataraxia Codebase Style Guide

This document captures the coding conventions, design patterns, and structure of the Ataraxia codebase.

## 1. Workspace & Architecture

Ataraxia is organized as a Cargo workspace with distinct, decoupled crates:

```
crates/
├── model/       # ataraxia-model (AST, Expression, Operator, Fraction - pure data)
├── lexer/       # ataraxia-lexer (Token enum via logos)
├── parser/      # ataraxia-parser (chumsky parser rules & unit tests)
├── interp/      # ataraxia-interp (runtime values, Scope, eval, Object implementations)
└── ataraxia/    # ataraxia (CLI entry point binary)
```

### Key Architectural Boundaries
- **`ataraxia-model`**: Pure data definitions. It does NOT depend on interpreter runtime or garbage collection (`gc` object traits).
- **`ataraxia-lexer`**: Token definitions, independent of parsing or evaluation logic.
- **`ataraxia-parser`**: Parses `Token` streams into `Expression` ASTs. Depends on `ataraxia-model` and `ataraxia-lexer`. Independent of `ataraxia-interp`.
- **`ataraxia-interp`**: Runtime evaluation. Implements `Object` traits for runtime values (including `Fraction`), manages `Scope`, `Value`, and `Bind`.
- **`ataraxia`**: CLI executable assembling lexer, parser, and interpreter.

## 2. Naming Conventions

- **Crates**: Kebab-case (`ataraxia-model`, `ataraxia-parser`).
- **Rust Modules & Files**: `snake_case`.
- **Structs, Enums, Traits**: `PascalCase` (`Expression`, `Token`, `Object`, `Scope`).
- **Enum Variants**: `PascalCase` (`Token::Function`, `Expression::Identifier`).
- **Functions & Variables**: `snake_case`.
- **AST Construct Helpers**: Prefix helper constructors on `Expression` with `op_` or `b_` or `l_` (e.g., `op_plus`, `b_true`, `l_while`).

## 3. Code Organization

- **Error Handling**: Prefer custom `Value::Error` or structured error tokens rather than panicking where possible.
- **Imports**: Group imports by std, external dependencies, workspace crates (`ataraxia_model::...`), and crate-internal modules.
- **Nightly Features**: Pin nightly toolchain in `rust-toolchain.toml`. Explicitly declare nightly crate features in `#![feature(...)]` blocks (e.g., `#![feature(map_try_insert)]`).
