<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human, design session): the join/boundary Decision revision landed
  in specs — `45-governance.md` "coupling is a join — one-way edges exist only at
  the governance boundary" (rewrites d53e207's Decision) + "the world is a node —
  reachability is a predicate"; `15-kinds.md` "Activation — a kind's inherent
  world-edges" (closed vocab: always / description-trigger / paths-match / event,
  cited); `20-surface.md` retires the `[edge.<target>]` clause family (coupling
  is a join riding requirement/satisfies; the example is reworked);
  `05-model.md` gains the world row. Engine consequences to file, in order:
  (1) ACTIVATION-KEY-PARSE — `CustomKind::from_header` accepts an `activation`
  key inert (same red-interim shape as FORMAT-KEY-PARSE: the embedded curated
  KIND.md files gain activation lines only AFTER the parser accepts them;
  unknown keys are load errors, kind.rs:775). Human adds the curated
  kinds/skill + kinds/rule activation lines when it drains.
  (2) EDGE-CLAUSE-RETIRE — remove `[edge.*]` surface support (document.rs:328-331
  emit path + fixtures/tests at document.rs:622,687,760); member-to-member
  coupling rides requirement/satisfies only, per the revised Decision.
  (3) REACHABILITY — the world node in graph.rs + the reachable predicate over
  declared activation (blank description-trigger field; zero-match paths globs),
  spec'd in 45-governance "the world is a node". Depends on (1) + curated lines.
