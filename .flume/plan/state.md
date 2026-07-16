# Plan state

- Spec derived through: 39a4833
- Audited through: ff7da32
- Residue swept through: ff7da32
- This tick: ROUTE the inbox (3 notes, stamped f08ffca/4a47b2b — that window
  carries no src/sdk/tests commit, so every premise was re-verified live rather
  than diffed forward). (1) dark-mention containment → open fork
  `(mention-gate-containment)`: the check needs a predicate the closed vocabulary
  lacks (`src/contract.rs:81` — `degree` is the only mention-edge predicate, none
  compares glob sets), and a language change is decision-tier (0022 `f67303c`
  ratified `glob-valid` before `46b8cd1` built it), so plan registers rather than
  files. (2) gate-opening-is-loud → evidence folded into that fork's cost side;
  the authoring-guidance digest is docs/human territory. (3) engine finding → two
  entries, both probed live at f4e3c13: CHECK-WORKSPACE-DIR-RESOLVE
  (`check .temper` → 1 member + 2 false `requirement.unfilled`, exit 0;
  `check .` → 10 members, green) and INSTALL-REPRESENTED-SKIPS-THE-QUESTION
  (`install . --dry-run` re-asks the settled fork and defaults No; `--yes
  --dry-run` already converges, placing the 3 notes `check .` demands).
- Queue: 1 pickable (CHECK-WORKSPACE-DIR-RESOLVE);
  INSTALL-REPRESENTED-SKIPS-THE-QUESTION blockedBy it (both edit src/main.rs);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag) — condition re-tested, still unmet.

Plan continues: no — the inbox job is complete and no input below it is live:
spec delta empty (39a4833 fully derived), no src/tests/sdk commit past ff7da32
(208e8f8 touched only .temper/, .claude/, examples/, docs/, CLAUDE.md). Cursors
copied forward verbatim — reconciliation was not this tick's job. The prior
tick's NB is retired: 208e8f8 flipped the SessionStart hook to `temper check .`,
and `check .` probes green on requirements (10 members, neither unfilled) — that
red was the same half-resolve CHECK-WORKSPACE-DIR-RESOLVE fixes, not a harness
gap.
