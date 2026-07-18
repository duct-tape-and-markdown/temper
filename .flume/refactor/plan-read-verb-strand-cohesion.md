## Surface
`read.rs` (1844 lines) dispatches five largely independent narration
strands off `explain()`'s `Species` match, each with its own private
helpers and `#[cfg(test)]` proofs: `why`/`why_one` (320-529),
`impact`/`impact_leaf`/`impact_one` (593-1237),
`context`/`context_leaf`/`context_member` (856-1110),
`requirements`/`roster_overview`/`requirement_detail` (1249-1435), and
`field` (1464-1537, the telemetry strand — shares graph/leaf-address
machinery with none of the other four). `architecture.md`'s Growth rules
anticipate exactly this shape of split ("Splits land flat: a cohesion
finding extracts a new module into its host's subsystem — the codemap
gains a name, the tree gains a file, no directories"), but the module's own
doc frames itself as "one CLI verb, four traversals + a telemetry strand" —
a stated design stance, not an accident.

## Observed at
7ac498a

## Suggested consolidation
A human call on whether `read.rs`'s five strands (particularly `field`,
which shares no machinery with the other four) want a flat split per the
codemap's own growth rule, or whether "one CLI verb" is the right unit and
the file's size is an accepted cost of that framing — not a mechanical
extraction plan can propose unilaterally.
