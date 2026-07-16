## Surface

The member-projection-path derivation (kind facts + name → harness-relative path)
now has two homes, in two languages, with no gate holding them in agreement:

- `src/drift.rs:579` `member_projection_path` — the engine's, whose own doc comment
  calls it "the Rust port of the retired SDK `projectionPath`".
- `sdk/src/emit.ts:127` `projectionPath` — re-introduced by EMBEDDED-FORMAT-TARGET-FACTS,
  because an embedded kind's `render` hook is erased at the seam: the engine never sees
  the hook, so the SDK must derive the target's path itself to render it.

The duplication is forced by the spec (`representation.md`, "kind": the target's
projection path is a fact the *format* selects), not by an avoidable choice. What is
avoidable is that nothing catches a drift: the SDK writes the link, the engine reaps and
owns the projection, and no test compares the two derivations. `builtin_lock_frozen`
crosses the seam but carries no projection path.

## Observed at

341717b (HEAD when observed)

## Suggested consolidation

Keep both derivations (no third language boundary to cross), but gate the agreement:
a Rust test that, for each built-in kind, asserts `member_projection_path` equals the
path the SDK renders for the same facts+name — the seam pattern `builtin_lock_frozen`
already establishes. Alternatively, lower the path onto the seam as a per-member column
the SDK reads back, making the engine the single derivation and the SDK a consumer —
but that inverts today's order (the SDK renders before the engine sees a byte).
