# Plan state

- Spec derived through: 63e1f22
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DERIVE one contained slice of 6d2cca6 (decision 0037, "the
  verifier is typed"). 0037 is a multi-part capability; its Consequences names
  FIVE derivation entries — routed as an incremental slice chain, one per tick:
  - **[5] verifier-type resolution at check** → **VERIFIER-TYPED** (filed this
    tick). The requirement's single `verified_by` path becomes a typed species
    union: `script(path)` (today's path resolution, retyped) + `telemetry(events)`
    (named documented harness events), each resolved per-species at admissibility.
    Two-sided (`sdk.md` seam): SDK constructors (`contract.ts`) + lowering
    (`declarations.ts`) + generated row; Rust `enum Verifier` (`compose.rs`),
    wire row (`drift.rs`), reconstruction (`main.rs`), resolution dispatch
    (`roster.rs`). Probe NOT typed — 0037 keeps it a documented pattern until
    its transcript surface is documented or a second consumer types it.
    blockedBy EXTENT-PREDICATE (shared contract.ts/declarations.ts/drift.rs,
    disjoint regions).
  - **[1] tap verb**, **[2] telemetry declaration + hook projection**,
    **[3] local-locus log kind**, **[4] field strand** → un-derived, one slice
    per tick. [2] projects a telemetry verifier as tap hook rows and owns the
    matcher cites (path_glob_match, Skill tool_input, UserPromptExpansion); [1]
    is the engine subcommand the tap invokes, owning the payload cites
    (session_id, load reason); [3] is the log the tap appends to (dial-kind
    local-locus precedent); [4] is explain's field strand narrating the record.
  Because [1-4] are un-derived, 0037 is NOT fully routed → cursor HOLDS at
  63e1f22 (a big delta takes several ticks, by design).
  - Consequences non-entry bullets, checked: `contract.md`/`pipeline.md`/
    `intent.md` body changes are IN the corpus (6d2cca6), not code. The
    `(eval-capability)` fork record — verified ALREADY absent from
    open-questions.md (deleted at 7d1215a). `docs/horizons.md` gains no entry
    (human territory; not plan's read path). Encode-time payload cites owed —
    routed to slices [1]/[2] where the consuming code lands.
  Disjointness: VERIFIER-TYPED overlaps EXTENT on contract.ts, declarations.ts,
  drift.rs → serialized `blockedBy: EXTENT-PREDICATE`. Disjoint from
  SETTINGS-LOCAL-KIND (no shared path) — both may run once EXTENT ships. No
  overlap with the two parked entries.
- Queue: 5 entries — EXTENT-PREDICATE **pickable** (gate:open); SETTINGS-LOCAL-KIND
  + VERIFIER-TYPED (both blockedBy EXTENT, mutually disjoint) + 2 parked
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). DAG-disjoint.

Plan continues: yes — spec delta still live (0037 slices [1-4] un-derived, one
per tick), and post-ship reconcile of b85df4a..HEAD (3 build commits: 6667265,
631bc83, 8955a17) still pending below it.
