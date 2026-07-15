<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Decision 0024 + the pipeline.md "The lock" amendment (human-ruled 07-15)
  resolve `(lock-upgrade-migration-posture)`: read robustly (join-time
  normalization; bare-label qualification where unambiguous; true collision
  stays malformed-loud), refuse at the cliff (total-reap and dropped-layer
  refusals; explicit teardown flag). The three dependsOnForks entries
  (LOCK-SPELLING-REAP, SATISFIES-LABEL-QUALIFY, EMIT-INTO-REROOT-REAP)
  unblock — rewrite each against the ruled posture, and the cliff refusals
  + teardown flag may want their own entry if none carries them.
  `(member-fence-dead-text)` resolved with it: the prose scan is rejected
  (0024 "Rejected"); the pre-0018 layer loss is the dropped-layer cliff
  refusal's job. observed at d26b3ce
- The pipeline.md "Emit"/"Refusing" amendment (human-ruled 07-15) resolves
  `(composed-mention-discovery-locus)`: the SDK defers a mention whose
  address names a declared kind with no composed member of that name — the
  row rides the lock, `check` owns the verdict; an undeclared kind still
  refuses at emit. Work: sdk/src/emit.ts renderTextBody deferral keyed on
  the program's declared kinds + engine-side mention-edge resolution over
  discovered members (verify graph already covers it); unblocks the
  example's skill→script mention demo. observed at d26b3ce
- `(frontmatterless-managed-by-banner)` ruled 07-15: the managed-by note
  gains a frontmatterless form — a block-level HTML comment for
  markdown-bodied frontmatterless kinds (Claude Code strips block-level
  HTML comments before injection: code.claude.com/docs/en/memory, retrieved
  2026-07-15 — human-visible, model-invisible). `src/install.rs:152`
  placement grows the second form. observed at d26b3ce
- `(custom-kind-consumer-docs)` ruled 07-15: the layout-authoring guidance
  lives in the bundled teaching skill (distribution.md "What ships": it
  teaches mechanics; layout authoring is mechanics). Docs-only entry when
  the bundle work is live; blocks nothing. observed at d26b3ce
