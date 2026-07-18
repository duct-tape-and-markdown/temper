# 0042 — the entry declares its shape

- **Date:** 2026-07-18 · **Status:** accepted

## Context

The manifest grammar answers one question at every collection: how
does an entry's *value* decompose into a member's fields? Three
answers exist in the documented formats — a plain object (its keys
are the fields), a bare scalar (`enabledPlugins`: the value is the
member's one declared field), and a group-array (`hooks.<Event>`: an
array of matcher groups fanning into one member per handler, the
group's `matcher` lifted onto each; both facts
code.claude.com/docs/en/plugins-reference and /hooks, retrieved
2026-07-16). The engine encodes the second and third as branches on
collection *identity* (`CollectionKeyPath::HooksEvent`,
`::EnabledPlugins`) inside otherwise-generic grammar code, with
`ENABLEMENT_FIELD` an engine constant. Each branch was added inside a
build entry correctly refusing to invent a vocabulary axis mid-entry
(the derived layer never widens the language); the wrong-level residue
surfaced to the level that may. Two shipped instances of one job done
twice is the consolidation trigger the posture pages define — the
second instance is the evidence, and "the fix lands at the mechanism"
names the cure. Session-argued from John's question ("is the fact not
representable, or did we not widen at the correct level?"),
human-ruled ("encode it") 2026-07-18.

## Decision

**A collection's kind data declares its entry shape.** The collection
address gains a closed **entry-shape** enum:

- **object** — the default and the unmarked case today: the entry
  value's keys are the member's fields, verbatim.
- **scalar** — the value is the member's one declared field; the
  field's name is kind data (retiring the `ENABLEMENT_FIELD` engine
  constant into the declaring kind).
- **group-array** — the value is an array of groups, each carrying
  lifted shared fields and a member array; one member per inner
  entry, the lifted fields joined on. The shape's parameters (the
  member-array key, the lifted field names) are kind data, settled at
  derivation from the cited format docs.

**Both faces dispatch on the declared shape, never on the collection
key.** Read decomposition and write recomposition stay inverses per
shape — the byte-faithful round-trip is per-shape, not per-instance.
The known collections re-declare under the axis and stop being
special; the next manifest format with a divergent entry value is a
declaration, not an engine edit. The enum grows only by deliberate
addition, the same bar every closed vocabulary here crosses.

## Rejected

- **Keeping identity-keyed branches**: a shadow model of Claude Code
  accreting inside generic code — the exact megalith-by-a-thousand-
  cited-exceptions failure mode, and a branch on a specific instance
  inside kind-generic code is residue by the altitude posture's own
  words.
- **A general decomposition language** (JSONPath-style transforms
  over entry values): more language than three documented shapes
  need; every operator is surface a contract author can get clever
  with, and the closed enum keeps decomposition decidable and
  enumerable.
- **Widening key syntax in the same stroke** (the
  `<plugin>@<marketplace>` composite-key split, the third
  identity-keyed special case): one instance in the wild — it waits
  for its second under the evidence bar, cited where it stands.

## Consequences

- `kind.rs`: the collection-address data gains the entry-shape enum;
  `ENABLEMENT_FIELD` retires into the declaring kind's data.
- `json_manifest.rs`: `manifest_members` and the write faces re-key
  from `CollectionKeyPath` identity to declared shape; the identity
  branches delete.
- The shipped registration kinds declare their shapes with the format
  cites they already carry; lock/schema rows for collection addresses
  carry the shape and round-trip (derivation verifies both faces).
- SDK: the kind authoring face mirrors the field; the ts-rs seam
  regenerates.
- The gauntlet gains a fixture composing the three shapes in one
  manifest; existing hook/enablement tests re-point at declarations.
- Plan derives from this checklist; no fork record exists to delete
  (the question arose and resolved in session).
