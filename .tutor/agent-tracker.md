# Tutor tracker (agent-owned)

Mode: tutor
Current: Level 1, step 1.7
Last session: 2026-11-11, Steps 1.2 to 1.6 completed and verified with 5 passing tests

## Diagnostic results
| Topic | Rating (none, shaky, solid) | Evidence |
|---|---|---|
| Compiler pipeline | solid | Repo already contains the planned library/main split and README progress list |
| Rust basics | solid | `src/lib.rs` and `src/main.rs` compile; comfortable with enums, structs, impl, Option, Result |
| Lexer stage (tokens & position) | solid | Completed TokenKind (1.3), Token struct with coords (1.4), advance/peek navigation (1.5), single-character tokens and whitespace skipping (1.6) |

## Misconceptions log
| Date | Step | Misconception | Correction given | Re-check passed |
|---|---|---|---|---|
| 2026-11-11 | 1.4 | Position tracking can wait | Lexer is sole stage reading raw characters, coordinates must be captured at scan time | yes |

## Question log (one line each)
| Date | Step | Question type | Result (right, partly, wrong) |
|---|---|---|---|
| 2026-11-11 | 1.2 | Recall: derive Debug, enums vs strings, pub mod | partly |
| 2026-11-11 | 1.3 | Recall: Int payload vs operators, EOF utility | right |
| 2026-11-11 | 1.4 | Design: Token position timing and coordinate prediction | partly |
| 2026-11-11 | 1.5 | Design: advance vs peek, SSOT for position | right |
| 2026-11-11 | 1.6 | Design: Lexer whitespace skipping and start column semantics | right |

## Decisions confirmed by the owner
- Language: Rust
- Repo layout: standard Cargo project

## Next session plan
1. Step 1.7: Multi-digit integer lexing (maximal munch, overflow handling with `i64`).
2. Step 1.8: Lexer errors and caret diagnostics.
