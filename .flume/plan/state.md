# Plan state

- Spec derived through: cc5a9b3
- Audited through: cac023a
- Residue swept through: cac023a
- This tick: SPEC DELTA — routed cc5a9b3 (0025's supporting-docs bullet
  amended: the skill built-in owns `supporting-doc`, kind-side, on the
  requirement precedent). The amendment lands on one queued entry, so
  SKILL-NESTED-REFERENCE-DOCS got the full rewrite a stale entry earns — and
  reading it against disk split it in two. Filed TEMPLATE-FILE-CHILD-FACT
  (`open`): the template's declared fact grows kind-side — child kind plus,
  for file children, the path pattern relative to the parent's unit — with an
  admission over the host overriding the child kind. That half is exactly what
  representation.md states and is derivable today. The built-in adoption is
  NOT: `supporting-doc` needs a file-locus nested child, and three disk facts
  say nothing spells one — the SDK's nesting surface is embedded-only
  (`EmbeddedMemberValue`/`NestedMemberRow`), `Locus` is binary
  (`sdk/src/kind.ts:57`) with no "path from my host's template" spelling, and
  a self-governing `supporting-doc` glob would overlap `skill`'s `*/SKILL.md`
  under a corpus that requires position to decide a document's kind. Filed
  `(nested-file-child)` rather than hand build the collision that already
  bailed it once. Rider re-routing followed the split: the `sdk/src/kind.ts`
  narration riders move to TEMPLATE-FILE-CHILD-FACT, and the two
  `packages/…PACKAGE.md` cites in builtins.ts lose their carrier — no queued
  entry opens builtins.ts now. Closing checklist run: the queue is disjoint
  (one serialization chain plus the park), and every gate reason was re-tested
  on disk — PACKAGING-CHANNELS-REMAINDER holds on all three park conditions at
  HEAD (`git tag -l 'v*'` empty, crate 0.1.0 vs npm 0.0.7, release.yml:7-9
  still defers darwin).
- Queue: 1 pickable (TEMPLATE-FILE-CHILD-FACT, `open`);
  EMBEDDED-FORMAT-TARGET-FACTS → FORMAT-OMITS-EDGE-CLAUSE serialize behind it
  (shared `sdk/src/kind.ts`, `sdk/test/emit.test.ts`);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag). Fork board: `(nested-file-child)` new and blocking the
  supporting-doc adoption; `(guidance-climb)` blocks nothing.

Plan continues: no — the spec delta was this tick's one job and cc5a9b3 is
routed (the amended bullet resolves to TEMPLATE-FILE-CHILD-FACT plus a
registered fork). No input below it is live: the audit/sweep window past
cac023a holds no `src/`/`tests/`/`sdk/` commit. **Build takes over:
TEMPLATE-FILE-CHILD-FACT is pickable.**
