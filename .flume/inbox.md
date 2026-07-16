<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- **Plugin/marketplace manifest kinds are identity-stated and unshipped.**
  `CLAUDE.md`/intent name "plugin & marketplace manifests" among the artifact
  kinds temper projects; no kind, contract, or emit exists for either
  (verified: no `plugin`/`marketplace` kind in `sdk/src/builtins.ts`). A
  simulated consumer war game (8 sonnet personas authoring against the real
  specs, every gap adversarially verified against the repo, 2026-07-16)
  surfaced the producer-side story as three adoption-blockers for an OSS
  skill-pack author: a version field, a compatibility statement, and the
  manifest kind itself — today a hand-authored `plugin.json` sits ungated
  beside a fully-gated skill roster and drifts silently. Encoding needs the
  live plugin/marketplace docs fetched and cited first (external facts) —
  route as an investigation/spec entry, not straight to build. Demand is
  simulated; the identity-vs-shipped gap is verified on disk. Observed at
  eb2776d.

- **War-game datums for two parked forks — evidence updates only, neither
  unparks.** Same campaign, same provenance (simulated, adversarially
  verified). `(multi-harness-projection)`: 2/8 personas rate one-member→N
  projections adoption-blocker and want a counterpart-drift check
  (claude/cursor/AGENTS.md); the fork's demand side is no longer zero, its
  timing unchanged. `(eval-capability)`: the MCP-heavy persona rates
  behavioral verification partially-expressible and won't retire an in-house
  eval harness without it — consistent with the record's 07-16 field
  evidence. Observed at eb2776d.

- **War-game long tail — vocabulary and UX datums, all simulated demand,
  none queue-jumping.** Each predicate needs its own 0022-style ceremony
  when its real consumer arrives: conditional-on-sibling-field validation
  (MCP `type: stdio` requires `command`); a hook-command-resolves-on-disk
  edge; field-level schema over global settings; a rule-`paths`-containment
  predicate for monorepo team ownership. One fork-shaped tension worth a
  future argument: a lastReviewed-staleness clause requires the gate to
  read the clock, colliding with deterministic, byte-reproducible check.
  UX: single-file `check` for the solo shape; a guided walk-and-convert
  onboarding for fully custom (non-claude-code) corpora. Observed at
  eb2776d.

