<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- TWO FORKS RULED (John, 07-15, in session; records deleted from
  open-questions per the anti-accumulation rule; spec homes authored in the
  same commit — observed at 52631eb):
  1. `(prose-interleaving)` → ACCEPTED as pure SDK surface: `blocks()` (or a
     sibling constructor) accepts ordered `Text | EmbeddedMemberValue`
     children — the write-side mirror of layout's ordered regions. Spec home:
     `specs/model/pipeline.md`, "The SDK" (new sentence). Rejected
     alternative: keeping `Prose` single-constructor and wrapper members for
     narrative — minting members whose job is to not be members pollutes
     nested-member rows and the addressability space. Follow-through once
     shipped: examples/base-harness drops its `passage` kind for the native
     interleave (the example's projected bytes should not change).
  2. `(discovery-nested-root-fence)` → ACCEPTED: discovery stops at a
     directory carrying its own `.temper/lock.toml`; a nested governed root
     is its own corpus, never the parent's members. Spec home:
     `specs/model/pipeline.md`, "Install" (amended sentence). Rejected
     alternative: unfenced walk with ignore-rule escape hatches — the outer
     gate governing bytes it does not own is cross-contamination, and
     vendored-harness inspection stays available as an explicit verb aimed
     at the nested root (`(vet)`). Acceptance evidence: the repo's own gate
     stops counting examples/base-harness/CLAUDE.md as a second memory
     member.
