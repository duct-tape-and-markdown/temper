<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Field report (dogfood adoption, 2026-07-07): the scaffold↔published-SDK
  version skew bit for the second time. `install --yes` scaffolds HEAD's
  two-arg `file(import.meta.url, "…")` (0009 module-relative) but ensures
  `^0.0.4`, whose published `file(path)` is one-arg — JS swallowed the extra
  arg and emit died ENOENT on a cwd-joined `file:` URL (`src/install.rs`
  `SDK_VERSION_RANGE`'s own comment records the identical ^0.0.2→^0.0.4
  incident). The gap is systemic: nothing gates the scaffold's SDK-surface
  assumptions against the version range install writes — cargo tests ride
  the workspace sdk, never the registry tarball. Candidate shapes for plan:
  pin exact + a release-ritual bump, a scaffold-vs-published contract test,
  or install preferring a workspace sdk when one is present (what the
  dogfood hand-wired: `file:../sdk`). Also noisy-but-survivable: the emit
  failure surfaces as a raw Node stack through the miette report.

- No-middles sweep findings (0017, 2026-07-07), three routes: (1) the
  "declared-and-inert" pattern — `src/kind.rs` carries columns documented
  "inert until <entry> lands" (the declared-frontmatter adapter,
  reachability, the nested-member predicate) — 0017 rules a declared
  surface ships with its consumer, never ahead of it; fold into entry
  scoping so no column lands before the entry that reads it, and reconcile
  the existing inert columns. (2) `src/kind.rs` ~1186: a legacy-key lock
  fallback kept "until a human re-emits" the old dogfood lock — a pre-1.0
  compatibility shim CLEAN SLATE forbids; the committed `.temper/` re-emits
  under PRs #15/#16, so retire the fallback. (3) Evidence only, no entry:
  `sdk/src/assembly.ts`'s residual settings list and `sdk/src/emit.ts`'s
  permissions "carried here until then" die with the 0015/0016 derivations;
  `.github/workflows/temper.yml` still runs the retired `import` verb
  (flagged in PR #15).
