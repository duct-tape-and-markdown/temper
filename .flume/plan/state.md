# Plan state

- Spec derived through: cd7135b
- Audited through: 5389103
- Residue swept through: 77b2eb9
- This tick: Ship audit (job 3). Commits past 99337b8 touching src/tests
  (706139a, 39079e3, 4ed4027) verified on disk: RETIRE-FLOOR-VOCABULARY-FOR-
  DEFAULT-CONTRACT's rename landed exactly where its own commit body scoped
  it (main.rs/compose.rs/read.rs identifiers, SDK exports,
  builtin_lock.toml header), deliberately leaving comment-only `floor`
  mentions to ride later entries — confirmed no live non-test symbol still
  named `floor` outside that carve-out; ACCEPTANCE-CUSTOM-KIND-VIA-LOCK
  rewrote both acceptance.rs custom-kind tests onto lock.toml declaration
  rows; INSTALL-WHOLE-CONVERSION rewrote install.rs's scaffold to a whole
  typed conversion. All three already absent from pending.json
  (auto-shipped). Re-tested stale gates per job 3: RETIRE-POSTURE-
  VOCABULARY-FOR-ENFORCEMENT-MODE was blockedBy INSTALL-WHOLE-CONVERSION —
  that shipped without touching src/install.rs's posture wording
  (reverified live at current line numbers: ~105-111, ~605; drift.rs
  ~1321; tests/install.rs's guard test names/labels) — blocker cleared,
  gate flipped to open. RETIRE-OWN-PATH-MACHINERY (blockedBy the posture
  entry) reverified: own_path is untouched and fully live in
  drift.rs/import.rs/main.rs/tests, so it stays correctly blocked,
  unchanged. PACKAGING-CHANNELS' parked reason reverified true (no
  release.yml, root package.json still the private flume manifest,
  sdk/package.json at 0.0.5).
- Queue: 3 — RETIRE-POSTURE-VOCABULARY-FOR-ENFORCEMENT-MODE now open and
  pickable; RETIRE-OWN-PATH-MACHINERY blockedBy it; PACKAGING-CHANNELS
  parked.

Plan continues: yes — `Residue swept through` (77b2eb9) trails HEAD
(5389103); job 4 is next. Note for that tick: RETIRE-FLOOR-VOCABULARY-FOR-
DEFAULT-CONTRACT's own commit body already carved out the remaining
`floor` mentions (test fn/variable names in lock_declaration_rows.rs,
session_start.rs, requirement_roster.rs, cli.rs, builtin_lock.rs; sdk/src
prose) as comment/identifier staleness riding later entries per
spec-system's exception — verify that reading still holds before filing
anything, since it may mean job 4 finds nothing fileable this class.
