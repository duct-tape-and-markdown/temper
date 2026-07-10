# 0021 — the manifest authoring surface

- **Date:** 2026-07-10 · **Status:** accepted

## Context

0015 ratified the semantics — a manifest represents its container's segment;
registration members are fields-only kinds at collection addresses — and left
the spelling open (`(manifest-authoring-surface)`): the authoring surface,
the emit/write architecture, and machinery none of which existed. Field
evidence sharpened the priority: a consumer's misfiring hook lived in the one
surface the gate itself flags `coverage.unmodeled-surface` — as does this
repo's own guard wiring.

## Decision

- **Authoring.** `hook` and `mcp-server` are built-in kinds: ordinary
  `kind<T>()` values at the embedded locus in `@dtmd/temper/claude-code`,
  fields-only by type shape (no prose slot exists to fill). The one new kind
  fact is the **collection address** (`hooks.<Event>`, `mcpServers.*`): which
  manifest, which key path — the manifest's fence, already named in 0015.
  Members compose into `harness()` and nest under the root member;
  unschematized settings residue stays the root member's opaque fields
  (`model/pipeline.md`, "The SDK"), so a represented manifest is never
  part-authored.
- **Members, not edges.** A registration member is a full member: it
  satisfies requirements like any member, and its member-to-member
  relationships exist only at the declared loci (`model/contract.md`) — a
  satisfies entry, or a documented field its kind marks as an edge field. A
  command string is prose to the graph; no edge is ever mined from it.
- **Write architecture.** One JSON manifest adapter, `frontmatter.rs`'s
  peer: a real parser owns the grammar, declared collection addresses walk
  into the generic extraction, undeclared keys are opaque fields. Reading an
  unrepresented manifest infers its registration members. A represented
  manifest regenerates whole — declared order then residue, LF — under the
  container's provenance row; registration members carry no lock rows of
  their own (0012). The unrepresented write stays 0008's splice.
  `bundle.rs`'s bespoke manifest writes convert to general-write instances
  (0015's named consequence).
- **Order of work.** Read side first (adapter read face, the fields-only
  kind shape, the collection-address kind fact, the kinds), hook before
  mcp-server, severable at the kind boundary; each kind files with its own
  doc fetch-and-cite (0014) and a strictest-documented-profile default
  contract. The consumer-face plugin kind follows. No ordering constraint
  binds this campaign to the v0.1 tag in either direction; the meta-freeze
  admits exactly this campaign and holds for everything else.
- **The guard asymmetry, named.** The guard is registered in the manifest it
  protects; a hand-edit deleting that entry cannot be blocked by the entry
  being deleted. Drift at the next check is the designed recovery — no
  self-healing placement, no shipped "guard must be wired" required clause:
  temper never escalates on its own determination (0006), and the default
  contract holds no taste about the tool's own adoption. Simple mechanisms
  over clever ones.

## Rejected

- Gating in either direction between this campaign and the v0.1 tag (the
  session grill's finding: the tag's gate needs none of this; this needs
  none of the tag).
- A distinct SDK constructor species for registration members (0012
  generalizes: locus is a kind fact, not a species).
- Mining hook→skill relationships from command strings (a declaration types
  a position, never a pattern — intent invariant 1, 0020).
- A self-healing guard or a required self-installation clause (escalation by
  the tool's own determination; the spine rule bound tightest on the tool
  itself).
- Staking SDK 0.1.0 before the tag (interim releases ride 0.0.x; 0.1.0 is
  the launch tag's to claim).

## Consequences

- The `(manifest-authoring-surface)` record resolves and deletes; plan
  derives phase 1 as open entries, hook-first: the JSON manifest adapter's
  read face, the fields-only kind shape, the collection-address kind fact,
  the hook kind with its cited default contract, the mcp-server kind with
  its cited default contract.
- Phase 2 derives behind phase 1: the SDK constructors and their erasure,
  the container-segment emit projection with the canonical manifest write,
  `bundle.rs`'s conversion to general-write instances, guard coverage of
  represented manifests.
- `coverage.unmodeled-surface` retires per manifest as its governing kinds
  land.
- `builtins.md`'s shipped-kinds list gains each registration kind as it
  ships.
