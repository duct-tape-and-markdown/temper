
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
- **One job, one home — extend before adding**
  (`specs/process/engineering.md`). Before a new fn/module/helper, `rg` for
  the existing surface; prefer delete/subsume > extend > generalize the
  near-duplicate > add new — and a new surface beside a near-duplicate names,
  in the commit body, what you considered and why it didn't fit. A sanctioned
  crate (direct or transitive) beats a hand-roll of the same mechanic; shared
  test scaffolding lives in `tests/common`, never per-file copies.
- One artifact kind per module (`skill`, later `hook`, `agent`, …). The `Rule`
  trait takes the whole workspace, never a single artifact — cross-artifact rules
  must slot in without a signature change.
- Keep `main.rs` a thin `clap` dispatch; logic lives in the library crate so
  `tests/` can drive it.
- Public items carry `///` docs: **one-line third-person summary first**, then
  `# Errors`/`# Panics` where reachable (RFC 1574; api-guidelines C-FAILURE).
  Module `//!` headers are an **overview, ~10 lines** —
  "without excessive detail" (RFC 1574); the spec documents itself, the header
  never restates it.
- **Comment only what the code can't say** (Ousterhout, *APoSD* ch. 13;
  antirez taxonomy, antirez.com/news/124). Keep: the *why* (chosen-over-
  alternative, ordering constraints, workarounds); invariants, units, edge
  behavior (`BTreeMap` for stable output); checklist warnings ("change X → also
  Y"); a what-summary only above code genuinely denser than the sentence.
  **Spec citations are retired from comments**: provenance lives in the
  pipeline (the entry's gated `per` cite) and git — a comment pointer is an
  unchecked cache that rots wholesale. State the constraint, never its
  source. Cut: spec-path pointers, restated spec narrative (a second,
  unchecked home for intent that drifts), narration of ordinary code,
  compliance narration ("per §X we…" — commit-message material). Comments are
  paid twice here: every agent reading the module bills for every line.
- **Comment churn is diff cost.** Never rewrite a comment whose constraint
  didn't change; when replacing or demolishing code, don't port the old
  prose — new code carries the taxonomy's minimum, written fresh. Module
  `//!` headers shrink on touch, never grow. A slice whose diff is mostly
  comment edits is over-commented — cut, don't polish.
- **The exit clause**: deleting or tag-ifying a comment
  the taxonomy cuts is **always in scope** on any entry touching the file —
  the anti-churn rule above bounds *rewrites*, never *removals*. Comments are
  the one ungated surface (no gate checks their truth), so their lifecycle is
  editorial: cut on contact, never annotate. Era narration ("no longer X",
  "renamed from", "shipped in Y") is commit-body material — git owns history.
- **Mechanical repo-wide edits** (`sed`/regex sweeps over many files): scope
  the match to comment-marked lines (`///`, `//!`, `//`) so string literals
  and test fixtures stay out of reach, and run the full test suite — not just
  a build — before calling the pass done; a green build proves nothing about
  fixture strings.

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
  assumption cannot falsify it.
