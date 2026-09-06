## Symptom

Routing 8 inbox notes this tick required rewriting/filing ~15 pending
entries, most sharing hot files (`tests/layout_kind.rs`, `src/read.rs`,
`src/admissibility.rs`, `sdk/src/kind.ts`, `sdk/test/builtins.test.ts`).
`pending-entry.md`'s "Disjoint, or serialized" rule requires two `open`
entries never share an edited path unserialized — I chained every
collision among entries I actively touched this tick, but two pre-existing,
untouched `open` entries still collide with the now-serialized clusters and
were left as-is (out of this tick's scope, since the rule binds "filing or
rewriting an entry"):

- `CHECK-UNDECLARED-LAYOUT-MEMBER-SILENT` and
  `LAYOUT-PROSE-REGION-UNREACHABLE-LOCK-EXPLAIN` both touch
  `tests/layout_kind.rs` (now chained: LOUD-READ-SWALLOWED →
  DUPLICATE-PROSE → OWN-SPAN-LEAF) and, for the latter, `src/read.rs`
  (now chained: QUALIFIED-ADDRESS-FORM → ADOPTER-ENTRY-POINT).
  `GRAPH-EDGE-TARGET-HOST-QUALIFIED-ADDRESS` also touches `src/read.rs`
  and `sdk/src/kind.ts`, bridging into the SDK cluster below.
- `EMBEDDED-MEMBER-VALUE-TYPED-LEAVES` touches `sdk/src/kind.ts` and
  `sdk/test/builtins.test.ts`, both now chained behind
  `KIND-REGISTRATION-EMPTY-EMBEDDED-TYPE`.

Also unresolved and out of scope: the existing `GUARD-SETTINGS-EMIT-OWNED-
REGISTRATION-WALK` (open) collides with the existing
`GUARD-PATH-MATCH-ABSOLUTE-SINGLE-SEGMENT` → `GUARD-MANIFEST-WRITE-DROPPED-
MEMBER-CONFORMS` chain on `tests/install.rs`, predating this tick.

## Cost this tick

No reverted commits — caught by review before shipping, not by a gate. But
the derivation cost was high: verifying and chaining ~15 entries' file
lists by hand this tick (several agent-dispatches) to avoid introducing
*new* unserialized collisions, while consciously drawing a line at
pre-existing ones outside today's touched set. That line is defensible for
one tick but leaves a real merge-conflict risk live for whichever wave
picks `CHECK-UNDECLARED-LAYOUT-MEMBER-SILENT`,
`LAYOUT-PROSE-REGION-UNREACHABLE-LOCK-EXPLAIN`,
`GRAPH-EDGE-TARGET-HOST-QUALIFIED-ADDRESS`, or
`EMBEDDED-MEMBER-VALUE-TYPED-LEAVES` alongside their now-chained
collision partners.

## Suggested fix

A dedicated plan tick (or a `posture-sweep`-style rotation) that audits
`pending.json`'s full `files[]` graph for shared-path collisions across
*every* `open` entry (not just ones a given tick touches) and serializes
or documents each. Possibly worth a lightweight afterCommit gate check
(`pending-gate`?) that flags two simultaneously-`open` entries sharing a
declared path, so this stops being a manual audit each time a cluster of
entries gets touched.
