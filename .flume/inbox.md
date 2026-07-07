<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

Field report 3 (2026-07-07, same testbed at 18dca38 — custom-kind, edge, and
nested-member probes). Route each:

- T9 custom kinds invisible to the read/gate side: the lock carries the
  `command` kind row and 5 member rows, the projections exist, guard covers
  them — but `check` reports "checked 16 members across 3 built-in kinds"
  (commands uncounted, their floor never runs), `coverage.unmodeled-surface`
  still flags `.claude/commands` even though a locked kind now governs it,
  and `install` skips custom-kind projections when placing managed-by notes
  (4 skills got notes; 5 commands did not). The write side is kind-generic;
  the read/gate side is hardwired to built-ins. One entry: make check,
  coverage, and install iterate the lock's kind rows, not a built-in list.
- T10 mention edges die at the seam: `text` interpolations are
  resolution-checked at emit (a dangling address refuses loudly — verified),
  but sdk/src/declarations.ts compiles no mention family — the edge reaches
  neither the seam payload, the lock, nor the graph. `explain build` says
  "it points at no member"; a `degree` clause over mentions can never bind.
  Law 8's declared-edges vocabulary evaporates one step after authoring.
- T11 the gate and the session-start reporter compute opposite verdicts on
  the same tree: plain `check .` evaluates no requirement clauses (T1);
  `check . --reporter session-start` DOES evaluate them
  (requirement.unfilled / .count / .admissibility all exist) but over a
  fresh one-shot import that drops the `satisfies` joins and the kind model
  — 9 false blocking findings on a green harness (every requirement
  "unfilled"). As wired by install, every new Claude Code session opens with
  a false "contract is failing — get approval" alarm. The evaluator belongs
  in the lock-reading path where the joins live. Same family as T1 /
  REQUIREMENT-GATE — likely one entry or a blockedBy sibling.
- T12 kind discovery ranges into `.temper/` itself: naming the memory
  sidecar CLAUDE.md made the `**/CLAUDE.md` scan count it as a second memory
  member ("memory (2)"); renaming the sidecar was the workaround. Discovery
  should skip the surface workspace unconditionally.
- T13 nested-member pipeline: both ends built, middle missing. Works: SDK
  `genre({name, withinHosts})` → host kind row carries `templates =
  ["directive"]`; engine fence fold (```genre.<kind> <key>` TOML →
  EmbeddedMember with leaves/collections, unit-tested in
  tests/nested_member.rs); leaf-grain read verbs (explain
  address:<member>/<kind>/<key>/<child-path> — grammar, impact, context
  faces). Missing: emit-time fold + lock serialization — graph.rs:1020
  hardcodes `nested_members: Vec::new()`, drift.rs never runs extraction, so
  the lock carries zero nested members and the read verbs always read empty;
  `from_kind_fact_row` round-trips the templates column but has zero
  production callers. One entry: land the fold-at-emit → serialize-to-lock →
  read-from-lock middle. Live consumer ready: the centercode `directive`
  genre is declared and a converted fence is one git revert away. The write
  face (blocks(), rendering nested members into host bodies) stays on the
  (genre-fence-format) fork — reconcile the key against the standing
  genre/embedded-member forks rather than filing a duplicate.
