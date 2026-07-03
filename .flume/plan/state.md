# Plan state

- **Phase:** reconcile. HEAD 2cc26ff.
- **Last shipped:** CHECK-WORKSPACE-KIND-MAP (build ef73b49) + DECLARED-FRONTMATTER-ADAPTER-CUSTOM
  (build 247e203), chore 2cc26ff — the memory wave's slices 3+4. `check::Workspace`
  now keys members by kind (`load()` iterates `builtin_kind::definitions()`), and
  `import_custom_unit` branches on the declared `format`: `yaml-frontmatter` rides
  `Member::from_source`, no-format keeps the whole file byte-faithful, and the id
  derives per the declared `unit_shape` in `wholefile_id` — both verified on disk.
- **This tick:** reconcile-only. Slice 4 shipped → RECURSIVE-GOVERNS-PLACEMENT-ID's
  `blockedBy DECLARED` is satisfied; unlocked it to `open` and refreshed its drifted
  cites — DECLARED split `import_custom_unit` into `wholefile_id` (import.rs:479-501) +
  the `from_source` File face (frontmatter.rs:186-196), and the glob scan is
  `discover_kind_units`/`collect_glob` (import.rs:326-380) + `glob_matches` (:545). No
  other entry moved; inbox empty; no new corpus↔src gap. Open questions unchanged.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** — re-verified this tick:
  the freshly-built binary's `temper check .temper` is clean (only advisory `max_lines`
  on the deliberate long-spec fixtures, exit 0); the stale `~/.cargo/bin/temper`
  reproduces the old findings. Fix is `cargo install --path .`, not spec/build work.
  (`temper check .` errors on a missing `kinds/architecture/KIND.md` — that is the
  wrong workspace; the dogfood is checked via `check .temper`, per install.rs.)
- **Pickable now:** RECURSIVE-GOVERNS-PLACEMENT-ID (sole `open`; the last engine slice
  of the memory wave). Parked: MEMORY-KIND (flip-ceremony validation, curated files),
  PACKAGING-CHANNELS, COMMUNITY-DOCS (human action). Deferred: EXTRACTION-VOCAB-GAPS,
  AGENT-KIND (no consumer). OPEN forks stay human-to-settle.

Plan continues: no — queue reconciled against disk, inbox empty, one `open` entry
pickable. Hand to build; shipping RECURSIVE drains the last engine slice of the wave.
