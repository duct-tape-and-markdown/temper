<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Law 8 landed in specs/00-intent.md ("the model is declared, never mined") with
  a corpus-wide rectification. Two engine consequences to file:
  1. RETIRE the `references` extraction primitive (specs/15-kinds.md Decision
     "no body-mined references"): remove `Primitive::References` +
     `strip_suffix` + `backtick_filename_refs` + `is_filename_reference` and
     their tests from src/kind.rs; the KIND.md loader then rejects `references`
     as an unknown primitive, which is correct. The dogfood
     `.temper/kinds/spec/KIND.md` no longer declares it (cleaned by hand).
  2. MEMBER-PUBLISHED REQUIREMENTS (specs/10-contracts.md Decision "a
     requirement's publisher is any authored surface document";
     specs/20-surface.md header clause list; specs/90-spec-system.md "the
     corpus is classed"): a member header may carry `[requirement.<name>]`
     tables — same shape as the assembly roster, one namespace, collisions are
     admissibility findings — joined by the existing `satisfies` fill and the
     existing coverage gate.
  Also update the `(reference-id-normalization)` breadcrumb in open-questions:
  SUPERSEDED by the retirement Decision (the shipped strip_suffix machinery is
  being removed, not wired into the dogfood). The specs/ class migration
  (intent/architecture/process placement reshuffle + header authoring) is a
  HUMAN ceremony after the engine ships — never a build entry.

- DEFECT (dogfood-found): `re-add` skips custom-kind members — hand edits to
  `specs/*.md` never reconcile back into `.temper/specs/`, so `check` runs over
  stale spec bodies (observed: law 8 edits invisible to the gate until a manual
  re-import). The drift model (`specs/20-surface.md`) owns all three directions
  for every kind, built-in and custom alike; `re-add` (and `diff`) must cover
  kinds registered in the assembly, scanning each `governs` locus.
