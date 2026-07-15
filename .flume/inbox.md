<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `specs/builtins.md` says "Five kinds ship" (line ~14) but seven authorable
  kinds ship in `sdk/src/builtins.ts` (`hook`, `mcp-server` added under 0021);
  the prose at the coverage-bar section already acknowledges them. Spec drift:
  the headline enumeration needs the correction. observed at e8edffa
- Built-in contract reconciliation against the 2026-07-15 Claude Code docs —
  drift register with per-item detail and source URLs in
  `docs/market-formats.md`, "Claude Code deep audit" section. Load-bearing
  items: commands merged into skills (`command` kind wants a legacy-posture
  note + cite refresh); skill frontmatter grew (new fields incl. `when_to_use`,
  `paths`, `context: fork`, component-scoped `hooks` — review `forbiddenKeys`
  and coverage against them; `paths` is a hard registration gate conditioning
  the other channels, verified empirically 2.1.210 — a composed channel the
  flat registration list can't express today, see the digest); `DOCUMENTED_HOOK_EVENTS` re-verify vs the
  current ~30-event set; rules `paths` + recursive discovery now first-class
  documented (cite refresh); agent `tools`-resolution failure now loud
  (v2.1.208+, candidate clause). Caveat carried in the digest: hooks/settings
  extracts were summarizer-mediated — any encoded `cite` re-fetches the raw
  page first, per the external-facts bar. observed at e8edffa
- `Requirement.kind` (`sdk/src/contract.ts:177-183`) is typed
  `KindDefinition<never>`, so a requirement cannot be keyed to any kind whose
  field type carries required members — `KindDefinition<Skill>`,
  `KindDefinition<Hook>` fail to assign; only all-optional-field kinds (rule,
  memory) work. The repo's own harness hit this: `.temper/harness.ts`'s
  `friction-capture-procedure` requirement documents dropping its `kind:` as
  a workaround. A requirement needs only the kind's identity for coverage
  resolution, never its field type; the collection child-kind slot already
  models this as `string | KindDefinition<any>` (`sdk/src/kind.ts:315`).
  Demand is live (human-ruled 07-15): the base-harness third cut prescribes
  skill/hook-keyed requirements. observed at 3540ebb
