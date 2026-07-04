---
# temper: managed projection — edit the .temper/ surface, not this generated file (temper re-add lifts a direct edit back).
# yaml-language-server: $schema=../../.temper/schema/rule.json
paths: ["src/**/*.rs","tests/**/*.rs","benches/**/*.rs"]
---

# Rust conventions — author

Path-scoped to `src/`, `tests/`, `benches/`. The bar is `cargo clippy
--all-targets -- -D warnings` clean and `cargo fmt --all --check` clean — those
are flume gates, so a violation reverts the commit.

## Errors & diagnostics

- This crate's value *is* its diagnostics. Model errors with `thiserror` and
  surface them with `miette` (`#[derive(Diagnostic)]`), including source spans
  where a file/line helps the user. `main` returns `miette::Result<()>`.
- **No `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!` on real code paths.**
  `expect` is allowed only for genuine invariants that cannot fail (document
  why). Tests may `unwrap` freely.
- A lint `Diagnostic` is a value the engine collects, not a thrown error — don't
  conflate "the artifact is invalid" (a finding to report) with "author failed"
  (an `Err`).

## Style & structure

- Prefer a `clone` or an owned `String`/`PathBuf` over a lifetime fight. This
  tool is I/O-bound over kilobyte files; zero-copy buys nothing and costs
  readability. Optimize only with a measured reason.
- One artifact kind per module (`skill`, later `hook`, `agent`, …). The `Rule`
  trait takes the whole workspace, never a single artifact — cross-artifact rules
  must slot in without a signature change.
- Keep `main.rs` a thin `clap` dispatch; logic lives in the library crate so
  `tests/` can drive it.
- Public items carry `///` docs: **one-line third-person summary first**, then
  `# Errors`/`# Panics` where reachable (RFC 1574; api-guidelines C-FAILURE).
  Module `//!` headers are an **overview plus the spec pointer, ~10 lines** —
  "without excessive detail" (RFC 1574); the spec documents itself, the header
  never restates it.
- **Comment only what code + cited spec can't say** (Ousterhout, *APoSD* ch. 13;
  antirez taxonomy, antirez.com/news/124). Keep: the *why* (chosen-over-
  alternative, ordering constraints, workarounds); invariants, units, edge
  behavior (`BTreeMap` for stable output); checklist warnings ("change X → also
  Y"); a what-summary only above code genuinely denser than the sentence. Spec
  citations are **terse pointer tags** (`// specs/architecture/20-surface.md, drift`), never
  prose recaps — the DO-178C trace-tag form. Cut: restated spec narrative (a
  second, unchecked home for intent that drifts), narration of ordinary code,
  compliance narration ("per §X we…" — commit-message material). Comments are
  paid twice here: every agent reading the module bills for every line.

## Round-trip discipline (the core invariant)

- Markdown bodies and companion files are copied **byte-for-byte** — never
  re-rendered. Only structured headers are rewritten, via `toml_edit` (preserves
  comments/order/formatting). Never reach for a lossy serialize-from-scratch on
  anything a human edits.
- Every imported artifact records `provenance { source_path, import_hash }`.
  Treat it as load-bearing, not bookkeeping.
- `import` must be idempotent: re-importing an unchanged harness produces an
  identical workspace (assert this with an `insta` snapshot).

## Tests

- Prefer `insta` snapshots for parse output and lint diagnostics — they make the
  contract reviewable and catch incidental drift.
- Fixtures of deliberately-broken artifacts (one per rule) live under
  `tests/fixtures/`; each rule has a test that proves it fires and that clean
  input does not trip it.
- **Harness-input fixtures mirror the real Claude Code layout**
  (`.claude/skills/<name>/SKILL.md`, `.claude/rules/*.md`) — never a layout
  invented for the test's convenience. A fixture shaped by the code's own
  assumption cannot falsify it (the import-locus bug hid behind exactly this).
