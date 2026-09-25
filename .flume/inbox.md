<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->









- observed at 0b20cadf (issue audit) — GH #59 is open and has no fork: no
  predicate can relate a property of one edge's source to a property of
  another edge's source when both edges meet at one member. The shape is
  `writer --writes--> target <--presents-- presenter --gatedBy--> condition`,
  and the defect is a gated presenter whose target has an ungated writer.
  `degree` counts edges and `membership` checks one field against a fixed
  set; `reachedFrom` (0.0.19) follows reachability, not a correlation. This
  is a contract-language addition with no spec section behind it, so it
  needs a fork, not an entry. The objection the fork must answer: a general
  path predicate is a query language, and the kernel should grow one only
  when a narrower shape (for example "every sibling source into X over
  field F carries the edge G that the source over field H carries") cannot
  cover the demand.
- observed at 0b20cadf (issue audit) — GH #35 is open and has no fork.
  Declined 09-03 as a bug, because a `/name` token in prose cannot be told
  apart from a quoted user command or an external plugin's command. What
  was left open is a feature: an opt-in typed edge from a skill or
  supporting doc to a `command` member, which the gate resolves like any
  edge. Unruled: what the edge is called, and whether it may target a
  command the harness does not own (a plugin's command). That second part
  touches `(external-commitment)` (GH #29).
- observed at 0b20cadf (issue audit) — GH #44 may still reproduce. 197eb703
  cites it, but that commit only detects a *shifted* heading
  (`SwallowedHeading`). The issue's core claim is untested: `required(slot)`
  on a layout `field` region is accepted at admissibility and never fires,
  including when the trailing slot's heading is absent and nothing shifts.
  No test pairs `required` with a layout slot. Reproduce on HEAD first. If
  it still holds, the choice is between refusing `required` on a layout
  slot at admissibility (as `sectionContains` on an embedded kind already
  is) and giving slot presence a meaning under positional binding.
- observed at 8676e53b (issue audit, measured on Claude Code 2.1.281) — the
  guard's `warn` mode reaches no one. `main.rs` Guard prints the finding with
  `eprintln!` and exits 0 on `Warn`, but a hook's stderr on exit 0 "goes to
  the debug log only, never the transcript, and Claude never sees it"
  (code.claude.com/docs/en/hooks, "Exit code 0", retrieved 2026-09-24). A
  probe confirmed it: a PreToolUse hook writing a token to stderr and exiting
  0 left the token in the debug log and out of the model's context.
  `distribution.md` "Per tool call" says `warn` "surfaces the finding in-band,
  into the live context", and warn is the default, so by default the guard
  does nothing visible. The spec is right and the code is wrong: no fork. In
  band for PreToolUse is `hookSpecificOutput.additionalContext` with
  `hookEventName: "PreToolUse"`. This shares its fix with 0060's PostToolUse
  edge (the answer in the firing event's own shape), so derive the two
  together. The manifest-findings branch has the same `Warn` arm.
- observed at 227b3bbb (adopter harness audit, item 2; confirmed by reading
  the code, not run) — a tap record appended from a linked worktree keeps an
  absolute identity. `log_path` sends the record to the primary checkout's
  log (`src/tap.rs` ~173-222), and `append` (~:288) relativizes identity
  against `log_path(..).parent().parent()`, the primary root. A rule path
  inside a worktree is not under that root, so `strip_prefix` fails and each
  worktree gets its own identity for one rule, which breaks the read-time
  join to lock members. `pipeline.md` already says the log homes in the
  primary so worktree records collapse, so this is a defect, not a fork.
  Relativize against the root the tap ran from (the parent of
  `workspace_dir`). Add a test that appends from a real linked worktree
  (`tests/tap.rs` ~162-193 covers only the primary). 0062's new
  trigger/parent paths take the same relativization.
- observed at 227b3bbb (adopter harness audit, item 4; reproduced in this
  repo the same day) — emitted hook commands run whatever `temper` is on
  PATH. The adopter's hooks ran a cargo-installed 0.0.15 against a 0.0.18
  pin, so all 834 tap records were written at version 1 and nothing flagged
  it. This repo ran 0.0.18 from `~/.cargo/bin` against the published
  0.0.20. Resolving the pinned binary from `node_modules` would break the
  stranger gate (distribution.md: the binary alone, no Node), so the lever
  is `check` reporting when the running engine differs from the one the
  program pins. This is the adopter-side driver for
  `(build-version-identity)`: fold it in there rather than filing a sibling.
- observed at 227b3bbb (adopter harness audit, item 5 and authoring
  patterns) — fork candidates, no entry. (i) A required
  `degree({incoming: {min: 1}})` passed for a standard because one
  path-scoped rule pointed at it, yet the standard's content never reached
  readers of that surface. Candidate clause: a part tagged with a surface
  glob must be carried by some member whose channel fires on that glob.
  It is decidable and sits beside `reachedFrom` and
  `mention-reachable.paths`. After 0061 a path channel fires on file tools
  only, and the clause must say so. (ii) Facts vs norms: the kinds have
  nowhere for task-independent facts that readers, reviewers and modellers
  need, so the facts land in skills that are triggered by writing. The
  adopter proposes a fact part (a statement, what a reader must not
  conclude, and the surfaces it applies to) carried by a rule covering
  those surfaces. (iii) Skill descriptions phrased only for writers, and
  pointers rendered as bare subject labels. These are taste, so they
  belong in default-contract guidance, never a decidable check.
