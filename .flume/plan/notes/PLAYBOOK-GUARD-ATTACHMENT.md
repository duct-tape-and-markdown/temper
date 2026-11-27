# PLAYBOOK-GUARD-ATTACHMENT

Shipped the guard attachment: `attachments.guard: { mode: "point" | "refuse", tool? }`.

- Seam: new `[[declaration.area]]` family (`AreaRow { rule, paths, mode }`, `AreaMode`
  closed vocabulary, an out-of-vocabulary `mode` is `LockRowError::Vocabulary`). It
  rides `Declarations.areas`, so `compose::read_committed_declarations` carries it with
  no change to `compose.rs` (the entry predicted an edit there; none was needed).
- Hook: the attachment is a `PreToolUse` gate hook over `GUARD_MATCHER` (moved to
  `builtins.ts`). One command per edge: `harness()` now drops a gate hook repeating an
  earlier one at the same address with the same tool and satisfies, so a playbook
  composed beside `gateHooks()` rides the harness's guard. A differing tool still hits
  the duplicate-address refusal.
- Area paths: a paths trigger's globs; a nested trigger is `<dir>/**`. A playbook with
  no trigger refuses a guard attachment at construction: derived from "an edit on the
  area's paths", since a launch-loaded area names none. Worth a spec sentence if plan
  reads it differently.
- Guard: the area's own mode decides, independent of the harness enforcement mode
  (refuse exits 2, point is an in-band Warn); `install::with_area` keeps the firmer
  verdict and both messages when another guard also fires. The rule is named by address
  plus its projection path from the lock's provenance rows.
