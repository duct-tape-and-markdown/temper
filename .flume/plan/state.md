# Plan state

- **Phase:** reconcile. HEAD eff71a1.
- **Last shipped:** INIT-ONRAMP (`build` f7a830b / `chore` eff71a1) — the on-ramp
  is now `init`: scan a harness into a `temper.toml` manifest over its members
  **in place**, zero `.temper/` copy tree (`main.rs:273`, `Command::Init` →
  `import::init`, per `20-surface.md` "Decision: `init` is the on-ramp"). Sixth
  and penultimate link of the floor `blockedBy` chain.
- **This tick:** inbox empty. Reconciled: INIT-ONRAMP shipped, so
  EMIT-OWNED-PLACEMENTS's `blockedBy INIT-ONRAMP` is discharged → **flips to
  `open`**, the chain tail and sole open head. Verified it unbuilt on disk:
  `project_bytes` (`drift.rs:576`, called from the double-emit verify at 518-524)
  still re-emits frontmatter from `fields` only, never reading the on-disk
  projection — so a whole-file emit drops install's schema modeline
  (`project_modeline` install.rs:630) + reflows the YAML; `project_note`
  (install.rs:653) and `the_managed_by_note_is_never_written_by_apply`
  (tests/install.rs:453) still stand. Corrected the entry's one stale line-ref
  (553 → 576).
- **Pickable now:** **EMIT-OWNED-PLACEMENTS** (`open`) — emit preserves install's
  modeline/managed-by lines instead of clobbering them; ends the `gate_installed`
  re-nudge loop; lifts the interim "do not run bare `apply`/`emit` on rules"
  discipline. Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer). Parked:
  PACKAGING-CHANNELS (human creds).
- **What's next:** build ships EMIT-OWNED-PLACEMENTS — the last floor-wave link;
  once it lands the whole `scripted-altitude` **floor** chain has drained (only
  the altitude rung stays parked on John's SDK/npm ask (a)). Human still owes the
  accepted-debt `temper.toml`+lock regen (∉ build's fence) so the manifest-read
  path exercises the dogfood, not just fixtures.

Plan continues: no — queue reconciled, EMIT-OWNED-PLACEMENTS is a pickable `open`
head, inbox empty. Hand to build; re-planning would re-emit the same queue.
