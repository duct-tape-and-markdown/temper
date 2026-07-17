# Plan state

- Spec derived through: 63e1f22
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DERIVE one contained slice of 6d2cca6 (decision 0037, "the
  verifier is typed"). 0037's Consequences names FIVE derivation entries,
  routed as an incremental slice chain, one per tick:
  - **[5] verifier-type resolution at check** → **VERIFIER-TYPED** (filed).
  - **[2] telemetry declaration + hook projection** → **TELEMETRY-HOOK-PROJECTION**
    (filed THIS tick). Emit synthesizes tap `hooks.<Event>` registrations from a
    requirement's telemetry verifier: SDK-side `tapHookRows(harness)` beside
    `registrationRows` (`declarations.ts`) unions the lifecycle events any
    telemetry verifier names into ONE deduped `hook` RegistrationRow per event
    (the tap is dumb — permission-union precedent, `needs.ts`), merged into the
    registrations family; the engine's existing registration→manifest loop
    (`drift.rs`:932, `hook_matcher_group`) projects them into settings.json
    UNCHANGED. Owns the event-NAME→(lifecycle event, matcher) mapping + cites
    (InstructionsLoaded/path_glob_match, Skill→PostToolUse tool_input,
    UserPromptExpansion). No `builtin_lock.toml` touch (no shipped telemetry
    verifier) → disjoint from EXTENT/SETTINGS-LOCAL; blockedBy VERIFIER-TYPED
    (needs the typed verifier + overlaps declarations.ts/emit.test.ts, disjoint
    regions).
  - **[1] tap verb**, **[3] local-locus log kind**, **[4] field strand** →
    un-derived, one slice per tick. [1] is the engine subcommand the tap hook
    invokes, owning the payload cites (session_id, load reason); [3] is the log
    the tap appends to (dial-kind local-locus precedent) — **its storage FORMAT
    is open**: temper's three formats are whole-file document reads, none an
    append log, so [3]'s derive must settle format (reuse json-document vs. a
    new append form) or surface it; [4] is explain's field strand narrating the
    record. [1]/[4] read the log, so both wait on [3]'s format call.
  Because [1],[3],[4] are un-derived, 0037 is NOT fully routed → cursor HOLDS at
  63e1f22 (a big delta takes several ticks, by design).
  - Consequences non-entry bullets, re-checked: `contract.md`/`pipeline.md`/
    `intent.md` body changes are IN the corpus (6d2cca6), not code. The
    `(eval-capability)` fork record — absent from open-questions.md (deleted at
    7d1215a). `docs/horizons.md` gains no entry (human territory). Encode-time
    payload/matcher cites owed — matcher cites routed to [2] (this tick);
    payload cites to [1].
- Queue: 6 entries — EXTENT-PREDICATE **pickable** (gate:open); SETTINGS-LOCAL-KIND
  + VERIFIER-TYPED (both blockedBy EXTENT, mutually disjoint); TELEMETRY-HOOK-PROJECTION
  (blockedBy VERIFIER-TYPED — chain EXTENT→VERIFIER-TYPED→this, disjoint from
  SETTINGS-LOCAL); + 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  DAG-disjoint.

Plan continues: yes — spec delta still live (0037 slices [1],[3],[4] un-derived,
one per tick), and post-ship reconcile of b85df4a..HEAD (3 build commits: 6667265,
631bc83, 8955a17) still pending below it.
