<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- From a cross-perspective review of the shipped requirements/satisfies/representation
  chain. File these; the three DECISIONS below are already being put to the human, so
  do NOT act on them — only the build entries here are for you.

- MUST-FIX (high, data-loss): `re-add`/`import` clobber authored representation.
  `drift::re_add` (`src/drift.rs:961-969`) re-projects drifted/added skills+rules through
  `import::import_skill`/`import_rule`, which rebuild `meta.toml` from source
  (`skill.rs:188-189` → `satisfies` empty, `rationale` None) and only emit
  `[representation]` when non-empty (`skill.rs:306`). Representation is surface-only
  authored state with no source side, so a body edit on disk + `re-add` silently WIPES
  authored `satisfies`/`rationale` → coverage flips a covered requirement to false
  UNFILLED. Violates `specs/20-surface.md` "merge rather than clobber" (three-state law).
  FIX: before writing, if the target surface `meta.toml` holds `[representation]`, read it
  (`from_surface_dir` already parses it, `skill.rs:238-242`) and carry `satisfies`/
  `rationale` forward — a small merge helper shared by `import_skill`/`import_rule` (and
  the rule equivalent). Add a round-trip test: author satisfies → drift the body on disk →
  re-add → assert satisfies+rationale survive. Touches `src/import.rs` + `src/skill.rs` +
  `src/rule.rs` + tests.

- HARDEN coverage (low, do in one entry): (a) pin EXACT-STRING match for
  `satisfies`↔`requirement.<name>` (both are literal human-authored TOML keys; no
  case/whitespace fold) and add a mismatch fixture proving a typo yields the paired
  UNFILLED+DANGLING (true positives, not spurious); (b) dedup `satisfies` before the
  dangling loop so `["x","x"]` emits ONE diagnostic (unfilled already uses a set);
  (c) add a doc cross-ref in `src/coverage.rs` noting DANGLING mirrors `graph::check`
  route-resolution and UNFILLED mirrors `graph::degree` min-in-degree over a NON-artifact
  target set, and WHY unifying into `graph.rs` is rejected (avoids a fake `requirement`
  kind in `by_kind`) — so a future third bipartite case finds the seam. Touches
  `src/coverage.rs` + `src/compose.rs` (match rule) + tests. Disjoint from the MUST-FIX
  (different files) → the two may run in parallel.

- DEFER (track as real entries, not code comments): (1) custom-kind (`spec`) gains a
  `[representation]` read + `Features.satisfies` population so temper's own spec corpus
  can participate in coverage — `kind::Unit` currently hardcodes `satisfies=Vec::new()`
  (`src/kind.rs:452-455`). (2) `temper why <artifact>` + `temper requirements` READ verbs
  (forward `satisfies → means`+rationale; reverse `requirement → satisfiers` = blast
  radius) — high payoff, queue AFTER the dogfood below; realizes the traversal
  `00-intent.md` already promises as prose. No engine change, read-only over post-keystone
  data.

- DOGFOOD (after MUST-FIX lands, so re-add won't wipe it): author temper's own
  `[requirement.*]` in the root `temper.toml` + a `satisfies` on an artifact — human
  territory (root `temper.toml`), NOT a build entry; noting it here only so plan sequences
  it after REPRESENTATION-PRESERVE. The friction (harness carries rules, not skills, so the
  `10-contracts.md` worked example can't be literally satisfied) IS the demo.

- DECISIONS being put to the human (do NOT file as build work): (i) `filled_by = {kind|role}`
  typed requirement fillers — revises the role-vs-requirement separation
  (`10-contracts.md:119-122`); (ii) ratify coverage kind-blindness explicitly in the
  `10-contracts.md` Decision (kind-typed fillers go via roles); (iii) project-wide
  stray-key hard-error posture in `compose.rs` table parsing. Await the human's calls.
