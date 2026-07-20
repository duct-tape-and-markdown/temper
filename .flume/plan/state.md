# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 79e0079 — unchanged, no new src/tests/sdk commits since.
- Residue swept through: 79e0079 — unchanged, same reason.
- Posture swept through: src/builtin.rs..src/json_splice.rs (prior rotation), plus src/kind.rs, src/layout.rs, src/lib.rs, src/main.rs, src/placement.rs, src/read.rs, src/roster.rs, src/schema.rs, src/tap.rs, src/telemetry.rs, src/test_support.rs covered — mid-rotation. Rotation continues to src/toml_document.rs next (the last module in src/'s list — a clean sweep there empties the frontier).
- This tick: POSTURE SWEEP, src/test_support.rs neighborhood (imports: src/kind.rs's `CustomKind`/`Extraction`/`Format`/`Governs`/`Primitive`/`Unit`/`UnitShape`, src/frontmatter.rs's `Member`). Clean: cohesive `#[cfg(test)]`-gated scaffolding (module itself gated at src/lib.rs:62-63, never enters a non-test build), no embedded provider knowledge (`skill_kind`/`rule_kind` synthesize schema-shaped kinds deliberately decoupled from `builtin_kind::definition`, per their own doc comments), no dead plumbing. Three stale `#[allow(dead_code)]` markers found (`skill_kind`/`rule_kind`/`surface_unit`, lines 22/51/70) — all have live `#[cfg(test)]` consumers today (frontmatter.rs, builtin_kind.rs); verified empirically that removing all three still passes `cargo clippy --all-targets -- -D warnings`. No engineering.md/architecture.md section covers stale lint-suppression attributes, so not filed — noted for whichever entry next opens the file.
- Queue: 2 pending, 0 open, 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected by this tick, HEAD unchanged since their last re-check). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: yes — no pickable entries exist (both remaining are parked), so the posture sweep is the next live job: rotation continues to src/toml_document.rs.
