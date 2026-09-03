<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at 6e4d9ca3 (GH #33, human-ruled 09-03) — `rule.mention-reachable.paths`
  false-positives on a strict-subset scope: `src/graph.rs`'s `uncontained` is
  literal glob-string membership (`gate.contains(glob)`), not subset over the
  path sets the globs denote, so `database/x/**/*.sql` vs gate `**/*.sql` is
  reported unreachable. Fix: subset-aware containment over resolved path sets
  via the sanctioned `globset` engine (never hand-rolled), and re-read the
  finding text so it states the direction the check actually holds (citing
  member's scope inside the cited member's gate). Owns 0028.

- observed at 6e4d9ca3 (GH #36, human-ruled 09-03) — `mentionReachable("paths",
  "paths")` is declared only on `ruleDefaultContract` (`sdk/src/builtins.ts`);
  the `skill` default contract carries no equivalent, so a skill consulting a
  narrower-gated member is silent where a rule is flagged. Add the same clause
  to the skill contract. Must be `blockedBy` the #33 fix, or the new clause
  multiplies the false positive.

- observed at 6e4d9ca3 (GH #32, human-ruled 09-03) — `hook` members address as
  `hook:<Event>` alone (fields shape, registers on `event`), so three hooks on
  `SessionStart` collapse to one address and edge resolution through the
  `kind:name` map keeps whichever composed last: silent wrong target. Same
  disease 21f61463 refused for `collectionAddress`. Ruling: a duplicate member
  address within a kind is a declaration-time refusal with a named error, not
  last-writer-wins. Whether `hook` also grows a discriminator in its address is
  a follow-on fork to surface, not to invent here.

- observed at 6e4d9ca3 (GH #34, human-ruled 09-03) — `check --help`
  (`src/main.rs:58`) promises "every spelling of one harness resolves to the
  same verdict", but the session-start reporter counts committed members only
  and the default reporter counts disk (adds `settings-local`). Ruling: the
  verdict is spelling-invariant, the coverage count is reporter-scoped; amend
  the help text and the coverage line to say so. No count split.

- observed at 6e4d9ca3 (GH #30, human-ruled 09-03) — an `at` locus rooted under
  `.temper/` emits and locks but is never discovered: `discoverable_paths`
  (`src/import.rs:439`) fences `WORKSPACE_DIR` for every non-local walk by
  design. Ruling: refuse an `at` locus under the workspace at emit (and at
  `check` on the lock row) with a named error; discovery does NOT descend the
  workspace. The workspace is authoring source, never a projection target.

- observed at 6e4d9ca3 (GH #38 half 1, human-ruled 09-03) — `temper guard`
  (`src/install.rs:732`) binds `file_path` against the lock's `EmitOwnedEntry`
  targets; `.claude/settings.json` is the emit-owned projection of every
  `hook`/`installed-plugin`/`known-marketplace` member but is not in that
  target set (it is spliced, not projected whole), so every spelling of it is
  silent. Ruling: the guard covers `settings.json` as an emit-owned target.
  Verify against `emit`'s codomain (0034) so the set has one home.

- observed at 6e4d9ca3 (GH #38 half 2, human-ruled 09-03) — the guard matcher
  is `Write|Edit|MultiEdit`; a Bash/PowerShell-mediated write reaches no guard
  and a Bash payload carries no `file_path`. Ruling: (a) the guard's docs and
  the install narration state plainly that it binds tool-mediated writes only,
  never a boundary; (b) a separate entry for a `PostToolUse` arm on `Bash` that
  runs drift detection after the call (check-style, against emit-owned targets)
  rather than parsing shell text before it. (a) ships this round; (b) is
  scoped as its own entry and may park if the PostToolUse hook contract needs
  a cited external fact first (code.claude.com/docs/en/hooks).

- observed at 6e4d9ca3 (GH #37, human-ruled 09-03) — the managed-projection
  banner is placed by `install` (`src/install.rs:1697`), not `emit`, so a
  member added by a flow that runs `emit` alone ships bannerless and only an
  advisory notices. Ruling: `emit` places the banner on the targets it owns
  (it owns the bytes; the banner is part of the projection contract); `install`
  keeps converging existing files. Check that `drift`'s byte comparison treats
  the banner as projection bytes so `emit` reports unchanged honestly.

- observed at 6e4d9ca3 (GH #28, human-ruled 09-03) — a requirement declared
  `kind: skill` is filled by an `installed-plugin` satisfier with no finding;
  `sdk/src/contract.ts:344` documents `kind` as "constrains what may fill it".
  Ruling: enforce it — a satisfier of another kind is a required-severity
  finding on the satisfier, and `explain` does not list it as filling. Owns
  contract.md "requirement — a shipped kind, not a primitive"; the roster
  already refuses an unmodeled kind (`src/roster.rs:123`), so the seam exists.

- observed at 6e4d9ca3 (GH #31, human-ruled 09-03) — `EdgeTargetFacts.path`
  (`sdk/src/kind.ts:417`) is relative to the host's projection directory; a
  skill body is read from repo root, so `../../rules/x.md` dangles at runtime.
  The spec already names a fifth fact as the open question. Ruling: add a
  repo-rooted path fact (the lock's `source_path` is the value) alongside
  `path`, and document which reader each serves. Existing `path` semantics are
  unchanged.

- observed at 6e4d9ca3 (GH #29, human-ruled 09-03, PARK) — no locus shape
  expresses "committed, not an emit target, still a roster member"; every
  shape tried (`at`, `commitment: "local"`, `fields`, `embedded`) fails one of
  the three. Proposed: a `commitment: "external"` class on the `at` locus —
  committed, never written by `emit`, takes a lock row, addressable as an
  edge target, drift = the file's bytes vs the lock's hash. This needs a
  Decision (0032 ruled `local` as a locus, this is its sibling); park as an
  open question `(external-commitment)` for the interactive session to draft.
  No entry until the Decision lands.

- observed at 6e4d9ca3 (GH #26, #27) — both already shipped on main (21f61463,
  09a79fef). No entry; they close at the 0.0.16 cut. Noted so plan's cite of
  the batch is complete.
