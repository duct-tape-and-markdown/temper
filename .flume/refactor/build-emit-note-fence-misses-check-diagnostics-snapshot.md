## Surface

`EMIT-PROJECTION-NOTE-DEFERRED-TO-INSTALL`'s fence is one path short of the change
it asks for. The entry lists `tests/snapshots/gauntlet__projection_tree.snap` but not
its sibling:

- `tests/snapshots/gauntlet__check_diagnostics.snap:6` — the gauntlet's `check`
  narration, which pins the `install.gate-installed` warning verbatim:
  `(missing or drifted: guard hook, post-tool-use hook, managed-by note:
  <HARNESS>/.claude/rules/authoring.md, managed-by note:
  <HARNESS>/.claude/rules/conventions.md)`.

The two `managed-by note:` clauses are exactly what the entry's acceptance removes —
"check's `gate_installed` reports no finding for it" — so this snapshot moves with the
entry, not after it. It is the acceptance rendered, not collateral. `tests/gauntlet.rs`
is in the fence, but the assertion lives in the `.snap` file, so no in-fence edit reaches
it; the whole change is otherwise complete and `cargo test --no-fail-fast` is green with
it accepted.

## Observed at

b9e5f715 (HEAD when observed; `INSTA_UPDATE=always cargo test --test gauntlet` accepts
both snapshots, and the accepted diff on each is the note line and its consequences and
nothing else).

## Suggested consolidation

Re-scope the entry with `tests/snapshots/gauntlet__check_diagnostics.snap` added to
`files.edit`, described as: the gate warning loses its two `managed-by note:` clauses
because emit now places the note the gate used to find missing. Two `.snap` files, one
`insta` accept — both belong to `tests/gauntlet.rs`, so a fence naming that test file
should carry every snapshot it writes.

The rest of the entry executed exactly as scoped and needs no re-thinking:
`renders_frontmatter(format, fields)` in `src/drift.rs` as the one predicate
`project_bytes` renders by and `emit_one` picks the marker form by; `NOTE_COMMENT`
relocated to `src/placement.rs` beside `BANNER`; `install::project_note` narrowed to
`converge_note_wording` (the `converge_banner_wording` contract, one form over); the four
re-cut assertions in `tests/install.rs`, `tests/emit.rs` and `tests/json_document_format.rs`.
