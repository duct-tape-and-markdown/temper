<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- 2026-07-04 (John, session): the intent-named harness surfaces are **in
  scope now** — the deferrals were phase judgment ("revive when a story
  demands"), and the story has arrived: whole-harness governance plus the
  first non-dogfood adoption. Ruling, per `specs/intent/00-intent.md` (the
  surface enumeration) and `specs/architecture/15-kinds.md` (the adapter's
  kind enumeration — `agent`, `hook`, `command`, MCP, settings, plugin):
  - **Revive AGENT-KIND** — its own revival condition is met. The reframe
    concern stands satisfied by its shape: mostly curated data (KIND.md +
    PACKAGE.md), near-zero engine code.
  - **File the remaining intent-named kinds**: `command`, `hook`,
    `settings`, MCP, plugin/marketplace manifests — engine slices where the
    adapter needs them, curated kind/package definitions surfaced back to
    the session (they sit outside build's fence and carry external-fact
    citation duty: each format claim needs its doc URL + retrieved date).
  - The **settings kind is a JSON-manifest kind** — the named consumer that
    revives EXTRACTION-VOCAB-GAPS' key-path half (`Primitive::Field` nested
    paths). Serialize the two accordingly.
  - Sequencing under plan's judgment; nothing here preempts the SDK seam
    tails. The check advisory `coverage.unmodeled-surface` on
    `.claude/settings.json` is the live marker to retire.
