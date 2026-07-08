# Plan state

- Spec derived through: cd7135b
- Audited through: 99337b8
- Residue swept through: 5264c0f
- This tick: Residue sweep (job 4). No commits past 99337b8 touch
  src/tests/sdk (HEAD is 5264c0f, the prior plan commit itself, writable-fence
  restricted) so job 3 stayed quiet and job 4 took over. Re-verified the
  standing accepted debt (open-questions.md, "kinds/ + packages/ curated
  trees — RETIRED"): tests/session_start.rs's stray `.temper/kinds/spec/
  KIND.md` + `.temper/packages/spec/PACKAGE.md` fixtures are untouched and
  still honestly named (asserts they're inert) — no entry rides it yet, note
  stands unchanged. Swept further and found a live gap the note didn't cover:
  tests/acceptance.rs's `check_dispatches_the_spec_custom_kind_...` and
  `check_reads_a_custom_kind_rooted_outside_specs` author the same dead
  KIND.md/PACKAGE.md format, but their `--deny-advisories` assertions are
  actively misleading, not just inert — confirmed empirically that the flip
  fires on ANY corpus (a fixture-less control still flips non-zero) via the
  always-on `coverage.checked`/`install.gate-installed` warn notes, so the
  tests currently prove nothing about custom-kind dispatch despite their
  names/docstrings. Filed ACCEPTANCE-CUSTOM-KIND-VIA-LOCK to rewrite both
  onto lock-declared kinds with diagnostic-content assertions. No other
  KIND.md/PACKAGE.md or temper.toml residue found (tests/requirement_roster.rs's
  temper.toml tests correctly assert inertness, not dead-format misuse).
- Queue: 3 — INSTALL-WHOLE-CONVERSION (open) and
  ACCEPTANCE-CUSTOM-KIND-VIA-LOCK (open) are both pickable and file-disjoint;
  RETIRE-OWN-PATH-MACHINERY (blockedBy INSTALL-WHOLE-CONVERSION);
  PACKAGING-CHANNELS (parked).

Plan continues: yes — quiet closing pass (job 5) is next; inbox, spec delta,
ship audit, and residue sweep are all current as of this tick.
