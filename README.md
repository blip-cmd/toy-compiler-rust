# Toy Compiler in Rust

A hands-on, educational compiler built from scratch in Rust, translating high-level expressions into lowered execution targets without external parsing frameworks.

We chose Rust to learn compiler engineering with explicit memory layout, algebraic data types (`enum`), and exhaustive pattern matching (`match`), balancing low-level control with safety.

[A course mapping from my school time](https://notebook.google.com/notebook/b7362179-7b82-44e8-8f9b-98052c9afc48/artifact/6c0c25f7-5c4a-4335-a94c-51f13bd9f572)

## Architecture Pipeline

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
- [] 1.9 Dump tokens (`--dump-tokens`)
- [] 1.10 Lexer golden tests
- [] 1.11 Checkpoint A
- [] 1.12 Grammar in theory
- [] 1.13 Write the EBNF (`docs/grammar.ebnf`)
- [] 1.14 The AST (`src/ast.rs`)
- [] 1.15 Parser skeleton
- [] 1.16 Primary expressions
- [] 1.17 Precedence and associativity
- [] 1.18 Parse errors
- [] 1.19 Dump AST (`--dump-ast`)
- [] 1.20 Tree-walking evaluator
- [] 1.21 End-to-end tests and CI
- [] 1.22 Checkpoint B and tag `v0.1`