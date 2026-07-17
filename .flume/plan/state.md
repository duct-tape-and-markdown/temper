# Plan state

- Spec derived through: b8396d4
- Audited through: b3a1636
- Residue swept through: b3a1636
- This tick: SPEC DELTA — route b8396d4 (`specs: amend 0028 — the trigger is
  the gate field, never paths-match registration`), the first live input
  (inbox empty, `.flume/refactor/` its README alone). **Zero entries derived:
  the amendment is dated errata on an already-routed Decision, and every
  claim in it verifies as shipped on disk** — not a log reading. The
  amendment's own claim, checked first: the trigger is the target's declared
  gate field carrying globs, hard-coding no kind — `Predicate::
  MentionReachable { scope_field, gate_field }` (`src/contract.rs:277`) and
  `graph::mention_reachable`'s header (`src/graph.rs:325-331`) both state it
  and both spell out *why* a registration lookup would be wrong ("would
  select rules and never skills, so the rule→skill mention this check exists
  for could never fire"). The premise behind it verifies too: `skill.paths`
  carries the gate-not-a-channel doc and no `paths-match` entry
  (`sdk/src/builtins.ts:120-131`), while `rule` alone registers that way
  (`sdk/src/builtins.ts:335`). **0028's Consequences re-walked bullet by
  bullet, all four verified-already-moot, none re-derivable:** the predicate
  variant ships (`src/contract.rs:277`, `sdk/src/contract.ts:137`); the
  `rule` default contract carries the clause at advisory with its own dated
  cite (`sdk/src/builtins.ts:724`); the built-in lock re-derived (the
  `mention-reachable` clause row at `src/builtin_lock.toml:296-302`, `gate =
  "paths"`); the fork record is gone from open-questions. So the cursor
  advances on verified routing, not on having looked. **One closing-checklist
  gate released, not a second job:** PLUGIN-JSON-DOCUMENT-FORMAT's
  `blockedBy: INSTALLED-PLUGIN-KIND` named a tag the queue no longer carries
  — the blocker shipped (9f22de2) and `chore(flume)` ce1a690 dropped it —
  verified on disk (`claude_code_installed_plugin` at
  `src/builtin_kind.rs:293`, in `all_kinds()` at 320;
  `tests/installed_plugin_kind.rs` present), so the gate is `open` and its
  notes re-worded. Both other cursors copied forward verbatim: this tick
  audited and swept nothing.
- Queue: 6 entries — 1 pickable (PLUGIN-JSON-DOCUMENT-FORMAT, in
  `src/`+`sdk/`+`tests/`), 3 serialized behind it on shared files
  (`builtin_kind.rs`/`builtins.ts`/`kind.rs`), 2 parked on human acts (a
  hop-depth probe; Apple notarizing + the v0.1 tag). Parks not re-tested this
  tick — they were re-tested at e81c758 and no commit since touches their
  conditions. No file appears in two `open` entries.

Plan continues: yes — post-ship reconciliation is live and untaken. Two
`build:` commits sit past both the audit and sweep cursors (b3a1636): 9f22de2
(`installed-plugin`, the third registration member) and 693f31f (the example's
edge targets as sets). Next tick audits that window on disk and sweeps it —
and the audit owes one named re-test: open-questions claims INSTALLED-PLUGIN-
KIND was the carrier that would discharge `src/read.rs`'s five stale strand
doc comments (270/495/633/770/1172). The entry shipped; whether the rider
discharged is a disk question, and the record stays until read.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
