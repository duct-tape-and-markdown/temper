<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- FOLLOW-ON (dogfooding the installed hook): `session-start` on a project WITH
  an authored surface must check the surface (two-step path), not re-import —
  a fresh import discards `satisfies` recognition, so temper's own two filled
  requirements report as blocking unfilled-coverage findings at every session
  start (false positive on clean input, law 3). Spec now pins the dispatch
  (specs/50-distribution.md, session-start Decision: surface present -> check
  the surface; surfaceless -> one-shot import). Repro: `temper session-start .`
  at temper's root — expect 0 blocking findings once fixed.
