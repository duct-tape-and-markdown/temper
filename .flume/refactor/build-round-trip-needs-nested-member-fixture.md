## Surface

`EMIT-REFUSES-A-PATH-ITS-GLOB-CANNOT-FIND` cannot reach green inside its fence:
the round-trip refusal it asks for fires on two **existing fixtures** in a file
`entry.files` does not list.

- `tests/it/nested_member.rs:244` and `:276` — both fixture programs declare
  `guide` as `unitShape: "directory"` at `glob: "GUIDE.md"`. A directory unit
  lands its entry file one segment deep (`<name>/GUIDE.md`), and
  `import::collect_glob` matches a glob segment-by-segment relative to the
  locus root, so that glob reaches only `.claude/guides/GUIDE.md` — the host's
  own projection is written yet ungoverned, exactly what the entry refuses. The
  declared shape is `*/GUIDE.md` (`src/builtin_kind.rs:123` spells `skill`'s
  `*/SKILL.md`; `tests/it/gauntlet.rs:67` already spells `*/GUIDE.md`).
  **Fix: `glob: "GUIDE.md"` → `glob: "*/GUIDE.md"` on both lines.** No assertion
  in either test moves — the derived path is byte-identical.
- `sdk/test/emit.test.ts:819` carries the same mis-declared glob. It stays green
  (the SDK half does no glob match, having no matcher), but it is a fixture
  teaching a shape the engine now refuses; worth the same one-token fix.

Everything else in the entry is written and verified at this SHA: `cargo fmt`,
`cargo clippy -D warnings`, `cargo doc` and `pnpm --dir sdk test` all green,
`cargo test --no-fail-fast` green but for those two. Left uncommitted.

## Observed at

703a7296 (HEAD when observed) — `--no-fail-fast` over the whole suite, so the
list above is complete.

## Suggested consolidation

Re-scope the entry with `tests/it/nested_member.rs` (and `sdk/test/emit.test.ts`)
added to `files.edit`, described as the fixture correction above rather than as
new behavior. What the tick validated, for the replay:

- The refusal belongs in `member_projection_path` before the locus dispatch, not
  in `splice_name`: the directory-unit branch never reaches the splice, so
  guarding the splice alone would leave the slashed name admitted for one shape.
- Two `DriftError` variants: `MemberNameSeparator` (name carries `/`) and
  `UngovernedProjection` (derived path, matched through `crate::glob::compile_glob`
  relative to `governs_root`, or to the host's unit for a nested file child).
- SDK-side, `isOneSegment` (`sdk/src/member-address.ts`) is the existing home for
  this predicate — `kind.ts`'s `refuseSegmentedKey` already holds it one grain
  down. Reuse it over `orderedMembers`' projected set (total, like the engine's)
  rather than adding a second `/` test inside `spliceName`, which only the
  link-derivation path reaches.
- `tests/it/check_cost.rs` needed no edit for the existing pin (its window is the
  discovery pass, which emit never enters); a new pin over `emit` measured the
  round trip at 2 compiles for 400 members of 2 kinds — per distinct glob, as the
  entry predicted.
