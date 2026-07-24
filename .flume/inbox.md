<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->



## Embedded-kind guidance unsurfaced (filed 2026-07-23) — 0045 gap

12. **A kind's `guidance` (decision 0045) is authored but never reaches the
    author for embedded-locus kinds.** 0045 decoupled guidance from the clause:
    it attaches to a clause, a **kind**, or a field, and on a kind it is pure
    authoring counsel delivered via `schema` (hover) and `explain` — for *any*
    kind, no embedded carve-out. But the SDK re-coupled kind-guidance to the
    **locus-bearing kind-fact row**: `sdk/src/declarations.ts` `kindFactRow`
    throws for `locus.kind === "embedded"` (~295, "an embedded kind carries no
    locus-bearing kind fact") yet carries `guidance` on that same row (~314), so
    a locus-less embedded kind's guidance never reaches the lock. `schema`
    iterates non-embedded kinds only (`atLocusKindsInPlay`, ~353); `explain`
    (`read.rs`) has no bare-kind form; the engine's kind-guidance rides
    `KindFactRow` (`drift.rs` ~3246), the row embedded kinds never get.
    Dogfood-verified three ways (zero lock diff after nine guidance strings;
    schema omits embedded; explain resolves only members/requirements/leaves).
    *observed at v0.0.12 (bf4b5cd9); findings in testbed centercode pr-571 @
    efcc1fd175. Decision 0045.*

    **Ruling (interactive, 2026-07-23): surface it — real gap, aligned with
    0045, not an intended exclusion.** It is the same coupling mistake 0045
    fixed, one layer down: 0045 decoupled guidance from the *clause*; the SDK
    left it coupled to the *locus row*, so a locus-less kind loses the channel
    0045 gave every kind. An embedded kind is still an authored kind with real
    authoring concerns, and centercode is a live driver (posture-vocabulary
    counsel moved into guidance on embedded kinds).
    - **Fix (spans SDK + engine):** guidance delivery decouples from the
      locus-bearing row — an embedded kind's `guidance` reaches the lock by a
      locus-independent path; `schema` includes embedded kinds; `explain` gains
      a bare-kind form (counsel with no member instance).
    - **Design sub-decision for the entry:** the lock-delivery *shape* for
      locus-less guidance — a guidance row keyed by kind name that any kind
      carries, vs. riding the host's `templates` row through which embedded
      kinds already reach the lock. Settle against a short spec note.
    - **Spec:** state in the model (`contract.md`, the 0045 guidance channel)
      that kind guidance is delivered for *all* kinds including embedded
      (decoupled from locus, as 0045 decoupled from clause). If the evergreen
      contract.md doesn't already carry this, the one-clause note is interactive
      pre-build work (cf. #10's `pipeline.md` clause) — plan: surface if the
      cite isn't there rather than routing build-ready without it.
