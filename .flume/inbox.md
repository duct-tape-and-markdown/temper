<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at e1b26728 (cascade, GH #39) — `guard` in block mode allows an
  absolute `file_path` to the root `CLAUDE.md`. `install.rs` `path_matches`
  is a suffix compare with a single-segment rule: when the candidate has no
  `/`, the prefix before the match must contain no `/` either, meant to stop
  `.temper/memory/CLAUDE.md` matching `CLAUDE.md` — but Claude Code always
  passes absolute paths, so the prefix is the repository's own directory and
  the root memory file never matches. `mode: "block"` on `CLAUDE.md` is
  cosmetic. This is the ledger's parked string-path-compare posture rule
  made concrete: the guard should relativize `file_path` against the harness
  root it already holds (`path` on the `Guard` command) and compare paths,
  never suffixes — the one glob engine / one path module discipline.

- observed at e1b26728 (cascade, GH #40) — a `Write` to a represented
  `.claude/settings.json` whose content drops every hook conforms vacuously:
  `manifest_write_findings` validates the members the pending content
  *carries* and never compares against the roster the lock declares, so a
  manifest emptied of its registration members produces no finding and is
  allowed at every mode. A 0048 instance on the guard: the write represents
  less than the lock was given. Fix: the manifest write check judges the
  lock's expected members too — an emit-owned collection entry absent from
  the pending content is a finding at the declared mode. Second half
  (`Edit` allowed) is a real regression, reproduced on THIS harness at
  e1b26728, mode block: `guard .` exits 0 for an Edit or Write to
  `.claude/settings.json` and 2 for an Edit to `.claude/rules/rust.md`.
  Mechanism: `drift::emit_owned_targets` decides settings.json is
  emit-owned when `walk_lock_rows` yields a row whose table name is
  `hook`/`installed-plugin`/`known-marketplace` — but the walk reads only
  top-level `[[<kind>]]` array-of-tables, and a real emit writes hook
  members under `[[declaration.registration]]` (kind column `hook`), never
  a top-level `[[hook]]` table; `tests/install.rs`
  `guard_binds_settings_json_when_registration_members_compose` forged a
  `[[hook]]` row emit never produces, so it is green over a path no lock
  takes. settings.json is unguarded on every real harness. Fix: derive the
  target from `Declarations::registrations` (the family the gate reads),
  and recut the test to a lock `emit` actually wrote. The #39 root
  `CLAUDE.md` case reproduces here too (exit 0, mode block).

- observed at e1b26728 (cascade, GH #42) — `check --deny-advisories` can
  never pass: `coverage_note::check` pushes `coverage.checked` unconditionally
  at warn severity, `main.rs` flips the exit on any warn, and the harness
  note is present on every represented harness. `--deny-advisories` is
  documented as promoting advisory *violations*; a coverage note is
  disclosure, not a violation (intent invariant 5 — a coverage clause
  enters advisory so it is dialable, not so it blocks CI). Fix: coverage
  notes carry a distinct non-violation marker (a third severity, or a
  `note` flag on `Diagnostic`) that `--deny-advisories` ignores and the
  session-start reporter still prints; the acceptance suite gains "clean
  harness + `--deny-advisories` exits zero", which no test asserts today.
