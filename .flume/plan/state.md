# Plan state

- Spec derived through: 8911c38 — "A seam gate reads what the real writer wrote" (specs/process/engineering.md) fully routed: surveyed every `tests/*.rs` file matching `agree|byte.pin|round.trip|frozen` for the anti-pattern (writer-vs-writer self-agreement, or a real reader driven over hand-authored counterpart rows, on an *agreement* claim). Two live findings filed — MENTION-REACHABLE-GATE-SDK-AGREEMENT, CUSTOM-KIND-CONTENT-SDK-AGREEMENT — both `tests/lock_declaration_rows.rs` comments overclaiming SDK-emit coverage three compiler-tier tests never exercise, verified against `src/builtin_lock.toml`, `src/contract.rs`, `src/kind.rs`, `sdk/src/kind.ts` on disk. `tests/tap.rs`'s legacy-version test checked and ruled exempt (refusal-test analogy: today's writer cannot produce a stale-schema row). Everything else checked compliant or non-applicable (full list in the plan commit body).
- Audited through: 1fe148c — unchanged, not this tick's job (no commits past it touch `src/`, `sdk/src/`, or `tests/`).
- Residue swept through: 1fe148c — unchanged, not this tick's job.
- Posture swept through: 6f74784 — re-armed: 8911c38 touched `specs/process/engineering.md` past this cursor (its own forward window is otherwise empty of `src/`/`sdk/src/`/`tests/` commits), so the rotation resumes next tick.
- This tick: SPEC DELTA — 8911c38 routed in full, 2 entries filed (1 open, 1 blockedBy).
- Queue: 4 pending, 1 open, 1 blockedBy, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unchanged, not this tick's job). Refactor: 0 live. Friction: 0 live. Amendments: 0 live (0044's first amendment ratified at 00c3a22, record deleted). Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the posture sweep (re-armed by 8911c38 past the 6f74784 cursor), and MENTION-REACHABLE-GATE-SDK-AGREEMENT is open and pickable this tick: ready work ships first, the sweep resumes once that wave hands back.
