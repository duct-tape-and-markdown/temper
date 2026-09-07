## Surface

`EMIT-PROJECTION-NOTE-DEFERRED-TO-INSTALL` cannot reach green inside its fence.
The change works (implemented and verified locally: `cargo test --test emit`
46/46 green with the new regression test), but four existing tests in three
files the entry did not list assert the *old* ownership split — that `emit`
does not stamp the note and that `install` originates it. They are the mirror
of the re-cuts `bccf42c0` already made for the banner, and every one of them
must invert with this entry, not after it.

Fence needed: the entry's three paths **plus**

- `tests/install.rs:1757` `emit_never_stamps_the_managed_by_note` — asserts the
  exact negation of the entry's acceptance (no `# temper: managed projection`
  in an emitted `SKILL.md`/`rust.md`). Re-cut to assert emit *does* stamp the
  note and still never stamps the modeline (which stays install's). Breaks on
  the `src/drift.rs` half alone, so no shippable subset exists.
- `tests/install.rs:1097` `gate_installed_names_drifted_un_noted_files` —
  deletes the note line from an emit-owned `SKILL.md` and expects
  `gate_installed` to name it. Under the `src/install.rs` narrowing that
  hand-edit is drift by hash (`check`'s comparison), not a missing placement,
  exactly as the sibling comment in that test already says of the banner.
  Re-cut to a stale-*wording* drift (splice a retired note wording in), which
  is what `project_note` still converges.
- `tests/json_document_format.rs:469`
  `the_write_dispatch_leaves_a_frontmatter_member_and_a_formatless_one_exactly_as_they_were`
  — byte-exact `.claude/rules/rust.md` expectation; needs the note line added,
  the same edit `tests/emit.rs` took in-fence.
- `tests/gauntlet.rs` + `tests/snapshots/gauntlet__projection_tree.snap` —
  `the_gauntlet_corpus_emits_and_checks_to_stable_snapshots`; the projection
  tree snapshot carries every corpus projection's bytes, so it needs
  `cargo insta accept` for the note line. The snapshot file is the write, not
  the test source.

## Observed at

d78c6d19 (HEAD when observed) — the four failures are the whole delta;
`cargo test --no-fail-fast` was otherwise green with the change applied.

## Suggested consolidation

Re-scope the entry with the four paths added to `files.edit` (snapshot file
included, so `cargo insta accept` is a fenced write). The implementation that
was verified against them:

- `src/drift.rs` — one predicate `renders_frontmatter(format, fields)`
  (`YamlFrontmatter|None` and non-empty fields) that both `project_bytes`
  renders by and `emit_one` picks the marker form by, so the writer and the
  marker placer cannot disagree: frontmatter gets `NOTE_COMMENT`,
  frontmatterless markdown gets `BANNER`, JSON gets neither. Appending the note
  to the placements vector keeps install's modeline the leading line on the
  next pass.
- `src/install.rs` — `project_note` narrowed to the `converge_banner_wording`
  contract: `None` when no note line is present, converge-only when one is.
  `NOTE_COMMENT` needs to leave `install.rs`; its home is `src/placement.rs`
  beside `BANNER` (also out of the current fence — it was made `pub` in place
  as a stopgap).
