# Toy Compiler in Rust

A small compiler built from scratch in Rust, with no parser generators or other external crates, to learn compiler engineering one stage at a time.

**Status: in progress.** The lexer for an integer arithmetic language works and has unit tests. The parser, semantic analysis and evaluation are planned and not written yet. The checklist below tracks every step.

Rust was chosen for explicit memory layout, algebraic data types (`enum`) and exhaustive pattern matching (`match`).

[A course mapping from my school time](https://notebook.google.com/notebook/b7362179-7b82-44e8-8f9b-98052c9afc48/artifact/6c0c25f7-5c4a-4335-a94c-51f13bd9f572)

## What works today

- Tokens: integers (`i64`), `+ - * /`, parentheses and end of file, defined in [src/token.rs](src/token.rs).
- A hand-written lexer that records the line and column of every token, in [src/lexer.rs](src/lexer.rs).
- Errors as values: unexpected characters and integers too large for `i64` return a `CompileError`, rendered with the source line and a caret.
- A short language spec with valid and invalid inputs, in [docs/spec.md](docs/spec.md).

## Tech stack

Rust (2024 edition), Cargo, no dependencies.

## How to run

```bash
git clone https://github.com/blip-cmd/toy-compiler-rust.git
cd toy-compiler-rust
cargo test   # lexer unit tests
cargo run    # prints sample tokens
```

## What I would improve next

- Connect the lexer to the command line with `--dump-tokens`.
- Add golden tests for the lexer and run `cargo test` in CI.
- Write the grammar, then the AST, a precedence-aware parser and a tree-walking evaluator.

## Planned architecture

```text
Source Code
    │
    ▼ (Lexer)
Token Stream
    │
    ▼ (Parser)
Abstract Syntax Tree (AST)
    │
    ▼ (Semantic Analyzer)
Decorated AST / Type Checked
    │
    ▼ (Evaluator / IR Lowering)
Execution / Bytecode / Machine Code
```

## Progress

### Level 0: Setup and plan
- [x] 0.1 Diagnostic
- [x] 0.2 Pipeline picture
- [x] 0.3 Blank folder to first commit
- [x] 0.4 Push to GitHub
- [x] 0.5 README plan
- [x] 0.6 Level 1 language spec
- [x] 0.7 Library and main

### Level 1: Arithmetic calculator
- [x] 1.1 Tokens in theory
- [x] 1.2 First file, first lines (`src/token.rs`)
- [x] 1.3 Finish the token kinds
- [x] 1.4 Token with position
- [x] 1.5 Lexer skeleton
- [x] 1.6 Single-character tokens
- [x] 1.7 Integers
- [x] 1.8 Lexer errors
- [ ] 1.9 Dump tokens (`--dump-tokens`)
- [ ] 1.10 Lexer golden tests
- [ ] 1.11 Checkpoint A
- [ ] 1.12 Grammar in theory
- [ ] 1.13 Write the EBNF (`docs/grammar.ebnf`)
- [ ] 1.14 The AST (`src/ast.rs`)
- [ ] 1.15 Parser skeleton
- [ ] 1.16 Primary expressions
- [ ] 1.17 Precedence and associativity
- [ ] 1.18 Parse errors
- [ ] 1.19 Dump AST (`--dump-ast`)
- [ ] 1.20 Tree-walking evaluator
- [ ] 1.21 End-to-end tests and CI
- [ ] 1.22 Checkpoint B and tag `v0.1`