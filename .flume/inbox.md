<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->









- observed at 0b20cadf (issue audit) — GH #59 is open and has no fork: no
  predicate can relate a property of one edge's source to a property of
  another edge's source when both edges meet at one member. The shape is
  `writer --writes--> target <--presents-- presenter --gatedBy--> condition`,
  and the defect is a gated presenter whose target has an ungated writer.
  `degree` counts edges and `membership` checks one field against a fixed
  set; `reachedFrom` (0.0.19) follows reachability, not a correlation. This
  is a contract-language addition with no spec section behind it, so it
  needs a fork, not an entry. The objection the fork must answer: a general
  path predicate is a query language, and the kernel should grow one only
  when a narrower shape (for example "every sibling source into X over
  field F carries the edge G that the source over field H carries") cannot
  cover the demand.
- observed at 0b20cadf (issue audit) — GH #35 is open and has no fork.
  Declined 09-03 as a bug, because a `/name` token in prose cannot be told
  apart from a quoted user command or an external plugin's command. What
  was left open is a feature: an opt-in typed edge from a skill or
  supporting doc to a `command` member, which the gate resolves like any
  edge. Unruled: what the edge is called, and whether it may target a
  command the harness does not own (a plugin's command). That second part
  touches `(external-commitment)` (GH #29).
- observed at 0b20cadf (issue audit) — GH #44 may still reproduce. 197eb703
  cites it, but that commit only detects a *shifted* heading
  (`SwallowedHeading`). The issue's core claim is untested: `required(slot)`
  on a layout `field` region is accepted at admissibility and never fires,
  including when the trailing slot's heading is absent and nothing shifts.
  No test pairs `required` with a layout slot. Reproduce on HEAD first. If
  it still holds, the choice is between refusing `required` on a layout
  slot at admissibility (as `sectionContains` on an embedded kind already
  is) and giving slot presence a meaning under positional binding.
