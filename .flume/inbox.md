<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): REGISTERED-KIND-SHADOWS-EMBEDDED (blocks the memory-kind
  file commit; found on the second placement attempt — 2 red tests remain).
  The Decision now rules it (specs/architecture/15-kinds.md, "an explicit
  registration owns its bare name outright"): a kind the assembly registers
  shadows any unbound embedded carrier of the same bare name; embedded kinds
  collide among themselves only over references no registration claims.
  Symptom A: import::tests::an_any_depth_glob_discovers_a_nested_hierarchy_
  with_placement_folded_ids authors a project kind bare-named `memory`; with
  claude-code.memory + agents-md.memory co-embedded its members silently stop
  importing (doc["memory"] absent, src/import.rs:1443) — a registered kind
  must never be preempted by unbound built-ins, and a silent skip is the worst
  outcome (fail-loud). Symptom B: import::tests::builtin_scan_is_generic_over_
  the_embedded_kind_set (src/import.rs:1272) asserts every embedded kind
  writes a lock section, but a harness with no CLAUDE.md/AGENTS.md writes no
  `memory` section — re-pin to assert sections exactly for kinds with
  discovered members (a memberless kind writing an empty section is noise).
  Also in scope: the roll-up/lock section key when two co-embedded kinds share
  a bare name and BOTH discover members in one harness — mirror resolution
  (bare while unique, qualified where two carriers meet); never clobber one
  kind's rows with another's. Repro for both: place any second bare-`memory`
  KIND.md under kinds/<provider>/ and run cargo test. After it drains, the
  human commits the four held curated files.
