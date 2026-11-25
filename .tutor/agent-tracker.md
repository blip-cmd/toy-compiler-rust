# Tutor tracker (agent-owned)

Mode: tutor
Current: Level 1, step 1.12
Last session: 2026-11-25, Checkpoint A (Step 1.11) passed! Lexer stage complete. Ready for Step 1.12 (Grammar in theory).

## Diagnostic results
| Topic | Rating (none, shaky, solid) | Evidence |
|---|---|---|
| Compiler pipeline | solid | Repo already contains the planned library/main split and README progress list |
| Rust basics | solid | `src/lib.rs` and `src/main.rs` compile; comfortable with enums, structs, impl, Option, Result, pattern matching, unwrap vs unwrap_err |
| Lexer stage (tokens, scanning, errors) | solid | Completed TokenKind (1.3), Token struct with coords (1.4), advance/peek navigation (1.5), single-character tokens (1.6), integers with overflow handling (1.7), error caret formatting (1.8) |

## Misconceptions log
| Date | Step | Misconception | Correction given | Re-check passed |
|---|---|---|---|---|
| 2026-11-11 | 1.4 | Position tracking can wait | Lexer is sole stage reading raw characters, coordinates must be captured at scan time | yes |
| 2026-11-17 | 1.8 | Generic unwrap for Ok and Err | Result<T,E> has two distinct types; compile-time safety requires separate unwrapping or matching | yes |

## Question log (one line each)
| Date | Step | Question type | Result (right, partly, wrong) |
|---|---|---|---|
| 2026-11-11 | 1.2 | Recall: derive Debug, enums vs strings, pub mod | partly |
| 2026-11-11 | 1.3 | Recall: Int payload vs operators, EOF utility | right |
| 2026-11-11 | 1.4 | Design: Token position timing and coordinate prediction | partly |
| 2026-11-11 | 1.5 | Design: advance vs peek, SSOT for position | right |
| 2026-11-11 | 1.6 | Design: Lexer whitespace skipping and start column semantics | right |
| 2026-11-17 | 1.7 | Explain: Maximal munch and integer overflow handling | right |
| 2026-11-17 | 1.8 | Explain: Result vs panic, caret spacing calculation | right |
| 2026-11-24 | 1.9 | Explain: CLI vs library split, pipeline observability, argv[0] | right |
| 2026-11-25 | 1.10 | Explain: golden tests, invalid inputs, integration test visibility | partly |
| 2026-11-25 | 1.11 | Checkpoint A: tokens vs lexemes, maximal munch, positions, Result errors, explain-back | right |

## Decisions confirmed by the owner
- Language: Rust
- Repo layout: standard Cargo project

## Next session plan
1. Step 1.12: Grammar in theory (context-free grammars, precedence, associativity).
2. Step 1.13: Write the EBNF (`docs/grammar.ebnf`).
