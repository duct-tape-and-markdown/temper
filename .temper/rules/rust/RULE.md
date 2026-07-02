+++
[clause.paths]
value = ["src/**/*.rs", "tests/**/*.rs", "benches/**/*.rs"]

[satisfies.engineering-standards]
rationale = "the path-scoped home for the Rust bar: diagnostics discipline, round-trip invariants, test conventions — loaded exactly when Rust files are in play"

[provenance]
source_path = "./.claude/rules/rust.md"
import_hash = "101393bd41985ecf3a373b81ed1a66567fbccd29ba7b13a43b7942aa0238921e"
+++

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
- Public items carry doc comments. Module headers state the artifact/schema they
  model and cite the `specs/` section.
- **Cite, don't restate.** A comment carries the spec pointer plus only what the
  code *adds*: an invariant the code can't show (`BTreeMap` for stable output), a
  deliberate narrowing ("parse-only tier; discovery is a follow-on entry"), a
  deviation. Spec narrative lives in the spec — a paraphrased model in a module
  header is a second, unchecked home for intent that silently drifts when the
  spec moves (the exact failure this tool hunts). Compliance narration ("this
  matches §X because…") belongs in the commit message, not the file.

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
