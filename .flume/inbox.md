<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## README drift vs the ratified corpus (preflight for the public flip, 2026-07-05)

The README is governed by `specs/intent/55-offering.md` ("The front door —
the README is the landing page") and it largely complies (tagline structure,
committed+regenerable diagnostic hero, honest pre-1.0 status, AGENTS.md at
root, run-on-your-own-harness quickstart). The drift is against
**20-surface's ratified CLI and posture**, verified against the trunk-head
binary (`temper --help`):

- README.md:77 (Status): the verb list promises `import` and `diff` — both
  retired by the demolition. The real surface is init / check / schema /
  emit / guard / install / bundle / explain.
- README.md:42 ("A typed surface, not a rule pile"): "`import` scans the
  whole harness" — `import` is no longer a verb; the scan lives in `init`
  and `check --harness`'s internal one-shot import.
- README.md:46 ("Requirements you declare. In a `temper.toml` …"): describes
  the hand-authored-TOML posture the sole-producer ruling rejects
  (20-surface: emit is the sole lock producer; the postures are prose media,
  never config dialects). Needs re-grounding in the ratified model.
- AGENTS.md:28 mentions `import.rs`/`drift.rs` as orientation — check it for
  the same staleness while filing, but it's contributor docs, lower stakes.

README.md and AGENTS.md sit outside the fence (`^(\.claude|docs|specs)/`),
so this is fileable build work. Scope honesty: this is a *reconcile* of the
stale claims, not a README rewrite — 55-offering's larger launch obligations
(logo, badge polish, ~800-word shape) ride the v0.1 offering work, not this
entry. Context: the repo flips public soon; the front door describing the
pre-demolition product is the sharpest edge.
