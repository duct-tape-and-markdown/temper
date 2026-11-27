# LOCK-FROM-ANOTHER-ENGINE-IS-STALE

Shipped. Things the next plan tick, or a human, has to act on:

- **Dogfood lock needs a human re-emit.** A checkout build now stamps
  `X.Y.Z+unreleased`. `.temper/lock.toml` still says `0.0.25`, so
  `temper.yml`'s `emit --frozen && git diff --exit-code` is red on main
  until a `chore(harness):` re-emit lands. Self-host check stays ok: it
  only gets an advisory engine-matches finding.
- **Release recipe.** Only the release workflow's build (`TEMPER_RELEASE`
  is set in `release.yml`) and a git-less crates.io tarball are
  "released". A tag on HEAD is not the signal, because that would flip
  committed locks at the cut. Committed locks therefore carry
  `+unreleased` permanently, and `.claude/rules/release.md` step 5 should
  say the re-emit writes `X.Y.Z+unreleased`. A release engine reading this
  repo's lock reports it as another engine's, which is true. The example
  lock (`examples/base-harness`) was restamped by its one changed line;
  its byte-compare test proves that line equals emit output.
- **Scope of the stale reading.** Only a built-in-named row that would
  split as a *collision* is retired under another stamp. A snapshot row
  that differs only in facts the overlay applies (`exclude`, `ceded`,
  locus) still overlays the embedded built-in, because it cannot be told
  apart from an authored `relocate`. The dogfood memory row is this case:
  its `exclude` still lists `.claude/rules`. Whether such a row should
  also read as embedded is a spec question.
- Retirement happens only in `gate`. `explain`/`read.rs` and `main.rs`
  still partition the raw rows. They already discard collisions, but
  under another stamp they apply the stale overlay.
- An unknown stamp (none recorded) keeps the collision reading.
