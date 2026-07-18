<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- RULED (session, standing delegation; the 0042 frame applied at subsystem level — provider
  facts embedded outside the provider face, three instances):
  TAP-PAYLOAD-SCHEMA-SPLIT: tap.rs (foundation) hardcodes Claude Code's hook-payload schema
  ("PostToolUse", tool_name == "Skill", tool_input.skill) — the payload→TapEvent mapping is
  provider knowledge and moves to the provider face, cited; the record vocabulary, version,
  and IO stay foundation. Same class as extract's restore; the foundation invariant then
  holds for tap on its own terms.
  COVERAGE-KNOWN-SURFACES-RELOCATE: coverage_note.rs (judges) compiles KNOWN_SURFACES — a
  cited registry of Claude Code surface facts — plus a hardcoded .claude scan root. The map
  says the provider face IS "the shipped kinds and their cited format facts"; the registry
  and the surface root move there, coverage_note consumes them as loaded data.
  GUARD-DECLARED-LOCUS-FILTER: install's GUARD_PATH_MATCH regex admits only .claude/ paths,
  while emit-owned targets are lock-declared arbitrary paths — a declared-locus kind
  governing outside .claude (0038 layout kinds) escapes the guard silently. The filter
  derives from the lock's declared targets; the .claude binding survives only as the
  no-lock fallback it already is, cited as the built-in default's locus family.
  observed at ab01fb4
