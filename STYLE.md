# Ataraxia Codebase Style Guide

This document captures the coding conventions, design patterns, and structure of the Ataraxia codebase.

## 1. Workspace Layout & Naming

Ataraxia is a Cargo workspace. The following rules are canonical and must be followed:

### 1.1 Crate directories are top-level

Crate directories live directly at the repository root — **not** nested under a `crates/` (or similar) folder. Each crate is a top-level directory whose name matches its published crate name.

```
ataraxia-proto/        # ataraxia-proto
ataraxia-lexer/        # ataraxia-lexer
ataraxia-parser/       # ataraxia-parser
ataraxia-interpreter/  # ataraxia-interpreter
ataraxia/              # ataraxia (CLI binary)
```

The root `Cargo.toml` lists members explicitly:

```toml
[workspace]
members = ["ataraxia-proto", "ataraxia-lexer", "ataraxia-parser", "ataraxia-interpreter", "ataraxia"]
resolver = "2"
```

### 1.2 Crate names use the `ataraxia-` prefix

Every library crate is named with the `ataraxia-` prefix in kebab-case (e.g. `ataraxia-lexer`, `ataraxia-parser`). The corresponding Rust identifier / import path replaces the hyphen with an underscore (e.g. `ataraxia_lexer`, `ataraxia_parser`).

### 1.3 No ambiguous or shortened names

- The data-model crate is **`ataraxia-proto`** (not `model`, which is ambiguous with "AI model" / LLM). Import path: `ataraxia_proto`.
- The runtime crate is **`ataraxia-interpreter`** (not `interp`). Import path: `ataraxia_interpreter`.

Do not introduce shortened crate names; prefer the full, descriptive name.

## 2. Architecture

```
ataraxia-proto/        # AST, Expression, Operator, Fraction — pure data
ataraxia-lexer/        # Token enum via logos
ataraxia-parser/       # chumsky parser rules & unit tests
ataraxia-interpreter/  # runtime values, Scope, eval, Object implementations
ataraxia/              # CLI entry point binary
```

### Key Architectural Boundaries
- **`ataraxia-proto`**: Pure data definitions. It does NOT depend on interpreter runtime or garbage collection (`gc` object traits).
- **`ataraxia-lexer`**: Token definitions, independent of parsing or evaluation logic.
- **`ataraxia-parser`**: Parses `Token` streams into `Expression` ASTs. Depends on `ataraxia-proto` and `ataraxia-lexer`. Independent of `ataraxia-interpreter`.
- **`ataraxia-interpreter`**: Runtime evaluation. Implements `Object` traits for runtime values (including `Fraction`), manages `Scope`, `Value`, and `Bind`.
- **`ataraxia`**: CLI executable assembling lexer, parser, and interpreter.

## 3. Naming Conventions

- **Crates**: Kebab-case with `ataraxia-` prefix (`ataraxia-proto`, `ataraxia-parser`).
- **Rust Modules & Files**: `snake_case`.
- **Structs, Enums, Traits**: `PascalCase` (`Expression`, `Token`, `Object`, `Scope`).
- **Enum Variants**: `PascalCase` (`Token::Function`, `Expression::Identifier`).
- **Functions & Variables**: `snake_case`.
- **AST Construct Helpers**: Prefix helper constructors on `Expression` with `op_` or `b_` or `l_` (e.g., `op_plus`, `b_true`, `l_while`).

## 4. Code Organization

- **Error Handling**: Prefer custom `Value::Error` or structured error tokens rather than panicking where possible.
- **Imports**: Group imports by std, external dependencies, workspace crates (`ataraxia_proto::...`), and crate-internal modules.
- **Nightly Features**: Pin nightly toolchain in `rust-toolchain.toml`. Explicitly declare nightly crate features in `#![feature(...)]` blocks (e.g., `#![feature(map_try_insert)]`).