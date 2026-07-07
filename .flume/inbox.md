<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

Field report 2026-07-07 (Windows 11 / centercode-platform testbed, 16
members, at 18dca38). Nine findings; verified fix shapes noted. Route each:

- WIN npm spawn: `install --yes` dies at `Command::new("npm")`
  (src/install.rs:1032) — Windows ships only `npm.cmd`, Rust spawn resolves
  `.exe` only. Fix verified: spawn `npm.cmd` under `cfg!(windows)` (or
  `cmd /c npm`). SAME ENTRY: the failure lands after scaffold, leaving a
  half-scaffolded `.temper/` — reorder so dependency assurance precedes
  scaffold (pipeline.md install: "no half-scaffolded state").
- WIN verbatim path: `emit` feeds Node `fs::canonicalize`'s `\\?\`-form
  entry path (src/drift.rs:382); Node dies in resolveMainPath. Keep the
  canonicalize, strip the verbatim prefix (`dunce` crate is the standard
  fix). Verified: with prefix stripped, first emit reproduces 16 projections
  byte-identically.
- SDK version range: src/install.rs:58 pins `^0.0.2`; npm caret on 0.0.x
  pins the PATCH (>=0.0.2 <0.0.3), so fresh installs get 0.0.2 — which
  predates the `file` export the scaffold imports — on every OS. 0.0.4
  (seam v2, has `file`) is published. Bump the range to match the seam the
  binary speaks; scaffold output + version range are one contract, changed
  in one commit.
- Path separators: scaffolded modules and lock rows embed `\` on Windows
  (`file("../.claude/rules\\cls.md")`, mixed-sep source_path). `.temper/` is
  committed and cross-platform; normalize both writes to `/`.
- guard vs own_path: guard regex-matches any `.claude/` file_path without
  consulting the lock; for `file()`-carried members the `.claude/` file IS
  the authored source (install itself skips placements on them; emit adopts
  direct edits). Under block it denies editing the source of truth. Guard
  reads the lock; own_path targets pass through.
- bundle report: on a 16-member surface `bundle` reports "surface: 0 skills,
  0 rules". Spec reads gate-delivery-only (distribution.md channel 3: the
  skill + SessionStart hook), so the composition is right and the REPORT
  line is the bug — name what the bundle ships, not member counts. If plan
  reads member delivery as intended instead, that is a fork, not an entry.
- explain not-found: `explain protocol` prints the not-found error (with a
  retired `import` verb suggestion) then narrates the member anyway — the
  resolver finds it one namespace later than the message checks.
- help-text recut: `guard --help` still speaks shared/surface (0006 recut to
  note/warn/block; runtime already migrated); most verbs' help cites the
  deleted specs/architecture/* paths. User-facing strings — OUTSIDE
  CITE-RETAG's comment scope; file separately.
- settings.json fidelity: install's hook merge is semantically correct but
  re-serializes the whole file (keys reordered, EOL churn). The round-trip
  discipline the corpus states for TOML/markdown has no stated JSON
  equivalent — if no clean cite exists, route as an open question
  (key-order-preserving insertion is the candidate ruling).
