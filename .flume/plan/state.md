# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: befa77f — unchanged; befa77f..HEAD touches only sdk/package.json + package-lock.json (version bumps), no src/tests/sdk-src.
- Residue swept through: befa77f — unchanged, same reason.
- Posture swept through: 00b880d — unchanged, still re-armed (no new build commits since).
- This tick: INBOX. Routed the sole note (centercode 0.0.9 field report: enablement wire-label reader/writer split) into one open pending entry, REGISTRATION-ENABLEMENT-LABEL-FIELD-CARRYING. Re-verified every claim on disk at HEAD rather than trusting the report: `sdk/src/declarations.ts:206`'s `registrationLabel` still returns bare `"enablement"` though `Registration::Enablement` (30c52e1) carries a field; `src/builtin_lock.toml:90` still embeds the bare form; `src/kind.rs:1024`'s `registration_from_label` has no bare-`"enablement"` arm (only `always`/`user-invoked`/`connection`/`registry` match bare) so it falls through to the `(field)` parse and returns `None`, surfacing as `LockRowError::Vocabulary` via `kind_vocab` (kind.rs:565) inside `CustomKind::from_kind_fact_row` — confirmed the exact failure mode. `tests/builtin_lock_frozen.rs` only byte-compares the SDK's derived lock against the embedded snapshot (writer vs writer), so the one-sided respell stays green; two Rust tests (`installed_plugin_kind.rs:260`, `lock_declaration_rows.rs:2185`) additionally pin the bare form as correct and need updating in the same commit. Entry adds the missing reader round-trip to `builtin_lock_frozen.rs` per the report's third fix. Inbox drained.
- Queue: 3 pending, 1 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: after-build — REGISTRATION-ENABLEMENT-LABEL-FIELD-CARRYING is pickable now; the only other live job is the posture sweep (re-armed, window 00b880d..HEAD touching main.rs/read.rs/drift.rs), which resumes once the wave hands back.
