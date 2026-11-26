# Parked: the locus grammar cannot say "no CLAUDE.md at or above"

What I checked:

- `specs/model/representation.md` "locus": a file template is literal
  segments with at most one `*`, optionally under `**/`, minus declared
  exclusions. It also says "A document's kind is declared by its position
  alone". Whether an `AGENTS.md` is memory depends on whether *another*
  document (a `CLAUDE.md` beside it or in an ancestor) exists. That is a
  claim conditioned on the tree's contents, not on the document's position.
  The model rules it out, so the gap is in the spec, not just in the
  grammar.
- `sdk/src/kind.ts` `Locus`: one `glob` per `at` kind, and `exclude` takes
  static literals only. `memory` already spends its glob on
  `**/CLAUDE.md`.
- `src/kind.rs` `bare_root_file` (row-less, used by `skill`) is
  unconditional. Using it for a root `AGENTS.md` breaks the pin "an agents
  md beside a claude md is not a memory member". It also misses nested
  `AGENTS.md` files with no `CLAUDE.md` above them.
- `specs/builtins.md` "The shipped kinds" calls memory "a `CLAUDE.md`-family
  file" and never names `AGENTS.md`. Decision 0101 (read as the runtime
  reads) motivates the entry but does not widen the locus model.

Open question for a human: does a kind's claim get a conditional term
(present only when no sibling or ancestor `CLAUDE.md` exists)? That would
amend the "position alone" invariant. Emit also needs a ruling: when an
author adds a `CLAUDE.md`, the `AGENTS.md` member silently stops being a
member. The other route keeps `AGENTS.md` unclaimed and governed only
through an `@AGENTS.md` import edge, which already holds. Either answer is
consistent with intent, so this needs a decision before an entry can ship.

The stale cite in `src/builtin_lock.toml:50` ("Claude Code does not read
`AGENTS.md`", retrieved 2026-07-15) is still wrong under the 2026-11-26
memory docs (v2.1.277+). It can be fixed by itself whichever way this is
decided.
